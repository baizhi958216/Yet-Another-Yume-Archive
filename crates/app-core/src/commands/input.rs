use yaya_provider_api::{BinaryAsset, ProviderView};

use crate::{AppCore, AppError};

impl AppCore {
    /// Resolve raw user input into a provider view via the registry.
    pub async fn inspect_source(&self, source: String) -> Result<ProviderView, AppError> {
        Ok(self.runtime.inspect(source).await?)
    }

    /// Fetch a small asset (cover image …) through the owning provider's
    /// network context.
    pub async fn fetch_provider_asset(
        &self,
        provider_id: &str,
        url: &str,
    ) -> Result<BinaryAsset, AppError> {
        Ok(self.runtime.fetch_provider_asset(provider_id, url).await?)
    }
}
