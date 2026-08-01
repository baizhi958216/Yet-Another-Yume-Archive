use tauri::State;
use yaya_app_core::{AppCore, AppError};
use yaya_provider_api::{ProviderUiActionRequest, ProviderUiBundle};
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
pub fn provider_ui_bundle(
    core: State<'_, AppCore>,
    provider_id: String,
) -> Result<ProviderUiBundle, AppError> {
    core.provider_ui_bundle(&provider_id)
}

#[tauri::command]
pub async fn provider_ui_invoke(
    core: State<'_, AppCore>,
    provider_id: String,
    request: ProviderUiActionRequest,
) -> Result<serde_json::Value, AppError> {
    core.provider_ui_invoke(&provider_id, request).await
}
