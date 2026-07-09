//! Segmented parallel download: pre-allocated file, fixed-size chunks with
//! seek writes, per-chunk retry/mirror rotation, resume via the sidecar.

use std::{
    collections::BTreeMap,
    path::Path,
    sync::{atomic::AtomicU64, Arc},
    time::Duration,
};

use reqwest::{header, StatusCode};
use tokio::{
    fs::{self, OpenOptions},
    io::{AsyncSeekExt, AsyncWriteExt},
    sync::{Mutex, Semaphore},
    task::JoinSet,
};
use tokio_util::sync::CancellationToken;

use super::{apply_headers, stream_response, DownloadEngine, ProgressCallback};
use crate::{
    model::Validator,
    resume::{self, ChunkState, ResumeMetadata},
    DownloadError, DownloadResult, ResourceSpec,
};

impl DownloadEngine {
    pub(crate) async fn download_segmented(
        &self,
        spec: ResourceSpec,
        validator: Validator,
        cancel: CancellationToken,
        progress: ProgressCallback,
    ) -> Result<DownloadResult, DownloadError> {
        let metadata_path = resume::resume_path(&spec.target);
        let mut metadata = resume::load(&metadata_path)
            .await
            .unwrap_or_else(|| new_metadata(&validator, self.options.chunk_size));
        if metadata.validator != validator {
            // server content changed since the partial download — start over
            fs::remove_file(&spec.target).await.ok();
            fs::remove_file(&metadata_path).await.ok();
            metadata = new_metadata(&validator, self.options.chunk_size);
        }
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(false)
            .open(&spec.target)
            .await?;
        file.set_len(validator.total).await?;
        drop(file);
        resume::persist(&metadata_path, &metadata).await?;

        let initial = metadata
            .chunks
            .iter()
            .filter(|value| value.complete)
            .map(|value| value.end - value.start + 1)
            .sum();
        let counter = Arc::new(AtomicU64::new(initial));
        let pending = metadata
            .chunks
            .iter()
            .enumerate()
            .filter(|(_, value)| !value.complete)
            .map(|(index, value)| (index, value.clone()))
            .collect::<Vec<_>>();
        let state = Arc::new(Mutex::new(metadata));
        let local = Arc::new(Semaphore::new(self.options.segment_concurrency.max(1)));
        let total = validator.total;
        let mut tasks = JoinSet::new();
        for (index, chunk) in pending {
            let engine = self.clone();
            let spec = spec.clone();
            let metadata_path = metadata_path.clone();
            let cancel = cancel.clone();
            let counter = counter.clone();
            let state = state.clone();
            let local = local.clone();
            let progress = progress.clone();
            tasks.spawn(async move {
                let _local = local.acquire().await.expect("semaphore open");
                engine
                    .download_chunk(&spec, chunk, total, cancel, counter, progress)
                    .await?;
                let mut value = state.lock().await;
                value.chunks[index].complete = true;
                resume::persist(&metadata_path, &value).await?;
                Ok::<_, DownloadError>(())
            });
        }
        while let Some(result) = tasks.join_next().await {
            result.map_err(|error| DownloadError::Resume(error.to_string()))??;
        }
        if cancel.is_cancelled() {
            return Err(DownloadError::Canceled);
        }
        fs::remove_file(&metadata_path).await.ok();
        Ok(DownloadResult {
            path: spec.target,
            bytes: total,
            etag: validator.etag,
            last_modified: validator.last_modified,
        })
    }

    async fn download_chunk(
        &self,
        spec: &ResourceSpec,
        chunk: ChunkState,
        total: u64,
        cancel: CancellationToken,
        counter: Arc<AtomicU64>,
        progress: ProgressCallback,
    ) -> Result<(), DownloadError> {
        let mut last_error = String::new();
        for attempt in 0..=self.options.retries {
            for url in &spec.urls {
                if cancel.is_cancelled() {
                    return Err(DownloadError::Canceled);
                }
                match self
                    .try_chunk(
                        url,
                        &spec.headers,
                        &spec.target,
                        &chunk,
                        total,
                        &cancel,
                        &counter,
                        &progress,
                    )
                    .await
                {
                    Ok(()) => return Ok(()),
                    Err(DownloadError::Canceled) => return Err(DownloadError::Canceled),
                    Err(error) => last_error = error.to_string(),
                }
            }
            if attempt < self.options.retries {
                tokio::time::sleep(Duration::from_millis(250 * (attempt as u64 + 1))).await;
            }
        }
        Err(DownloadError::Exhausted(last_error))
    }

    #[allow(clippy::too_many_arguments)]
    async fn try_chunk(
        &self,
        url: &str,
        headers: &BTreeMap<String, String>,
        target: &Path,
        chunk: &ChunkState,
        total: u64,
        cancel: &CancellationToken,
        counter: &Arc<AtomicU64>,
        progress: &ProgressCallback,
    ) -> Result<(), DownloadError> {
        let _global = self
            .global_connections
            .acquire()
            .await
            .expect("semaphore open");
        let response = apply_headers(self.client.get(url), headers)
            .header(
                header::RANGE,
                format!("bytes={}-{}", chunk.start, chunk.end),
            )
            .send()
            .await?;
        if response.status() != StatusCode::PARTIAL_CONTENT {
            return Err(DownloadError::Http(response.status()));
        }
        let mut file = OpenOptions::new().write(true).open(target).await?;
        file.seek(std::io::SeekFrom::Start(chunk.start)).await?;
        stream_response(
            response,
            &mut file,
            counter.clone(),
            Some(chunk.end - chunk.start + 1),
            total,
            cancel.clone(),
            progress.clone(),
            true,
        )
        .await?;
        file.flush().await?;
        Ok(())
    }
}

fn new_metadata(validator: &Validator, chunk_size: u64) -> ResumeMetadata {
    ResumeMetadata {
        validator: validator.clone(),
        chunks: resume::make_chunks(validator.total, chunk_size.max(256 * 1024)),
    }
}
