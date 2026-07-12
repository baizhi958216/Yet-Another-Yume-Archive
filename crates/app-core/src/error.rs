use serde::Serialize;

/// Serializable error every host wrapper can hand to its transport.
#[derive(Debug, Clone, Serialize, thiserror::Error)]
#[serde(rename_all = "camelCase")]
#[error("{message}")]
pub struct AppError {
    pub code: String,
    pub message: String,
}

impl AppError {
    pub fn new(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
        }
    }

    pub fn internal(message: impl std::fmt::Display) -> Self {
        Self::new("internal", message.to_string())
    }
}

impl From<yaya_task_runtime::RuntimeError> for AppError {
    fn from(error: yaya_task_runtime::RuntimeError) -> Self {
        match error {
            yaya_task_runtime::RuntimeError::Provider(error) => error.into(),
            other => Self::internal(other),
        }
    }
}

impl From<yaya_provider_api::ProviderError> for AppError {
    fn from(error: yaya_provider_api::ProviderError) -> Self {
        let code = serde_json::to_value(error.code)
            .ok()
            .and_then(|value| value.as_str().map(str::to_string))
            .unwrap_or_else(|| "internal".into());
        Self::new(code, error.message)
    }
}

impl From<yaya_provider_host::HostError> for AppError {
    fn from(error: yaya_provider_host::HostError) -> Self {
        error.into_provider_error().into()
    }
}

impl From<std::io::Error> for AppError {
    fn from(error: std::io::Error) -> Self {
        Self::internal(error)
    }
}
