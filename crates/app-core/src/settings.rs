//! App-level settings persisted as JSON in the data dir (runtime settings
//! such as concurrency live in the task runtime's own storage).

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::AppError;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppSettings {
    pub default_output_dir: PathBuf,
    pub max_active_tasks: usize,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct StoredSettings {
    pub default_output_dir: Option<PathBuf>,
}

pub(crate) fn load(path: &Path) -> StoredSettings {
    std::fs::read(path)
        .ok()
        .and_then(|bytes| serde_json::from_slice(&bytes).ok())
        .unwrap_or_default()
}

pub(crate) fn save(path: &Path, value: &StoredSettings) -> Result<(), AppError> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(
        path,
        serde_json::to_vec_pretty(value).map_err(AppError::internal)?,
    )?;
    Ok(())
}
