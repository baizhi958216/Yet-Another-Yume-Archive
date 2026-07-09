mod probe;
mod segmented;
mod stream;
mod transfer;

use std::{sync::Arc, time::Duration};

use reqwest::Client;
use tokio::{fs, sync::Semaphore};
use tokio_util::sync::CancellationToken;

use crate::{
    resume, DownloadError, DownloadOptions, DownloadProgress, DownloadResult, ResourceSpec,
};

pub(crate) use transfer::{apply_headers, header_string, stream_response, ProgressCallback};

#[derive(Clone)]
pub struct DownloadEngine {
    pub(crate) client: Client,
    pub(crate) options: DownloadOptions,
    pub(crate) global_connections: Arc<Semaphore>,
}

impl DownloadEngine {
    pub fn new(
        options: DownloadOptions,
        global_connection_limit: usize,
    ) -> Result<Self, DownloadError> {
        Ok(Self {
            client: Client::builder()
                .user_agent("Mozilla/5.0 YAYA/0.1")
                .timeout(Duration::from_secs(60))
                .build()?,
            options,
            global_connections: Arc::new(Semaphore::new(global_connection_limit.max(1))),
        })
    }

    /// Probe the resource, then download segmented when the server supports
    /// ranges (falling back to streaming on non-cancel errors).
    pub async fn download<F>(
        &self,
        spec: ResourceSpec,
        cancel: CancellationToken,
        progress: F,
    ) -> Result<DownloadResult, DownloadError>
    where
        F: Fn(DownloadProgress) + Send + Sync + 'static,
    {
        if spec.urls.is_empty() {
            return Err(DownloadError::Exhausted("no resource URL".into()));
        }
        if let Some(parent) = spec.target.parent() {
            fs::create_dir_all(parent).await?;
        }
        let callback: ProgressCallback = Arc::new(progress);
        let probe = self.probe(&spec.urls, &spec.headers, &cancel).await?;
        let segmentable = probe.accept_ranges
            && probe.validator.total > 0
            && self.options.segment_concurrency > 1;
        if !segmentable {
            return self
                .download_stream(spec, probe.validator, cancel, callback)
                .await;
        }
        match self
            .download_segmented(
                spec.clone(),
                probe.validator.clone(),
                cancel.clone(),
                callback.clone(),
            )
            .await
        {
            Ok(result) => Ok(result),
            Err(DownloadError::Canceled) => Err(DownloadError::Canceled),
            Err(_) => {
                // segmented state is unusable — wipe it and stream from scratch
                fs::remove_file(&spec.target).await.ok();
                fs::remove_file(resume::resume_path(&spec.target))
                    .await
                    .ok();
                self.download_stream(spec, probe.validator, cancel, callback)
                    .await
            }
        }
    }
}
