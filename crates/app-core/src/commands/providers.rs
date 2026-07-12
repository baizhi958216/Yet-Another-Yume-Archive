//! Provider management and the auth control-plane passthrough. Only the
//! closed set of auth methods from the protocol doc is forwarded.

use serde::{Deserialize, Serialize};
use yaya_provider_host::ProviderInfo;

use crate::{AppCore, AppError};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthQrSession {
    pub key: String,
    pub url: String,
    #[serde(default)]
    pub expires_in_sec: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthQrPoll {
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthStatus {
    pub logged_in: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<serde_json::Value>,
}

impl AppCore {
    pub fn list_providers(&self) -> Vec<ProviderInfo> {
        self.providers.list()
    }

    pub fn set_provider_enabled(&self, id: &str, enabled: bool) -> Result<ProviderInfo, AppError> {
        Ok(self.providers.set_enabled(id, enabled)?)
    }

    pub async fn provider_auth_qr_start(&self, id: &str) -> Result<AuthQrSession, AppError> {
        self.auth_invoke(id, "auth_qr_start", serde_json::json!({}))
            .await
    }

    pub async fn provider_auth_qr_poll(&self, id: &str, key: &str) -> Result<AuthQrPoll, AppError> {
        self.auth_invoke(id, "auth_qr_poll", serde_json::json!({ "key": key }))
            .await
    }

    pub async fn provider_auth_status(&self, id: &str) -> Result<AuthStatus, AppError> {
        self.auth_invoke(id, "auth_status", serde_json::json!({}))
            .await
    }

    pub async fn provider_auth_logout(&self, id: &str) -> Result<(), AppError> {
        let _: serde_json::Value = self
            .auth_invoke(id, "auth_logout", serde_json::json!({}))
            .await?;
        Ok(())
    }

    async fn auth_invoke<T: serde::de::DeserializeOwned>(
        &self,
        id: &str,
        method: &str,
        params: serde_json::Value,
    ) -> Result<T, AppError> {
        let value = self.providers.invoke(id, method, params).await?;
        serde_json::from_value(value).map_err(AppError::internal)
    }
}
