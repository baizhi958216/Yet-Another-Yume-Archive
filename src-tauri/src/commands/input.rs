use tauri::State;
use yaya_app_core::{AppCore, AppError};
use yaya_provider_api::{BinaryAsset, ProviderView};

#[tauri::command]
pub async fn inspect_source(
    core: State<'_, AppCore>,
    source: String,
) -> Result<ProviderView, AppError> {
    core.inspect_source(source).await
}

#[tauri::command]
pub async fn fetch_provider_asset(
    core: State<'_, AppCore>,
    provider_id: String,
    url: String,
) -> Result<BinaryAsset, AppError> {
    core.fetch_provider_asset(&provider_id, &url).await
}
