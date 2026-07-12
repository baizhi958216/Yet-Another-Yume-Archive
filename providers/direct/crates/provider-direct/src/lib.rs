//! Direct HTTP/HTTPS download provider: probes the URL for name/size/type,
//! then downloads with the shared engine. Priority -100 catch-all.

mod naming;

use std::{collections::BTreeMap, sync::Arc, time::Duration};

use reqwest::{header, Client, Response};
use tokio_util::sync::CancellationToken;
use url::Url;
use yaya_download_engine::{DownloadEngine, DownloadOptions, ResourceSpec};
use yaya_provider_api::{
    Artifact, ProgressReporter, Provider, ProviderError, ProviderInput, ProviderTaskRequest,
    ProviderView, TaskDraft, TaskProgress,
};
use yaya_provider_host::{ProviderCapabilities, ProviderControl, ProviderDescriptor};

#[derive(Clone)]
pub struct DirectProvider {
    client: Client,
}

impl DirectProvider {
    pub fn new() -> Result<Self, ProviderError> {
        Ok(Self {
            client: Client::builder()
                .user_agent("Mozilla/5.0 YAYA/0.1")
                .redirect(reqwest::redirect::Policy::limited(10))
                .timeout(Duration::from_secs(30))
                .build()
                .map_err(|error| ProviderError::internal(error.to_string()))?,
        })
    }

    async fn inspect_direct(&self, input: ProviderInput) -> Result<ProviderView, ProviderError> {
        let source_url = parse_url(&input.value)?;
        let response = self.probe(source_url.as_str()).await?;
        let final_url = response.url().clone();
        let size = response_size(&response);
        let mime_type = response
            .headers()
            .get(header::CONTENT_TYPE)
            .and_then(|value| value.to_str().ok())
            .and_then(|value| value.split(';').next())
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .unwrap_or("application/octet-stream")
            .to_string();
        let file_name = response
            .headers()
            .get(header::CONTENT_DISPOSITION)
            .and_then(|value| value.to_str().ok())
            .and_then(naming::file_name_from_disposition)
            .or_else(|| naming::file_name_from_url(&final_url))
            .map(|value| naming::ensure_extension(value, &mime_type))
            .unwrap_or_else(|| naming::default_file_name(&mime_type));
        let host = final_url.host_str().unwrap_or("HTTP").to_string();
        Ok(ProviderView {
            provider: self.id().into(),
            title: file_name.clone(),
            description: host,
            image_url: String::new(),
            tasks: vec![TaskDraft {
                key: source_url.to_string(),
                title: file_name.clone(),
                description: mime_type.clone(),
                size,
                image_url: String::new(),
                selected: true,
                fields: Vec::new(),
                payload: serde_json::json!({
                    "url": final_url.to_string(),
                    "fileName": file_name,
                    "mimeType": mime_type,
                }),
            }],
            fields: Vec::new(),
        })
    }

    async fn probe(&self, url: &str) -> Result<Response, ProviderError> {
        if let Ok(response) = self.client.head(url).send().await {
            if response.status().is_success() {
                return Ok(response);
            }
        }
        let response = self
            .client
            .get(url)
            .header(header::RANGE, "bytes=0-0")
            .send()
            .await
            .map_err(|error| ProviderError::network(error.to_string()))?;
        if !response.status().is_success() {
            return Err(ProviderError::network(format!(
                "HTTP {}",
                response.status()
            )));
        }
        Ok(response)
    }
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct DirectPayload {
    url: String,
    file_name: String,
    mime_type: String,
}

#[async_trait::async_trait]
impl Provider for DirectProvider {
    fn id(&self) -> &str {
        "direct"
    }

    fn supports(&self, input: &str) -> bool {
        parse_url(input).is_ok()
    }

    fn priority(&self) -> i32 {
        -100
    }

    async fn inspect(&self, input: ProviderInput) -> Result<ProviderView, ProviderError> {
        self.inspect_direct(input).await
    }

    async fn run(
        &self,
        request: ProviderTaskRequest,
        reporter: Arc<dyn ProgressReporter>,
        cancellation: CancellationToken,
    ) -> Result<Vec<Artifact>, ProviderError> {
        let payload: DirectPayload = serde_json::from_value(request.task.payload)
            .map_err(|error| ProviderError::invalid_params(error.to_string()))?;
        let io = |error: std::io::Error| ProviderError::internal(error.to_string());
        tokio::fs::create_dir_all(&request.output_dir)
            .await
            .map_err(io)?;
        tokio::fs::create_dir_all(&request.work_dir)
            .await
            .map_err(io)?;
        let temporary = request.work_dir.join("download.part");
        let output = naming::unique_file_path(&request.output_dir, &payload.file_name);
        let engine = DownloadEngine::new(DownloadOptions::default(), 8)
            .map_err(|error| ProviderError::internal(error.to_string()))?;
        engine
            .download(
                ResourceSpec {
                    urls: vec![payload.url],
                    headers: BTreeMap::new(),
                    target: temporary.clone(),
                },
                cancellation,
                move |progress| {
                    reporter.report(TaskProgress {
                        completed: progress.downloaded_bytes,
                        total: progress.total_bytes,
                        rate: progress.speed_bytes_per_second,
                        message: "Downloading".into(),
                    });
                },
            )
            .await
            .map_err(|error| match error {
                yaya_download_engine::DownloadError::Canceled => ProviderError::canceled(),
                other => ProviderError::network(other.to_string()),
            })?;
        tokio::fs::rename(&temporary, &output).await.map_err(io)?;
        let size = tokio::fs::metadata(&output)
            .await
            .ok()
            .map(|value| value.len());
        Ok(vec![Artifact {
            name: output
                .file_name()
                .and_then(|value| value.to_str())
                .unwrap_or("download")
                .into(),
            path: output,
            mime_type: payload.mime_type,
            size,
            metadata: BTreeMap::new(),
        }])
    }
}

#[async_trait::async_trait]
impl ProviderControl for DirectProvider {
    fn descriptor(&self) -> ProviderDescriptor {
        ProviderDescriptor {
            id: self.id().into(),
            name: "Direct Download".into(),
            version: env!("CARGO_PKG_VERSION").into(),
            description: "下载 HTTP 或 HTTPS 地址指向的内容。".into(),
            capabilities: ProviderCapabilities {
                authentication: false,
            },
            enabled_by_default: true,
        }
    }
}

fn parse_url(value: &str) -> Result<Url, ProviderError> {
    let url = Url::parse(value.trim())
        .map_err(|_| ProviderError::invalid_params("invalid direct download URL"))?;
    if !matches!(url.scheme(), "http" | "https") || url.host_str().is_none() {
        return Err(ProviderError::invalid_params("invalid direct download URL"));
    }
    Ok(url)
}

fn response_size(response: &Response) -> Option<u64> {
    response
        .headers()
        .get(header::CONTENT_RANGE)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.rsplit('/').next())
        .filter(|value| *value != "*")
        .and_then(|value| value.parse().ok())
        .or_else(|| response.content_length())
}
