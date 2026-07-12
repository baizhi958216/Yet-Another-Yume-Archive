use crate::TaskStatus;

#[derive(Debug, thiserror::Error)]
pub enum RuntimeError {
    #[error(transparent)]
    Provider(#[from] yaya_provider_api::ProviderError),
    #[error("database operation failed: {0}")]
    Database(#[from] rusqlite::Error),
    #[error("task data is invalid: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("task {0} was not found")]
    NotFound(String),
    #[error("task cannot transition from {from:?} to {to:?}")]
    InvalidTransition { from: TaskStatus, to: TaskStatus },
    #[error("provider {0} is not available")]
    ProviderUnavailable(String),
    #[error("file operation failed: {0}")]
    Io(#[from] std::io::Error),
}
