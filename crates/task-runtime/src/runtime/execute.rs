//! Runs one task through its provider, with cancellation registered so
//! pause/cancel can interrupt it.

use std::sync::Arc;

use tokio_util::sync::CancellationToken;
use yaya_provider_api::ProviderTaskRequest;

use super::{now_millis, reporter::RuntimeReporter, TaskRuntime};
use crate::{RuntimeError, TaskStatus};

impl TaskRuntime {
    pub(super) async fn run_task(&self, id: String) {
        let result = self.execute_provider_task(&id).await;
        self.inner.cancellations.lock().await.remove(&id);
        if let Err(error) = result {
            // pause/cancel already transitioned the task; only a task still
            // marked running actually failed
            let current = self
                .inner
                .tasks
                .read()
                .await
                .get(&id)
                .map(|task| task.status);
            if current == Some(TaskStatus::Running) {
                let _ = self
                    .transition(&id, TaskStatus::Failed, Some(error.to_string()))
                    .await;
            }
        }
    }

    async fn execute_provider_task(&self, id: &str) -> Result<(), RuntimeError> {
        let task = self.task(id).await?;
        let provider = self
            .inner
            .providers
            .provider(&task.provider)
            .ok_or_else(|| RuntimeError::ProviderUnavailable(task.provider.clone()))?;
        let cancellation = CancellationToken::new();
        self.inner
            .cancellations
            .lock()
            .await
            .insert(id.to_string(), cancellation.clone());
        let work_dir = task.output_dir.join(".yaya").join(id);
        tokio::fs::create_dir_all(&work_dir).await?;
        let request = ProviderTaskRequest {
            id: task.id.clone(),
            source: task.source.clone(),
            task: task.draft.clone(),
            options: task.options.clone(),
            output_dir: task.output_dir.clone(),
            work_dir: work_dir.clone(),
            settings: Default::default(),
        };
        let reporter = Arc::new(RuntimeReporter {
            runtime: self.clone(),
            id: id.to_string(),
        });
        let artifacts = provider.run(request, reporter, cancellation).await?;
        {
            let mut tasks = self.inner.tasks.write().await;
            let task = tasks
                .get_mut(id)
                .ok_or_else(|| RuntimeError::NotFound(id.into()))?;
            task.artifacts = artifacts;
            task.updated_at = now_millis();
            self.inner.storage.upsert_task(task)?;
            self.emit(task.clone());
        }
        self.transition(id, TaskStatus::Completed, None).await?;
        tokio::fs::remove_dir_all(&work_dir).await.ok();
        Ok(())
    }
}
