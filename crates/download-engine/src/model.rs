use std::{collections::BTreeMap, path::PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadOptions {
    pub segment_concurrency: usize,
    pub chunk_size: u64,
    pub retries: usize,
}

impl Default for DownloadOptions {
    fn default() -> Self {
        Self {
            segment_concurrency: 4,
            chunk_size: 8 * 1024 * 1024,
            retries: 3,
        }
    }
}

/// One resource to fetch: mirror URLs tried in order, extra request headers,
/// and the target file path.
#[derive(Debug, Clone)]
pub struct ResourceSpec {
    pub urls: Vec<String>,
    pub headers: BTreeMap<String, String>,
    pub target: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadProgress {
    pub downloaded_bytes: u64,
    pub total_bytes: Option<u64>,
    pub speed_bytes_per_second: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadResult {
    pub path: PathBuf,
    pub bytes: u64,
    pub etag: Option<String>,
    pub last_modified: Option<String>,
}

/// Server identity captured at probe time; a mismatch on resume invalidates
/// any partial data.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct Validator {
    pub total: u64,
    pub etag: Option<String>,
    pub last_modified: Option<String>,
}

pub(crate) struct Probe {
    pub validator: Validator,
    pub accept_ranges: bool,
}
