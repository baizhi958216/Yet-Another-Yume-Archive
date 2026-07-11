//! Pairing of a provider's execution surface (`Provider`) with its management
//! surface (`ProviderControl`), regardless of whether it runs in-process or
//! as an external subprocess.

use std::sync::Arc;

use async_trait::async_trait;
use yaya_provider_api::Provider;

use crate::{HostError, ProviderCapabilities, ProviderInfo};

#[derive(Debug, Clone)]
pub struct ProviderDescriptor {
    pub id: String,
    pub name: String,
    pub version: String,
    pub description: String,
    pub capabilities: ProviderCapabilities,
    pub enabled_by_default: bool,
}

#[async_trait]
pub trait ProviderControl: Send + Sync {
    fn descriptor(&self) -> ProviderDescriptor;

    /// Generic control-plane dispatch (auth methods etc.). The host forwards
    /// method names blindly; the closed set lives in the protocol doc.
    async fn invoke(
        &self,
        method: &str,
        _params: serde_json::Value,
    ) -> Result<serde_json::Value, HostError> {
        Err(HostError::Protocol(format!(
            "provider control method is not supported: {method}"
        )))
    }
}

#[derive(Clone)]
pub struct HostedProvider {
    source: Arc<dyn Provider>,
    control: Arc<dyn ProviderControl>,
}

impl HostedProvider {
    pub fn new<P>(provider: P) -> Self
    where
        P: Provider + ProviderControl + 'static,
    {
        let provider = Arc::new(provider);
        Self {
            source: provider.clone(),
            control: provider,
        }
    }

    pub fn id(&self) -> String {
        self.control.descriptor().id
    }

    pub fn source(&self) -> Arc<dyn Provider> {
        self.source.clone()
    }

    pub(crate) fn control(&self) -> Arc<dyn ProviderControl> {
        self.control.clone()
    }

    pub(crate) fn info(&self, enabled: bool) -> ProviderInfo {
        let descriptor = self.control.descriptor();
        ProviderInfo {
            id: descriptor.id,
            name: descriptor.name,
            version: descriptor.version,
            description: descriptor.description,
            enabled,
            capabilities: descriptor.capabilities,
        }
    }
}
