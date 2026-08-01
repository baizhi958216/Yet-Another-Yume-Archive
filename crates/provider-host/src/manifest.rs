use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use yaya_provider_api::{ProviderUiDescriptor, ProviderUiSurface};

/// Parsed `provider.json`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderManifest {
    pub schema_version: u32,
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub version: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub ui: Option<ProviderUiManifest>,
    #[serde(default = "default_enabled")]
    pub enabled_by_default: bool,
    #[serde(default)]
    pub matches: Vec<MatchRule>,
    #[serde(default)]
    pub priority: i32,
    /// Candidate executable paths (relative to the package dir) per target triple.
    pub executables: BTreeMap<String, Vec<String>>,
}

const fn default_enabled() -> bool {
    true
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderUiManifest {
    pub api_version: u32,
    pub entry: String,
    #[serde(default)]
    pub style: Option<String>,
    pub surfaces: Vec<ProviderUiSurface>,
}

impl ProviderUiManifest {
    pub(crate) fn descriptor(&self) -> ProviderUiDescriptor {
        ProviderUiDescriptor {
            api_version: self.api_version,
            surfaces: self.surfaces.clone(),
        }
    }
}

/// Host-facing summary shown in the providers UI.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderInfo {
    pub id: String,
    pub name: String,
    pub version: String,
    pub description: String,
    pub enabled: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ui: Option<ProviderUiDescriptor>,
}

/// Process-free pre-filter deciding whether an input belongs to a provider.
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum MatchRule {
    Contains {
        value: String,
    },
    Prefix {
        value: String,
        #[serde(default, rename = "thenDigits")]
        then_digits: bool,
    },
    Digits,
}

impl MatchRule {
    pub(crate) fn matches(&self, input: &str) -> bool {
        let input = input.trim().to_ascii_lowercase();
        match self {
            Self::Contains { value } => input.contains(&value.to_ascii_lowercase()),
            Self::Prefix { value, then_digits } => {
                let Some(rest) = input.strip_prefix(&value.to_ascii_lowercase()) else {
                    return false;
                };
                !*then_digits
                    || (!rest.is_empty() && rest.chars().all(|value| value.is_ascii_digit()))
            }
            Self::Digits => !input.is_empty() && input.chars().all(|value| value.is_ascii_digit()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_ui_manifest_without_site_capabilities() {
        let manifest: ProviderManifest = serde_json::from_value(serde_json::json!({
            "schemaVersion": 2,
            "id": "example",
            "name": "Example",
            "ui": {
                "apiVersion": 1,
                "entry": "ui/dist/provider-ui.js",
                "style": "ui/dist/provider-ui.css",
                "surfaces": [{ "id": "management", "initialHeight": 360 }]
            },
            "executables": { "aarch64-apple-darwin": ["bin/example"] }
        }))
        .unwrap();
        let ui = manifest.ui.unwrap();
        assert_eq!(ui.entry, "ui/dist/provider-ui.js");
        assert_eq!(ui.descriptor().surfaces[0].id, "management");
    }
}
