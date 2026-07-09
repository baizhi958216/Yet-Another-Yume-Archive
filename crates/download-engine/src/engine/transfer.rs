//! Shared response-body streaming: writes chunks to the file, keeps the
//! shared byte counter honest (rollback on error) and throttles progress.

use std::{
    collections::BTreeMap,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
    time::{Duration, Instant},
};

use futures_util::StreamExt;
use reqwest::header;
use tokio::{fs::File, io::AsyncWriteExt};
use tokio_util::sync::CancellationToken;

use crate::{DownloadError, DownloadProgress};

pub(crate) type ProgressCallback = Arc<dyn Fn(DownloadProgress) + Send + Sync>;

const PROGRESS_INTERVAL: Duration = Duration::from_millis(250);

pub(crate) fn apply_headers(
    mut request: reqwest::RequestBuilder,
    headers: &BTreeMap<String, String>,
) -> reqwest::RequestBuilder {
    for (name, value) in headers {
        request = request.header(name, value);
    }
    request
}

pub(crate) fn header_string(
    headers: &header::HeaderMap,
    name: header::HeaderName,
) -> Option<String> {
    headers
        .get(name)
        .and_then(|value| value.to_str().ok())
        .map(str::to_string)
}

#[allow(clippy::too_many_arguments)]
pub(crate) async fn stream_response(
    response: reqwest::Response,
    file: &mut File,
    counter: Arc<AtomicU64>,
    expected: Option<u64>,
    total: u64,
    cancel: CancellationToken,
    progress: ProgressCallback,
    rollback_on_error: bool,
) -> Result<(), DownloadError> {
    let started = Instant::now();
    let initial = counter.load(Ordering::Relaxed);
    let mut received = 0;
    let mut stream = response.bytes_stream();
    let mut last_emit = Instant::now() - Duration::from_secs(1);
    while let Some(value) = tokio::select! {
        _ = cancel.cancelled() => {
            rollback(&counter, received, rollback_on_error);
            return Err(DownloadError::Canceled);
        },
        value = stream.next() => value,
    } {
        let bytes = match value {
            Ok(bytes) => bytes,
            Err(error) => {
                rollback(&counter, received, rollback_on_error);
                return Err(error.into());
            }
        };
        if let Err(error) = file.write_all(&bytes).await {
            rollback(&counter, received, rollback_on_error);
            return Err(error.into());
        }
        received += bytes.len() as u64;
        let downloaded =
            counter.fetch_add(bytes.len() as u64, Ordering::Relaxed) + bytes.len() as u64;
        if last_emit.elapsed() >= PROGRESS_INTERVAL {
            let elapsed = started.elapsed().as_secs_f64().max(0.001);
            progress(DownloadProgress {
                downloaded_bytes: downloaded,
                total_bytes: (total > 0).then_some(total),
                speed_bytes_per_second: ((downloaded - initial) as f64 / elapsed) as u64,
            });
            last_emit = Instant::now();
        }
    }
    // a short read means the server lied about the range
    if expected.is_some_and(|value| value != received) {
        rollback(&counter, received, rollback_on_error);
        return Err(DownloadError::InvalidRange);
    }
    progress(DownloadProgress {
        downloaded_bytes: counter.load(Ordering::Relaxed),
        total_bytes: (total > 0).then_some(total),
        speed_bytes_per_second: (received as f64 / started.elapsed().as_secs_f64().max(0.001))
            as u64,
    });
    Ok(())
}

fn rollback(counter: &AtomicU64, received: u64, enabled: bool) {
    if enabled && received > 0 {
        counter.fetch_sub(received, Ordering::Relaxed);
    }
}
