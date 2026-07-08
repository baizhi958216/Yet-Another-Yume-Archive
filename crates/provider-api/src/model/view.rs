use serde::{Deserialize, Serialize};

use super::{FormField, TaskDraft};

/// Raw user input handed to a provider for inspection.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderInput {
    pub value: String,
}

impl From<String> for ProviderInput {
    fn from(value: String) -> Self {
        Self { value }
    }
}

/// A provider-owned interaction model. The host renders it without knowing
/// what the tasks mean or how they will be executed.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderView {
    /// Stamped by the registry, not the provider itself.
    #[serde(default)]
    pub provider: String,
    pub title: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub image_url: String,
    pub tasks: Vec<TaskDraft>,
    /// View-level options applied to every selected task.
    #[serde(default)]
    pub fields: Vec<FormField>,
}
