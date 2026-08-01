use serde::{Deserialize, Serialize};

/// Version of the browser-side Provider UI bridge. It is independent from
/// the subprocess protocol version so the two contracts can evolve safely.
pub const PROVIDER_UI_API_VERSION: u32 = 1;

/// A UI surface implemented by a Provider frontend bundle.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderUiSurface {
    /// Stable host-defined surface id (`management` or `resolve`).
    pub id: String,
    #[serde(default = "default_surface_height")]
    pub initial_height: u32,
}

/// Public UI metadata. File paths stay inside the Host and are never exposed
/// to the webview.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderUiDescriptor {
    pub api_version: u32,
    pub surfaces: Vec<ProviderUiSurface>,
}

/// Self-contained, build-time compiled Provider frontend returned to the
/// webview. The Host runs the module in an opaque-origin sandbox.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderUiBundle {
    pub api_version: u32,
    pub surfaces: Vec<ProviderUiSurface>,
    pub module: String,
    #[serde(default)]
    pub style: String,
}

/// Opaque frontend-to-Provider RPC. Action names and payload meanings belong
/// exclusively to the Provider.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderUiActionRequest {
    pub action: String,
    #[serde(default)]
    pub payload: serde_json::Value,
}

const fn default_surface_height() -> u32 {
    320
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ui_bundle_uses_camel_case_wire_fields() {
        let value = serde_json::to_value(ProviderUiBundle {
            api_version: PROVIDER_UI_API_VERSION,
            surfaces: vec![ProviderUiSurface {
                id: "resolve".into(),
                initial_height: 480,
            }],
            module: "export default {}".into(),
            style: String::new(),
        })
        .unwrap();
        assert_eq!(value["apiVersion"], 1);
        assert_eq!(value["surfaces"][0]["initialHeight"], 480);
        assert!(value.get("api_version").is_none());
    }
}
