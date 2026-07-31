use serde::{Deserialize, Serialize};

/// Provider-owned authentication page. The host renders the HTML in an
/// opaque-origin sandbox and exposes only the auth action bridge.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderAuthPage {
    pub html: String,
    #[serde(default = "default_page_height")]
    pub height: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderAuthActionRequest {
    pub action: String,
    #[serde(default)]
    pub payload: serde_json::Value,
}

const fn default_page_height() -> u32 {
    480
}
