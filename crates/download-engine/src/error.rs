use reqwest::StatusCode;

#[derive(Debug, thiserror::Error)]
pub enum DownloadError {
    #[error("download canceled")]
    Canceled,
    #[error("HTTP request failed: {0}")]
    Network(#[from] reqwest::Error),
    #[error("file operation failed: {0}")]
    Io(#[from] std::io::Error),
    #[error("server returned HTTP {0}")]
    Http(StatusCode),
    #[error("server returned an invalid byte range")]
    InvalidRange,
    #[error("all download URLs failed: {0}")]
    Exhausted(String),
    #[error("invalid resume metadata: {0}")]
    Resume(String),
}
