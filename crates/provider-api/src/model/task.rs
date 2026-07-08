use std::{collections::BTreeMap, path::PathBuf};

use serde::{Deserialize, Serialize};

use super::FormField;

/// One downloadable item offered by a provider.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskDraft {
    pub key: String,
    pub title: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub size: Option<u64>,
    #[serde(default)]
    pub image_url: String,
    #[serde(default = "default_true")]
    pub selected: bool,
    /// Per-task options, on top of the view-level fields.
    #[serde(default)]
    pub fields: Vec<FormField>,
    /// Opaque data understood only by the provider.
    #[serde(default)]
    pub payload: serde_json::Value,
}

const fn default_true() -> bool {
    true
}

/// Everything a provider needs to execute one accepted task.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderTaskRequest {
    pub id: String,
    /// The original user input the task came from.
    pub source: String,
    pub task: TaskDraft,
    /// User answers to the declared form fields, keyed by field key.
    #[serde(default)]
    pub options: BTreeMap<String, serde_json::Value>,
    pub output_dir: PathBuf,
    /// Scratch space owned by the runtime; wiped after completion.
    pub work_dir: PathBuf,
    #[serde(default)]
    pub settings: BTreeMap<String, serde_json::Value>,
}
