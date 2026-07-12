//! User-facing task operations: create, pause, resume, retry, cancel, delete.

use std::sync::atomic::Ordering;

use crate::{CreateTasksRequest, RuntimeError, TaskEvent, TaskSnapshot, TaskStatus};

use super::{now_millis, TaskRuntime};

impl TaskRuntime {
    pub async fn create_tasks(
        &self,
        request: CreateTasksRequest,
    ) -> Result<Vec<TaskSnapshot>, RuntimeError> {
        if self.inner.providers.provider(&request.provider).is_none() {
            return Err(RuntimeError::ProviderUnavailable(request.provider));
        }
        let now = now_millis();
        let mut created = Vec::new();
        for value in request.tasks {
            let task = TaskSnapshot {
                id: uuid::Uuid::new_v4().to_string(),
                provider: request.provider.clone(),
                batch_id: request.batch_id.clone(),
                group: request.group.clone(),
                source: request.source.clone(),
                draft: value.draft,
                options: value.options,
                output_dir: request.output_dir.clone(),
                status: TaskStatus::Queued,
                completed: 0,
                total: None,
                rate: 0,
                message: String::new(),
                error: None,
                warnings: Vec::new(),
                artifacts: Vec::new(),
                created_at: now,
                updated_at: now,
            };
            self.inner.storage.upsert_task(&task)?;
            self.inner
                .tasks
                .write()
                .await
                .insert(task.id.clone(), task.clone());
            self.emit(task.clone());
            created.push(task);
        }
        self.kick();
        Ok(created)
    }

    /// Pause = cancel the running provider; resume correctness relies on the
    /// provider's own on-disk resume state.
    pub async fn pause_task(&self, id: &str) -> Result<(), RuntimeError> {
        if let Some(token) = self.inner.cancellations.lock().await.get(id) {
            token.cancel();
        }
        self.transition(id, TaskStatus::Paused, None).await
    }

    pub async fn resume_task(&self, id: &str) -> Result<(), RuntimeError> {
        self.transition(id, TaskStatus::Queued, None).await?;
        self.kick();
        Ok(())
    }

    pub async fn retry_task(&self, id: &str) -> Result<(), RuntimeError> {
        self.resume_task(id).await
    }

    pub async fn cancel_task(&self, id: &str) -> Result<(), RuntimeError> {
        if let Some(token) = self.inner.cancellations.lock().await.get(id) {
            token.cancel();
        }
        self.transition(id, TaskStatus::Canceled, None).await?;
        self.remove_work_dir(id).await;
        Ok(())
    }

    pub async fn delete_task(&self, id: &str) -> Result<(), RuntimeError> {
        let task = self.task(id).await?;
        if !task.status.is_terminal() && task.status != TaskStatus::Failed {
            self.cancel_task(id).await?;
        }
        self.remove_work_dir(id).await;
        self.inner.tasks.write().await.remove(id);
        self.inner.storage.delete_task(id)
    }

    async fn remove_work_dir(&self, id: &str) {
        if let Some(task) = self.inner.tasks.read().await.get(id) {
            tokio::fs::remove_dir_all(task.output_dir.join(".yaya").join(id))
                .await
                .ok();
        }
    }

    pub(super) async fn transition(
        &self,
        id: &str,
        status: TaskStatus,
        error: Option<String>,
    ) -> Result<(), RuntimeError> {
        let mut tasks = self.inner.tasks.write().await;
        let task = tasks
            .get_mut(id)
            .ok_or_else(|| RuntimeError::NotFound(id.into()))?;
        if !task.status.can_transition_to(status) {
            return Err(RuntimeError::InvalidTransition {
                from: task.status,
                to: status,
            });
        }
        task.status = status;
        task.error = error;
        task.updated_at = now_millis();
        self.inner.storage.upsert_task(task)?;
        self.emit(task.clone());
        Ok(())
    }

    pub(super) fn emit(&self, task: TaskSnapshot) {
        let sequence = self.inner.sequence.fetch_add(1, Ordering::Relaxed) + 1;
        let _ = self.inner.events.send(TaskEvent { sequence, task });
    }
}
