use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

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
    pub capabilities: ProviderCapabilities,
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

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderCapabilities {
    #[serde(default)]
    pub authentication: bool,
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
    pub capabilities: ProviderCapabilities,
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
