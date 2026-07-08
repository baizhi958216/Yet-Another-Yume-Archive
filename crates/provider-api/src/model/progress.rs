use serde::{Deserialize, Serialize};

/// Generic progress: completed/total are provider-defined units (usually
/// bytes); `message` names the current phase for the UI.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskProgress {
    #[serde(default)]
    pub completed: u64,
    pub total: Option<u64>,
    /// Units per second.
    #[serde(default)]
    pub rate: u64,
    #[serde(default)]
    pub message: String,
}
