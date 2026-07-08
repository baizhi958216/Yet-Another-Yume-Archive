//! Resume sidecar: `<target>.resume.json` written atomically next to the
//! partially downloaded file.

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use tokio::fs;

use crate::{error::DownloadError, model::Validator};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ResumeMetadata {
    pub validator: Validator,
    pub chunks: Vec<ChunkState>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ChunkState {
    pub start: u64,
    pub end: u64,
    pub complete: bool,
}

pub(crate) fn make_chunks(total: u64, size: u64) -> Vec<ChunkState> {
    let mut chunks = Vec::new();
    let mut start = 0;
    while start < total {
        let end = (start + size - 1).min(total - 1);
        chunks.push(ChunkState {
            start,
            end,
            complete: false,
        });
        start = end + 1;
    }
    chunks
}

pub(crate) fn resume_path(target: &Path) -> PathBuf {
    let mut value = target.as_os_str().to_os_string();
    value.push(".resume.json");
    PathBuf::from(value)
}

pub(crate) async fn load(path: &Path) -> Option<ResumeMetadata> {
    let bytes = fs::read(path).await.ok()?;
    serde_json::from_slice(&bytes).ok()
}

pub(crate) async fn persist(path: &Path, metadata: &ResumeMetadata) -> Result<(), DownloadError> {
    let temp = path.with_extension("json.tmp");
    let bytes =
        serde_json::to_vec(metadata).map_err(|error| DownloadError::Resume(error.to_string()))?;
    fs::write(&temp, bytes).await?;
    fs::rename(temp, path).await?;
    Ok(())
}
