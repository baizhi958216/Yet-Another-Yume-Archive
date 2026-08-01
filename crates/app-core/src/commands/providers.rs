//! Provider discovery, enablement and generic Provider-owned UI dispatch.

use yaya_provider_api::{ProviderUiActionRequest, ProviderUiBundle};
use yaya_provider_host::ProviderInfo;

use crate::{AppCore, AppError};

impl AppCore {
    pub fn list_providers(&self) -> Vec<ProviderInfo> {
        self.providers.list()
    }

    pub fn set_provider_enabled(&self, id: &str, enabled: bool) -> Result<ProviderInfo, AppError> {
        Ok(self.providers.set_enabled(id, enabled)?)
    }

    pub fn provider_ui_bundle(&self, id: &str) -> Result<ProviderUiBundle, AppError> {
        self.providers
            .ui_bundle(id)?
            .ok_or_else(|| AppError::internal(format!("Provider {id} 没有前端组件")))
    }

    pub async fn provider_ui_invoke(
        &self,
        id: &str,
        request: ProviderUiActionRequest,
    ) -> Result<serde_json::Value, AppError> {
        if request.action.trim().is_empty() {
            return Err(AppError::internal("Provider UI action 不能为空"));
        }
        let params = serde_json::to_value(request).map_err(AppError::internal)?;
        Ok(self.providers.invoke(id, "ui_invoke", params).await?)
    }
}
