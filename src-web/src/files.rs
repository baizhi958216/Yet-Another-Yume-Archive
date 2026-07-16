//! Browser file delivery: wait for the task to finish, stream the artifact,
//! and delete the isolated download dir once the stream is dropped.

use std::{
    path::PathBuf,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};

use axum::{
    body::Body,
    extract::{Path as AxumPath, State},
    http::{header, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
};
use futures_util::Stream;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use tokio_util::io::ReaderStream;
use yaya_app_core::AppError;
use yaya_task_runtime::TaskStatus;

use crate::{error::ApiError, WebState};

pub(crate) async fn download_task_file(
    State(state): State<Arc<WebState>>,
    AxumPath(id): AxumPath<String>,
) -> Result<Response, ApiError> {
    let mut events = state.core.subscribe();
    let task = loop {
        let task = state.core.get_task(&id).await?;
        match task.status {
            TaskStatus::Completed => break task,
            TaskStatus::Failed | TaskStatus::Canceled => {
                // the task list already exposes the failure; an empty response
                // stops browsers from saving the error body as a bogus file
                return Ok(StatusCode::NO_CONTENT.into_response());
            }
            _ => {
                events.recv().await.map_err(AppError::internal)?;
            }
        }
    };
    let artifact = task
        .artifacts
        .first()
        .ok_or_else(|| AppError::new("not_found", "completed task has no downloadable file"))?;
    let file = tokio::fs::File::open(&artifact.path).await?;
    let size = file.metadata().await?.len();
    let filename = artifact
        .path
        .file_name()
        .and_then(|value| value.to_str())
        .unwrap_or("download");
    let encoded = utf8_percent_encode(filename, NON_ALPHANUMERIC).to_string();
    let disposition = HeaderValue::from_str(&format!(
        "attachment; filename=\"download\"; filename*=UTF-8''{encoded}"
    ))
    .map_err(AppError::internal)?;
    let stream = CleanupStream {
        inner: ReaderStream::new(file),
        path: artifact.path.clone(),
    };
    Response::builder()
        .header(header::CONTENT_TYPE, artifact.mime_type.as_str())
        .header(header::CONTENT_LENGTH, size)
        .header(header::CONTENT_DISPOSITION, disposition)
        .body(Body::from_stream(stream))
        .map_err(|error| ApiError::from(AppError::internal(error)))
}

/// Streams the file, then removes it (and its per-request dir) on drop.
struct CleanupStream {
    inner: ReaderStream<tokio::fs::File>,
    path: PathBuf,
}

impl Stream for CleanupStream {
    type Item = std::io::Result<bytes::Bytes>;

    fn poll_next(mut self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Pin::new(&mut self.inner).poll_next(context)
    }
}

impl Drop for CleanupStream {
    fn drop(&mut self) {
        let path = self.path.clone();
        tokio::spawn(async move {
            tokio::fs::remove_file(&path).await.ok();
            if let Some(parent) = path.parent() {
                tokio::fs::remove_dir(parent.join(".yaya")).await.ok();
                tokio::fs::remove_dir(parent).await.ok();
            }
        });
    }
}
