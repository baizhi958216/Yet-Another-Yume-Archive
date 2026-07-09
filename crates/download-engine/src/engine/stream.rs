//! Single-connection streaming download with append-based resume
//! (`Range: bytes=<existing>-`). Fallback path when ranges are unusable.

use std::sync::{atomic::AtomicU64, Arc};

use reqwest::{header, StatusCode};
use tokio::{
    fs::{self, OpenOptions},
    io::AsyncWriteExt,
};
use tokio_util::sync::CancellationToken;

use super::{apply_headers, stream_response, DownloadEngine, ProgressCallback};
use crate::{model::Validator, DownloadError, DownloadResult, ResourceSpec};

impl DownloadEngine {
    pub(crate) async fn download_stream(
        &self,
        spec: ResourceSpec,
        validator: Validator,
        cancel: CancellationToken,
        progress: ProgressCallback,
    ) -> Result<DownloadResult, DownloadError> {
        let mut last_error = String::new();
        let counter = Arc::new(AtomicU64::new(0));
        for attempt in 0..=self.options.retries {
            for url in &spec.urls {
                if cancel.is_cancelled() {
                    return Err(DownloadError::Canceled);
                }
                match self
                    .try_stream(url, &spec, &validator, &cancel, &counter, &progress)
                    .await
                {
                    Ok(result) => return Ok(result),
                    Err(DownloadError::Canceled) => return Err(DownloadError::Canceled),
                    Err(error) => last_error = error.to_string(),
                }
            }
            if attempt < self.options.retries {
                tokio::time::sleep(std::time::Duration::from_millis(250 * (attempt as u64 + 1)))
                    .await;
            }
        }
        Err(DownloadError::Exhausted(last_error))
    }

    async fn try_stream(
        &self,
        url: &str,
        spec: &ResourceSpec,
        validator: &Validator,
        cancel: &CancellationToken,
        counter: &Arc<AtomicU64>,
        progress: &ProgressCallback,
    ) -> Result<DownloadResult, DownloadError> {
        let existing = fs::metadata(&spec.target)
            .await
            .map(|value| value.len())
            .unwrap_or(0);
        let _permit = self
            .global_connections
            .acquire()
            .await
            .expect("semaphore open");
        let mut request = apply_headers(self.client.get(url), &spec.headers);
        if existing > 0 {
            request = request.header(header::RANGE, format!("bytes={existing}-"));
        }
        let response = request.send().await?;
        if !response.status().is_success() {
            return Err(DownloadError::Http(response.status()));
        }
        let append = existing > 0 && response.status() == StatusCode::PARTIAL_CONTENT;
        let response_total = if append {
            response
                .headers()
                .get(header::CONTENT_RANGE)
                .and_then(|value| value.to_str().ok())
                .and_then(|value| value.rsplit('/').next())
                .and_then(|value| value.parse().ok())
                .unwrap_or(validator.total)
        } else if validator.total > 0 {
            validator.total
        } else {
            response.content_length().unwrap_or(0)
        };
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(append)
            .truncate(!append)
            .open(&spec.target)
            .await?;
        let initial = if append { existing } else { 0 };
        let expected = (response_total > 0)
            .then(|| response_total.checked_sub(initial))
            .flatten();
        counter.store(initial, std::sync::atomic::Ordering::Relaxed);
        stream_response(
            response,
            &mut file,
            counter.clone(),
            expected,
            response_total,
            cancel.clone(),
            progress.clone(),
            false,
        )
        .await?;
        file.flush().await?;
        Ok(DownloadResult {
            path: spec.target.clone(),
            bytes: counter.load(std::sync::atomic::Ordering::Relaxed),
            etag: validator.etag.clone(),
            last_modified: validator.last_modified.clone(),
        })
    }
}
