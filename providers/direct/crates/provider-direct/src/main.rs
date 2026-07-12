//! stdio protocol shell (protocol v1): one JSON request on stdin, progress
//! event lines plus one terminal response on stdout.

use std::{io::Read, sync::Arc};

use provider_direct::DirectProvider;
use serde::Deserialize;
use serde_json::{json, Value};
use tokio_util::sync::CancellationToken;
use yaya_provider_api::{
    ProgressReporter, Provider, ProviderError, ProviderInput, ProviderTaskRequest, TaskProgress,
};
use yaya_provider_host::PROVIDER_PROTOCOL_VERSION;

struct StdoutReporter;

impl ProgressReporter for StdoutReporter {
    fn report(&self, progress: TaskProgress) {
        println!("{}", json!({ "event": "progress", "progress": progress }));
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Request {
    protocol_version: u32,
    method: String,
    params: Value,
}

#[tokio::main]
async fn main() {
    let line = match run().await {
        Ok(result) => json!({ "result": result }),
        Err(error) => json!({ "error": error }),
    };
    println!("{line}");
}

async fn run() -> Result<Value, ProviderError> {
    let mut input = Vec::new();
    std::io::stdin()
        .read_to_end(&mut input)
        .map_err(|error| ProviderError::internal(error.to_string()))?;
    let request: Request = serde_json::from_slice(&input)
        .map_err(|error| ProviderError::invalid_params(error.to_string()))?;
    if request.protocol_version != PROVIDER_PROTOCOL_VERSION {
        return Err(ProviderError::new(
            yaya_provider_api::ProviderErrorCode::UnsupportedProtocol,
            format!("unsupported protocol version {}", request.protocol_version),
        ));
    }
    let provider = DirectProvider::new()?;
    let invalid = |error: serde_json::Error| ProviderError::invalid_params(error.to_string());
    let encode = |value: Value| Ok(value);
    match request.method.as_str() {
        "describe" => encode(json!({
            "id": provider.id(),
            "name": "Direct Download",
            "version": env!("CARGO_PKG_VERSION"),
            "capabilities": { "authentication": false },
        })),
        "inspect" => {
            let params: ProviderInput = serde_json::from_value(request.params).map_err(invalid)?;
            let view = provider.inspect(params).await?;
            serde_json::to_value(view).map_err(|e| ProviderError::internal(e.to_string()))
        }
        "run" => {
            let params: ProviderTaskRequest =
                serde_json::from_value(request.params).map_err(invalid)?;
            let artifacts = provider
                .run(params, Arc::new(StdoutReporter), CancellationToken::new())
                .await?;
            serde_json::to_value(artifacts).map_err(|e| ProviderError::internal(e.to_string()))
        }
        method => Err(ProviderError::unsupported_method(method)),
    }
}
