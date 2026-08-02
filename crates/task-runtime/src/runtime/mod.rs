mod control;
mod execute;
mod reporter;
mod scheduler;

use std::{
    collections::HashMap,
    path::Path,
    sync::{
        atomic::{AtomicBool, AtomicU64, AtomicUsize},
        Arc,
    },
};

use tokio::sync::{broadcast, Mutex, RwLock};
use tokio_util::sync::CancellationToken;
use yaya_provider_api::{BinaryAsset, ProviderRegistry, ProviderView};

use crate::{
    storage::Storage, ArtifactPublisher, RuntimeError, RuntimeSettings, TaskEvent, TaskSnapshot,
    TaskStatus,
};

#[derive(Clone)]
pub struct TaskRuntime {
    pub(super) inner: Arc<Inner>,
}

pub(super) struct Inner {
    pub providers: ProviderRegistry,
    pub storage: Storage,
    pub tasks: RwLock<HashMap<String, TaskSnapshot>>,
    pub cancellations: Mutex<HashMap<String, CancellationToken>>,
    pub settings: RwLock<RuntimeSettings>,
    pub publisher: RwLock<Option<Arc<dyn ArtifactPublisher>>>,
    pub sequence: AtomicU64,
    pub active: AtomicUsize,
    pub scheduling: AtomicBool,
    pub events: broadcast::Sender<TaskEvent>,
}

impl TaskRuntime {
    pub async fn open(
        database_path: &Path,
        providers: ProviderRegistry,
    ) -> Result<Self, RuntimeError> {
        let storage = Storage::open(database_path)?;
        let settings = storage.load_settings()?;
        let mut tasks = HashMap::new();
        for mut task in storage.load_tasks()? {
            // a task that was running when the host died can't still be
            // running; demote to paused so the user can resume it
            if task.status.is_running() {
                task.status = TaskStatus::Paused;
                task.error = None;
                task.updated_at = now_millis();
                storage.upsert_task(&task)?;
            }
            tasks.insert(task.id.clone(), task);
        }
        let (events, _) = broadcast::channel(256);
        let runtime = Self {
            inner: Arc::new(Inner {
                providers,
                storage,
                tasks: RwLock::new(tasks),
                cancellations: Mutex::new(HashMap::new()),
                settings: RwLock::new(settings),
                publisher: RwLock::new(None),
                sequence: AtomicU64::new(0),
                active: AtomicUsize::new(0),
                scheduling: AtomicBool::new(false),
                events,
            }),
        };
        runtime.kick();
        Ok(runtime)
    }

    pub fn subscribe(&self) -> broadcast::Receiver<TaskEvent> {
        self.inner.events.subscribe()
    }

    pub async fn inspect(&self, source: String) -> Result<ProviderView, RuntimeError> {
        Ok(self.inner.providers.inspect(source.into()).await?)
    }

    pub async fn fetch_provider_asset(
        &self,
        provider_id: &str,
        url: &str,
    ) -> Result<BinaryAsset, RuntimeError> {
        let provider = self
            .inner
            .providers
            .provider(provider_id)
            .ok_or_else(|| RuntimeError::ProviderUnavailable(provider_id.into()))?;
        Ok(provider.fetch_asset(url).await?)
    }

    pub async fn list_tasks(&self) -> Vec<TaskSnapshot> {
        let mut values = self
            .inner
            .tasks
            .read()
            .await
            .values()
            .cloned()
            .collect::<Vec<_>>();
        values.sort_by_key(|value| std::cmp::Reverse(value.created_at));
        values
    }

    pub async fn task(&self, id: &str) -> Result<TaskSnapshot, RuntimeError> {
        self.inner
            .tasks
            .read()
            .await
            .get(id)
            .cloned()
            .ok_or_else(|| RuntimeError::NotFound(id.into()))
    }

    pub async fn settings(&self) -> RuntimeSettings {
        self.inner.settings.read().await.clone()
    }

    pub async fn update_settings(&self, mut settings: RuntimeSettings) -> Result<(), RuntimeError> {
        settings.max_active_tasks = settings.max_active_tasks.clamp(1, 10);
        self.inner.storage.save_settings(&settings)?;
        *self.inner.settings.write().await = settings;
        self.kick();
        Ok(())
    }

    pub async fn set_artifact_publisher(&self, publisher: Arc<dyn ArtifactPublisher>) {
        *self.inner.publisher.write().await = Some(publisher);
    }
}

pub(super) fn now_millis() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as i64
}
