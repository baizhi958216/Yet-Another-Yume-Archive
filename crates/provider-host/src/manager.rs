//! Enable/disable management: persists the enabled set and mirrors it into
//! the shared `ProviderRegistry`.

use std::{
    collections::{BTreeMap, BTreeSet},
    path::{Path, PathBuf},
    sync::{Arc, RwLock},
};

use serde::{Deserialize, Serialize};
use yaya_provider_api::ProviderRegistry;

use crate::{HostError, HostedProvider, ProviderControl, ProviderInfo};

#[derive(Clone)]
pub struct ProviderManager {
    inner: Arc<ManagerInner>,
}

struct ManagerInner {
    providers: BTreeMap<String, HostedProvider>,
    enabled: RwLock<BTreeSet<String>>,
    state_path: PathBuf,
    registry: ProviderRegistry,
}

#[derive(Default, Deserialize, Serialize)]
struct ProviderState {
    enabled: BTreeSet<String>,
}

impl ProviderManager {
    pub fn open(
        discovered: Vec<HostedProvider>,
        state_path: PathBuf,
        registry: ProviderRegistry,
    ) -> Result<Self, HostError> {
        let providers = discovered
            .into_iter()
            .map(|provider| (provider.id(), provider))
            .collect::<BTreeMap<_, _>>();
        let enabled = if state_path.is_file() {
            load_enabled(&state_path)?
        } else {
            providers
                .values()
                .filter(|provider| provider.control().descriptor().enabled_by_default)
                .map(HostedProvider::id)
                .collect()
        };
        for id in &enabled {
            if let Some(provider) = providers.get(id) {
                registry.register_shared(provider.source());
            }
        }
        Ok(Self {
            inner: Arc::new(ManagerInner {
                providers,
                enabled: RwLock::new(enabled),
                state_path,
                registry,
            }),
        })
    }

    pub fn list(&self) -> Vec<ProviderInfo> {
        let enabled = self.inner.enabled.read().expect("provider state poisoned");
        self.inner
            .providers
            .values()
            .map(|provider| provider.info(enabled.contains(&provider.id())))
            .collect()
    }

    pub fn provider(&self, id: &str) -> Option<HostedProvider> {
        self.inner.providers.get(id).cloned()
    }

    pub fn set_enabled(&self, id: &str, value: bool) -> Result<ProviderInfo, HostError> {
        let provider = self
            .inner
            .providers
            .get(id)
            .ok_or_else(|| HostError::Manifest(format!("provider {id} was not found")))?;
        let mut enabled = self.inner.enabled.write().expect("provider state poisoned");
        if value {
            enabled.insert(id.to_string());
            self.inner.registry.register_shared(provider.source());
        } else {
            enabled.remove(id);
            self.inner.registry.unregister(id);
        }
        persist_state(&self.inner.state_path, &enabled)?;
        Ok(provider.info(value))
    }

    /// Forward a control-plane method (auth etc.) to one provider.
    pub async fn invoke(
        &self,
        id: &str,
        method: &str,
        params: serde_json::Value,
    ) -> Result<serde_json::Value, HostError> {
        self.control(id)?.invoke(method, params).await
    }

    fn control(&self, id: &str) -> Result<Arc<dyn ProviderControl>, HostError> {
        self.inner
            .providers
            .get(id)
            .map(HostedProvider::control)
            .ok_or_else(|| HostError::Manifest(format!("provider {id} was not found")))
    }
}

fn load_enabled(path: &Path) -> Result<BTreeSet<String>, HostError> {
    let state = serde_json::from_slice::<ProviderState>(&std::fs::read(path)?)?;
    Ok(state.enabled)
}

fn persist_state(path: &Path, enabled: &BTreeSet<String>) -> Result<(), HostError> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(
        path,
        serde_json::to_vec_pretty(&ProviderState {
            enabled: enabled.clone(),
        })?,
    )?;
    Ok(())
}
