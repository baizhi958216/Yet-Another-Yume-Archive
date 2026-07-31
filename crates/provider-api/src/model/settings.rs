use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use super::FormField;

/// Provider-declared settings page rendered generically by the host.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderSettingsView {
    #[serde(default)]
    pub sections: Vec<ProviderSettingsSection>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_page: Option<ProviderSettingsPage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderSettingsSection {
    pub key: String,
    pub title: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub fields: Vec<FormField>,
    #[serde(default)]
    pub statuses: Vec<ProviderSettingStatus>,
    #[serde(default)]
    pub actions: Vec<ProviderSettingAction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderSettingStatus {
    pub key: String,
    pub label: String,
    pub available: bool,
    pub value: String,
    #[serde(default)]
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderSettingAction {
    pub key: String,
    pub label: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub style: ProviderSettingActionStyle,
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderSettingActionStyle {
    Primary,
    #[default]
    Secondary,
    Danger,
}

/// Optional Provider-owned HTML page. The host loads it in a script-only,
/// opaque-origin sandbox and exposes only the settings message bridge.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderSettingsPage {
    pub html: String,
    #[serde(default = "default_page_height")]
    pub height: u32,
}

const fn default_page_height() -> u32 {
    480
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderSettingsState {
    #[serde(default)]
    pub values: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderSettingsActionRequest {
    pub action: String,
    #[serde(default)]
    pub values: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderSettingsActionResult {
    #[serde(default)]
    pub message: String,
    #[serde(default)]
    pub refresh: bool,
}
