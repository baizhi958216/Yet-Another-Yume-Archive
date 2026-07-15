use tauri::State;
use yaya_app_core::{AppCore, AppError, AppSettings};

#[tauri::command]
pub async fn get_settings(core: State<'_, AppCore>) -> Result<AppSettings, AppError> {
    core.get_settings().await
}

#[tauri::command]
pub async fn update_settings(
    core: State<'_, AppCore>,
    settings: AppSettings,
) -> Result<AppSettings, AppError> {
    core.update_settings(settings).await
}
