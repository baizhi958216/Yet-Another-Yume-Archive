//! Tauri host for desktop and mobile. Assembles the shared AppCore and
//! bridges runtime events to the webview as `task://event`.

mod commands;

use tauri::{Emitter, Manager};
use yaya_app_core::{AppCore, AppPaths};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let data_dir = app.path().app_data_dir()?;
            // iOS forbids creating directories at the container root (where
            // `download_dir()` points); Documents is writable and, with file
            // sharing enabled, visible in the Files app
            #[cfg(target_os = "ios")]
            let default_output_dir = app
                .path()
                .document_dir()
                .unwrap_or_else(|_| data_dir.join("downloads"))
                .join("YAYA");
            #[cfg(not(target_os = "ios"))]
            let default_output_dir = app
                .path()
                .download_dir()
                .unwrap_or_else(|_| data_dir.join("downloads"))
                .join("YAYA");
            let core = tauri::async_runtime::block_on(AppCore::open(
                AppPaths {
                    data_dir,
                    default_output_dir,
                },
                yaya_provider_bundle::providers(),
            ))?;

            let mut events = core.subscribe();
            let handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                while let Ok(event) = events.recv().await {
                    let _ = handle.emit("task://event", event);
                }
            });
            app.manage(core);

            // the window starts hidden and the frontend reveals it once
            // mounted; if the frontend never gets there, show it anyway so
            // the user is not left with an invisible app
            if let Some(window) = app.get_webview_window("main") {
                std::thread::spawn(move || {
                    std::thread::sleep(std::time::Duration::from_secs(10));
                    if !window.is_visible().unwrap_or(true) {
                        let _ = window.show();
                    }
                });
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::input::inspect_source,
            commands::input::fetch_provider_asset,
            commands::tasks::create_tasks,
            commands::tasks::list_tasks,
            commands::tasks::pause_task,
            commands::tasks::resume_task,
            commands::tasks::retry_task,
            commands::tasks::cancel_task,
            commands::tasks::delete_task,
            commands::settings::get_settings,
            commands::settings::update_settings,
            commands::providers::list_providers,
            commands::providers::set_provider_enabled,
            commands::providers::provider_ui_bundle,
            commands::providers::provider_ui_invoke,
        ])
        .run(tauri::generate_context!())
        .expect("error while running yaya");
}
