//! Spawn-per-call subprocess transport: one process per method invocation,
//! JSON request on stdin, event lines + one terminal response on stdout.

use std::{path::PathBuf, process::Stdio, sync::Arc};

use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    process::{Child, Command},
};
use tokio_util::sync::CancellationToken;
use yaya_provider_api::{
    Artifact, BinaryAsset, ProgressReporter, Provider, ProviderError, ProviderInput,
    ProviderTaskRequest, ProviderView,
};

use crate::{
    protocol::{Request, Response},
    HostError, ProviderControl, ProviderDescriptor, ProviderManifest, PROVIDER_PROTOCOL_VERSION,
};

#[derive(Clone)]
pub struct ExternalProvider {
    manifest: ProviderManifest,
    executable: PathBuf,
    data_dir: PathBuf,
}

impl ExternalProvider {
    pub(crate) fn new(manifest: ProviderManifest, executable: PathBuf, data_dir: PathBuf) -> Self {
        Self {
            manifest,
            executable,
            data_dir,
        }
    }

    pub fn id(&self) -> &str {
        &self.manifest.id
    }

    fn spawn(&self) -> Result<Child, HostError> {
        std::fs::create_dir_all(&self.data_dir)?;
        Command::new(&self.executable)
            .env("YAYA_PROVIDER_DATA_DIR", &self.data_dir)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|error| HostError::Process(format!("{}: {error}", self.executable.display())))
    }

    fn encode<P: Serialize>(&self, method: &str, params: P) -> Result<Vec<u8>, HostError> {
        let request = Request {
            protocol_version: PROVIDER_PROTOCOL_VERSION,
            method: method.to_string(),
            params,
        };
        let mut bytes = serde_json::to_vec(&request)?;
        bytes.push(b'\n');
        Ok(bytes)
    }

    /// Single-shot call: spawn, send request, wait for exit, parse the last
    /// stdout line as the terminal response.
    pub async fn call<P: Serialize, T: DeserializeOwned>(
        &self,
        method: &str,
        params: P,
    ) -> Result<T, HostError> {
        let mut child = self.spawn()?;
        let payload = self.encode(method, params)?;
        child
            .stdin
            .take()
            .expect("provider stdin is piped")
            .write_all(&payload)
            .await?;
        let output = child.wait_with_output().await?;
        if !output.status.success() {
            return Err(HostError::Process(
                String::from_utf8_lossy(&output.stderr).trim().to_string(),
            ));
        }
        let terminal = String::from_utf8_lossy(&output.stdout);
        let terminal = terminal
            .lines()
            .rfind(|line| !line.trim().is_empty())
            .ok_or_else(|| HostError::Protocol("provider produced no output".into()))?;
        parse_terminal(terminal)
    }

    /// Streaming call for `run`: forward progress events, kill on cancel.
    async fn run_task(
        &self,
        request: ProviderTaskRequest,
        reporter: Arc<dyn ProgressReporter>,
        cancellation: CancellationToken,
    ) -> Result<Vec<Artifact>, HostError> {
        let mut child = self.spawn()?;
        let payload = self.encode("run", request)?;
        let mut stdin = child.stdin.take().expect("provider stdin is piped");
        stdin.write_all(&payload).await?;
        drop(stdin);

        let stdout = child.stdout.take().expect("provider stdout is piped");
        let mut lines = BufReader::new(stdout).lines();
        let mut result: Option<Result<Vec<Artifact>, HostError>> = None;
        loop {
            let line = tokio::select! {
                _ = cancellation.cancelled() => {
                    child.kill().await?;
                    return Err(HostError::Provider(ProviderError::canceled()));
                }
                line = lines.next_line() => line?,
            };
            let Some(line) = line else { break };
            if line.trim().is_empty() {
                continue;
            }
            let value: serde_json::Value = serde_json::from_str(&line)
                .map_err(|error| HostError::Protocol(format!("{error}; output={}", line.trim())))?;
            match value.get("event").and_then(serde_json::Value::as_str) {
                Some("progress") => {
                    let progress = value.get("progress").cloned().ok_or_else(|| {
                        HostError::Protocol("progress event has no payload".into())
                    })?;
                    reporter.report(serde_json::from_value(progress)?);
                }
                Some(_) => {} // unknown events (log, …) are ignored by design
                None => result = Some(parse_terminal(&line)),
            }
        }
        let status = child.wait().await?;
        if !status.success() {
            return Err(HostError::Process(format!(
                "{} exited with {status}",
                self.executable.display()
            )));
        }
        result.ok_or_else(|| HostError::Protocol("provider returned no result".into()))?
    }
}

fn parse_terminal<T: DeserializeOwned>(line: &str) -> Result<T, HostError> {
    let response: Response<T> = serde_json::from_str(line)
        .map_err(|error| HostError::Protocol(format!("{error}; output={}", line.trim())))?;
    if let Some(error) = response.error {
        return Err(HostError::Provider(error));
    }
    response
        .result
        .ok_or_else(|| HostError::Protocol("provider returned no result".into()))
}

#[async_trait]
impl ProviderControl for ExternalProvider {
    fn descriptor(&self) -> ProviderDescriptor {
        ProviderDescriptor {
            id: self.manifest.id.clone(),
            name: self.manifest.name.clone(),
            version: self.manifest.version.clone(),
            description: self.manifest.description.clone(),
            capabilities: self.manifest.capabilities.clone(),
            enabled_by_default: self.manifest.enabled_by_default,
        }
    }

    async fn invoke(
        &self,
        method: &str,
        params: serde_json::Value,
    ) -> Result<serde_json::Value, HostError> {
        self.call(method, params).await
    }
}

#[async_trait]
impl Provider for ExternalProvider {
    fn id(&self) -> &str {
        &self.manifest.id
    }

    fn supports(&self, input: &str) -> bool {
        self.manifest.matches.iter().any(|rule| rule.matches(input))
    }

    fn priority(&self) -> i32 {
        self.manifest.priority
    }

    async fn inspect(&self, input: ProviderInput) -> Result<ProviderView, ProviderError> {
        self.call("inspect", input)
            .await
            .map_err(HostError::into_provider_error)
    }

    async fn run(
        &self,
        request: ProviderTaskRequest,
        reporter: Arc<dyn ProgressReporter>,
        cancellation: CancellationToken,
    ) -> Result<Vec<Artifact>, ProviderError> {
        self.run_task(request, reporter, cancellation)
            .await
            .map_err(HostError::into_provider_error)
    }

    async fn fetch_asset(&self, url: &str) -> Result<BinaryAsset, ProviderError> {
        self.call("fetch_asset", serde_json::json!({ "url": url }))
            .await
            .map_err(HostError::into_provider_error)
    }
}
