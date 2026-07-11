use yaya_provider_api::ProviderError;

#[derive(Debug, thiserror::Error)]
pub enum HostError {
    #[error("provider manifest is invalid: {0}")]
    Manifest(String),
    #[error("provider process failed: {0}")]
    Process(String),
    #[error("provider protocol failed: {0}")]
    Protocol(String),
    /// Structured error returned by the provider itself.
    #[error(transparent)]
    Provider(#[from] ProviderError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
}

impl HostError {
    /// Collapse into the protocol-level error type, preserving the code when
    /// the provider produced one.
    pub fn into_provider_error(self) -> ProviderError {
        match self {
            Self::Provider(error) => error,
            other => ProviderError::internal(other.to_string()),
        }
    }
}
