use std::path::PathBuf;

use tokio::sync::broadcast;
use yaya_provider_api::ProviderRegistry;
use yaya_provider_host::{scan_roots, HostedProvider, ProviderManager};
use yaya_task_runtime::{TaskEvent, TaskRuntime};

use crate::AppError;

/// Filesystem locations the host resolves before opening the core.
#[derive(Debug, Clone)]
pub struct AppPaths {
    pub data_dir: PathBuf,
    pub default_output_dir: PathBuf,
}

#[derive(Clone)]
pub struct AppCore {
    pub(crate) runtime: TaskRuntime,
    pub(crate) providers: ProviderManager,
    pub(crate) paths: AppPaths,
    pub(crate) settings_path: PathBuf,
}

impl AppCore {
    /// Assemble registry + providers + runtime.
    ///
    /// `builtin` comes from the host's provider bundle (statically linked);
    /// external providers are discovered from `<data_dir>/providers`, the
    /// working directory's `providers/` (dev convenience) and
    /// `$YAYA_PROVIDERS_DIR`. External packages win on id conflicts so a
    /// user-installed provider can override a bundled one.
    pub async fn open(paths: AppPaths, builtin: Vec<HostedProvider>) -> Result<Self, AppError> {
        std::fs::create_dir_all(&paths.data_dir)?;
        std::fs::create_dir_all(&paths.default_output_dir)?;

        let mut discovered = builtin;
        for provider in scan_roots(external_roots(&paths))? {
            discovered.retain(|value| value.id() != provider.id());
            discovered.push(provider);
        }

        let registry = ProviderRegistry::new();
        let providers = ProviderManager::open(
            discovered,
            paths.data_dir.join("providers.json"),
            registry.clone(),
        )?;
        let runtime = TaskRuntime::open(&paths.data_dir.join("yaya.db"), registry).await?;
        let settings_path = paths.data_dir.join("settings.json");
        Ok(Self {
            runtime,
            providers,
            paths,
            settings_path,
        })
    }

    pub fn subscribe(&self) -> broadcast::Receiver<TaskEvent> {
        self.runtime.subscribe()
    }

    pub fn runtime(&self) -> &TaskRuntime {
        &self.runtime
    }
}

fn external_roots(paths: &AppPaths) -> Vec<PathBuf> {
    let mut roots = vec![paths.data_dir.join("providers")];
    if let Ok(cwd) = std::env::current_dir() {
        roots.push(cwd.join("providers"));
    }
    if let Some(extra) = std::env::var_os("YAYA_PROVIDERS_DIR") {
        roots.push(PathBuf::from(extra));
    }
    roots
}
