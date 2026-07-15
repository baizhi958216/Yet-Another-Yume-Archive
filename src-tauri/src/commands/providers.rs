use tauri::State;
use yaya_app_core::{AppCore, AppError, AuthQrPoll, AuthQrSession, AuthStatus};
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
pub async fn provider_auth_qr_start(
    core: State<'_, AppCore>,
    provider_id: String,
) -> Result<AuthQrSession, AppError> {
    core.provider_auth_qr_start(&provider_id).await
}

#[tauri::command]
pub async fn provider_auth_qr_poll(
    core: State<'_, AppCore>,
    provider_id: String,
    key: String,
) -> Result<AuthQrPoll, AppError> {
    core.provider_auth_qr_poll(&provider_id, &key).await
}

#[tauri::command]
pub async fn provider_auth_status(
    core: State<'_, AppCore>,
    provider_id: String,
) -> Result<AuthStatus, AppError> {
    core.provider_auth_status(&provider_id).await
}

#[tauri::command]
pub async fn provider_auth_logout(
    core: State<'_, AppCore>,
    provider_id: String,
) -> Result<(), AppError> {
    core.provider_auth_logout(&provider_id).await
}
