use tauri::State;
use yaya_app_core::{AppCore, AppError, AppSettings};

#[cfg(target_os = "android")]
use tauri::AppHandle;

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

#[cfg(target_os = "android")]
#[tauri::command]
pub fn get_android_download_directory(
    app: AppHandle<tauri::Wry>,
) -> Result<crate::android_download::DownloadDirectory, String> {
    crate::android_download::get_directory(&app)
}

#[cfg(not(target_os = "android"))]
#[tauri::command]
pub fn get_android_download_directory() -> Result<(), String> {
    Err("Android download directories are only available on Android".into())
}

#[cfg(target_os = "android")]
#[tauri::command]
pub fn pick_android_download_directory(
    app: AppHandle<tauri::Wry>,
) -> Result<crate::android_download::DownloadDirectory, String> {
    crate::android_download::pick_directory(&app)
}

#[cfg(not(target_os = "android"))]
#[tauri::command]
pub fn pick_android_download_directory() -> Result<(), String> {
    Err("Android download directories are only available on Android".into())
}
