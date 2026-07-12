use yaya_task_runtime::RuntimeSettings;

use crate::{
    settings::{load, save, StoredSettings},
    AppCore, AppError, AppSettings,
};

impl AppCore {
    pub async fn get_settings(&self) -> Result<AppSettings, AppError> {
        let stored = load(&self.settings_path);
        let runtime = self.runtime.settings().await;
        Ok(AppSettings {
            default_output_dir: stored
                .default_output_dir
                .unwrap_or_else(|| self.paths.default_output_dir.clone()),
            max_active_tasks: runtime.max_active_tasks,
        })
    }

    pub async fn update_settings(&self, settings: AppSettings) -> Result<AppSettings, AppError> {
        self.runtime
            .update_settings(RuntimeSettings {
                max_active_tasks: settings.max_active_tasks,
            })
            .await?;
        save(
            &self.settings_path,
            &StoredSettings {
                default_output_dir: Some(settings.default_output_dir),
            },
        )?;
        self.get_settings().await
    }
}
