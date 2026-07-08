use serde::{Deserialize, Serialize};

/// Closed set of provider error codes, mirrored 1:1 in the subprocess protocol.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderErrorCode {
    UnsupportedProtocol,
    InvalidParams,
    UnsupportedMethod,
    AuthRequired,
    NotFound,
    Network,
    Canceled,
    Internal,
}

#[derive(Debug, Clone, thiserror::Error, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[error("{message}")]
pub struct ProviderError {
    pub code: ProviderErrorCode,
    pub message: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

impl ProviderError {
    pub fn new(code: ProviderErrorCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
            data: None,
        }
    }

    pub fn invalid_params(message: impl Into<String>) -> Self {
        Self::new(ProviderErrorCode::InvalidParams, message)
    }

    pub fn unsupported_method(method: &str) -> Self {
        Self::new(
            ProviderErrorCode::UnsupportedMethod,
            format!("unsupported method: {method}"),
        )
    }

    pub fn unsupported_capability(provider: &str, capability: &str) -> Self {
        Self::new(
            ProviderErrorCode::UnsupportedMethod,
            format!("provider {provider} does not support {capability}"),
        )
    }

    pub fn no_provider(input: &str) -> Self {
        Self::new(
            ProviderErrorCode::NotFound,
            format!("no provider accepts input: {input}"),
        )
    }

    pub fn not_found(message: impl Into<String>) -> Self {
        Self::new(ProviderErrorCode::NotFound, message)
    }

    pub fn auth_required(message: impl Into<String>) -> Self {
        Self::new(ProviderErrorCode::AuthRequired, message)
    }

    pub fn network(message: impl Into<String>) -> Self {
        Self::new(ProviderErrorCode::Network, message)
    }

    pub fn internal(message: impl Into<String>) -> Self {
        Self::new(ProviderErrorCode::Internal, message)
    }

    pub fn canceled() -> Self {
        Self::new(ProviderErrorCode::Canceled, "canceled")
    }
}
