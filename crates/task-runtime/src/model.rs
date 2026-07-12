use std::{collections::BTreeMap, path::PathBuf};

use serde::{Deserialize, Serialize};
use yaya_provider_api::{Artifact, TaskDraft};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeSettings {
    pub max_active_tasks: usize,
}

impl Default for RuntimeSettings {
    fn default() -> Self {
        Self {
            max_active_tasks: 3,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTasksRequest {
    pub provider: String,
    pub source: String,
    pub output_dir: PathBuf,
    pub tasks: Vec<CreateTask>,
    pub batch_id: Option<String>,
    /// Optional collection title; multi-task batches download into a
    /// sub-directory of this name so they don't scatter across `output_dir`.
    #[serde(default)]
    pub group: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTask {
    pub draft: TaskDraft,
    #[serde(default)]
    pub options: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskStatus {
    Queued,
    Running,
    Paused,
    Completed,
    Failed,
    Canceled,
}

impl TaskStatus {
    pub fn is_running(self) -> bool {
        self == Self::Running
    }

    pub fn is_terminal(self) -> bool {
        matches!(self, Self::Completed | Self::Canceled)
    }

    pub fn can_transition_to(self, next: Self) -> bool {
        use TaskStatus::*;
        matches!(
            (self, next),
            (Queued, Running)
                | (Queued, Paused)
                | (Queued, Canceled)
                | (Running, Paused)
                | (Running, Completed)
                | (Running, Failed)
                | (Running, Canceled)
                | (Paused, Queued)
                | (Paused, Canceled)
                | (Failed, Queued)
                | (Failed, Canceled)
        ) || self == next
    }

    pub fn name(self) -> &'static str {
        match self {
            Self::Queued => "queued",
            Self::Running => "running",
            Self::Paused => "paused",
            Self::Completed => "completed",
            Self::Failed => "failed",
            Self::Canceled => "canceled",
        }
    }
}

/// Full persisted state of one task; also the event payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskSnapshot {
    pub id: String,
    pub provider: String,
    pub batch_id: Option<String>,
    /// Collection title of the batch this task belongs to (display only).
    #[serde(default)]
    pub group: Option<String>,
    pub source: String,
    pub draft: TaskDraft,
    #[serde(default)]
    pub options: BTreeMap<String, serde_json::Value>,
    pub output_dir: PathBuf,
    pub status: TaskStatus,
    #[serde(default)]
    pub completed: u64,
    pub total: Option<u64>,
    #[serde(default)]
    pub rate: u64,
    #[serde(default)]
    pub message: String,
    pub error: Option<String>,
    #[serde(default)]
    pub warnings: Vec<String>,
    #[serde(default)]
    pub artifacts: Vec<Artifact>,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskEvent {
    pub sequence: u64,
    pub task: TaskSnapshot,
}
