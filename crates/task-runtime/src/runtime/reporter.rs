//! Bridges provider progress callbacks into task snapshot updates + events.

use yaya_provider_api::{ProgressReporter, TaskProgress};

use super::{now_millis, TaskRuntime};
use crate::RuntimeError;

pub(super) struct RuntimeReporter {
    pub runtime: TaskRuntime,
    pub id: String,
}

impl ProgressReporter for RuntimeReporter {
    fn report(&self, progress: TaskProgress) {
        let runtime = self.runtime.clone();
        let id = self.id.clone();
        tokio::spawn(async move {
            let _ = runtime.update_progress(&id, progress).await;
        });
    }
}

impl TaskRuntime {
    pub(super) async fn update_progress(
        &self,
        id: &str,
        progress: TaskProgress,
    ) -> Result<(), RuntimeError> {
        let mut tasks = self.inner.tasks.write().await;
        let task = tasks
            .get_mut(id)
            .ok_or_else(|| RuntimeError::NotFound(id.into()))?;
        task.completed = progress.completed;
        task.total = progress.total;
        task.rate = progress.rate;
        task.message = progress.message;
        task.updated_at = now_millis();
        self.inner.storage.upsert_task(task)?;
        self.emit(task.clone());
        Ok(())
    }
}
