use serde::{Deserialize, Serialize};

/// A provider-declared option the host renders blindly.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FormField {
    pub key: String,
    pub label: String,
    #[serde(default)]
    pub description: String,
    #[serde(flatten)]
    pub control: FormControl,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum FormControl {
    Toggle {
        #[serde(default)]
        default: bool,
    },
    Select {
        options: Vec<SelectOption>,
        #[serde(default)]
        default: serde_json::Value,
    },
    Text {
        #[serde(default)]
        default: String,
        #[serde(default)]
        placeholder: String,
    },
    Secret {
        #[serde(default)]
        default: String,
        #[serde(default)]
        placeholder: String,
    },
    Number {
        default: f64,
        min: Option<f64>,
        max: Option<f64>,
        step: Option<f64>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SelectOption {
    pub label: String,
    pub value: serde_json::Value,
    #[serde(default)]
    pub description: String,
}
