//! Provider management and typed control-plane passthrough for Provider-owned
//! authentication and settings.

use yaya_provider_api::{
    ProviderAuthActionRequest, ProviderAuthPage, ProviderSettingsActionRequest,
    ProviderSettingsActionResult, ProviderSettingsState, ProviderSettingsView,
};
use yaya_provider_host::ProviderInfo;

use crate::{AppCore, AppError};

impl AppCore {
    pub fn list_providers(&self) -> Vec<ProviderInfo> {
        self.providers.list()
    }

    pub fn set_provider_enabled(&self, id: &str, enabled: bool) -> Result<ProviderInfo, AppError> {
        Ok(self.providers.set_enabled(id, enabled)?)
    }

    pub async fn provider_auth_describe(&self, id: &str) -> Result<ProviderAuthPage, AppError> {
        self.provider_invoke(id, "auth_describe", serde_json::json!({}))
            .await
    }

    pub async fn provider_auth_invoke(
        &self,
        id: &str,
        request: ProviderAuthActionRequest,
    ) -> Result<serde_json::Value, AppError> {
        let params = serde_json::to_value(request).map_err(AppError::internal)?;
        self.provider_invoke(id, "auth_invoke", params).await
    }

    pub async fn provider_settings_describe(
        &self,
        id: &str,
    ) -> Result<ProviderSettingsView, AppError> {
        self.provider_invoke(id, "settings_describe", serde_json::json!({}))
            .await
    }

    pub async fn provider_settings_get(&self, id: &str) -> Result<ProviderSettingsState, AppError> {
        self.provider_invoke(id, "settings_get", serde_json::json!({}))
            .await
    }

    pub async fn provider_settings_update(
        &self,
        id: &str,
        state: ProviderSettingsState,
    ) -> Result<ProviderSettingsState, AppError> {
        let params = serde_json::to_value(state).map_err(AppError::internal)?;
        self.provider_invoke(id, "settings_update", params).await
    }

    pub async fn provider_settings_invoke(
        &self,
        id: &str,
        request: ProviderSettingsActionRequest,
    ) -> Result<ProviderSettingsActionResult, AppError> {
        let params = serde_json::to_value(request).map_err(AppError::internal)?;
        self.provider_invoke(id, "settings_invoke", params).await
    }

    async fn provider_invoke<T: serde::de::DeserializeOwned>(
        &self,
        id: &str,
        method: &str,
        params: serde_json::Value,
    ) -> Result<T, AppError> {
        let value = self.providers.invoke(id, method, params).await?;
        serde_json::from_value(value).map_err(AppError::internal)
    }
}
