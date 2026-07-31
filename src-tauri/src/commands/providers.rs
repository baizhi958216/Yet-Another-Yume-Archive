use tauri::State;
use yaya_app_core::{AppCore, AppError};
use yaya_provider_api::{
    ProviderAuthActionRequest, ProviderAuthPage, ProviderSettingsActionRequest,
    ProviderSettingsActionResult, ProviderSettingsState, ProviderSettingsView,
};
use yaya_provider_host::ProviderInfo;

#[tauri::command]
pub fn list_providers(core: State<'_, AppCore>) -> Vec<ProviderInfo> {
    core.list_providers()
}

#[tauri::command]
pub fn set_provider_enabled(
    core: State<'_, AppCore>,
    id: String,
    enabled: bool,
) -> Result<ProviderInfo, AppError> {
    core.set_provider_enabled(&id, enabled)
}

#[tauri::command]
pub async fn provider_auth_describe(
    core: State<'_, AppCore>,
    provider_id: String,
) -> Result<ProviderAuthPage, AppError> {
    core.provider_auth_describe(&provider_id).await
}

#[tauri::command]
pub async fn provider_auth_invoke(
    core: State<'_, AppCore>,
    provider_id: String,
    request: ProviderAuthActionRequest,
) -> Result<serde_json::Value, AppError> {
    core.provider_auth_invoke(&provider_id, request).await
}

#[tauri::command]
pub async fn provider_settings_describe(
    core: State<'_, AppCore>,
    provider_id: String,
) -> Result<ProviderSettingsView, AppError> {
    core.provider_settings_describe(&provider_id).await
}

#[tauri::command]
pub async fn provider_settings_get(
    core: State<'_, AppCore>,
    provider_id: String,
) -> Result<ProviderSettingsState, AppError> {
    core.provider_settings_get(&provider_id).await
}

#[tauri::command]
pub async fn provider_settings_update(
    core: State<'_, AppCore>,
    provider_id: String,
    state: ProviderSettingsState,
) -> Result<ProviderSettingsState, AppError> {
    core.provider_settings_update(&provider_id, state).await
}

#[tauri::command]
pub async fn provider_settings_invoke(
    core: State<'_, AppCore>,
    provider_id: String,
    request: ProviderSettingsActionRequest,
) -> Result<ProviderSettingsActionResult, AppError> {
    core.provider_settings_invoke(&provider_id, request).await
}
