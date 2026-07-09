//! Range probe: a `bytes=0-0` request that discovers total size, range
//! support and the resume validator (ETag/Last-Modified).

use std::collections::BTreeMap;

use reqwest::{header, StatusCode};
use tokio_util::sync::CancellationToken;

use super::{apply_headers, header_string, DownloadEngine};
use crate::{
    model::{Probe, Validator},
    DownloadError,
};

impl DownloadEngine {
    pub(crate) async fn probe(
        &self,
        urls: &[String],
        headers: &BTreeMap<String, String>,
        cancel: &CancellationToken,
    ) -> Result<Probe, DownloadError> {
        let mut last_error = String::new();
        for url in urls {
            if cancel.is_cancelled() {
                return Err(DownloadError::Canceled);
            }
            let _permit = self
                .global_connections
                .acquire()
                .await
                .expect("semaphore open");
            match apply_headers(self.client.get(url), headers)
                .header(header::RANGE, "bytes=0-0")
                .send()
                .await
            {
                Ok(response) if response.status().is_success() => {
                    let status = response.status();
                    let headers = response.headers();
                    let total = headers
                        .get(header::CONTENT_RANGE)
                        .and_then(|value| value.to_str().ok())
                        .and_then(|value| value.rsplit('/').next())
                        .and_then(|value| value.parse().ok())
                        .or_else(|| response.content_length())
                        .unwrap_or(0);
                    return Ok(Probe {
                        validator: Validator {
                            total,
                            etag: header_string(headers, header::ETAG),
                            last_modified: header_string(headers, header::LAST_MODIFIED),
                        },
                        accept_ranges: status == StatusCode::PARTIAL_CONTENT,
                    });
                }
                Ok(response) => last_error = format!("HTTP {}", response.status()),
                Err(error) => last_error = error.to_string(),
            }
        }
        Err(DownloadError::Exhausted(last_error))
    }
}
