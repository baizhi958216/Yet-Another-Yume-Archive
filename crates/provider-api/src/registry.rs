use std::sync::{Arc, RwLock};

use crate::{Provider, ProviderError, ProviderInput, ProviderView};

/// Shared set of active providers; picks the highest-priority match for an
/// input. Registration replaces any provider with the same id.
#[derive(Clone, Default)]
pub struct ProviderRegistry {
    providers: Arc<RwLock<Vec<Arc<dyn Provider>>>>,
}

impl ProviderRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register<P: Provider + 'static>(&self, provider: P) {
        self.register_shared(Arc::new(provider));
    }

    pub fn register_shared(&self, provider: Arc<dyn Provider>) {
        let mut providers = self.providers.write().expect("provider registry poisoned");
        providers.retain(|value| value.id() != provider.id());
        providers.push(provider);
    }

    pub fn unregister(&self, id: &str) -> Option<Arc<dyn Provider>> {
        let mut providers = self.providers.write().expect("provider registry poisoned");
        let index = providers.iter().position(|value| value.id() == id)?;
        Some(providers.remove(index))
    }

    pub fn ids(&self) -> Vec<String> {
        self.read()
            .iter()
            .map(|provider| provider.id().to_string())
            .collect()
    }

    pub fn provider(&self, id: &str) -> Option<Arc<dyn Provider>> {
        self.read().iter().find(|value| value.id() == id).cloned()
    }

    pub fn provider_for_input(&self, input: &str) -> Option<Arc<dyn Provider>> {
        self.read()
            .iter()
            .filter(|provider| provider.supports(input))
            .max_by_key(|provider| provider.priority())
            .cloned()
    }

    /// Inspect via the best-matching provider, stamping its id on the view.
    pub async fn inspect(&self, input: ProviderInput) -> Result<ProviderView, ProviderError> {
        let provider = self
            .provider_for_input(&input.value)
            .ok_or_else(|| ProviderError::no_provider(&input.value))?;
        let mut view = provider.inspect(input).await?;
        view.provider = provider.id().to_string();
        Ok(view)
    }

    fn read(&self) -> std::sync::RwLockReadGuard<'_, Vec<Arc<dyn Provider>>> {
        self.providers.read().expect("provider registry poisoned")
    }
}
