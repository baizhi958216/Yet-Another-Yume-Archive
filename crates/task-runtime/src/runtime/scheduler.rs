//! FIFO scheduler: `kick()` is safe to call from anywhere; a CAS guard
//! ensures a single scheduling pass runs at a time.

use std::sync::atomic::Ordering;

use crate::TaskStatus;

use super::TaskRuntime;

impl TaskRuntime {
    pub(super) fn kick(&self) {
        if self
            .inner
            .scheduling
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::Relaxed)
            .is_err()
        {
            return;
        }
        let runtime = self.clone();
        tokio::spawn(async move {
            runtime.schedule().await;
        });
    }

    async fn schedule(&self) {
        loop {
            let limit = self.inner.settings.read().await.max_active_tasks;
            if self.inner.active.load(Ordering::Acquire) >= limit {
                break;
            }
            let next = self
                .inner
                .tasks
                .read()
                .await
                .values()
                .filter(|value| value.status == TaskStatus::Queued)
                .min_by_key(|value| value.created_at)
                .map(|value| value.id.clone());
            let Some(id) = next else { break };
            if self
                .transition(&id, TaskStatus::Running, None)
                .await
                .is_err()
            {
                continue;
            }
            self.inner.active.fetch_add(1, Ordering::AcqRel);
            let runtime = self.clone();
            tokio::spawn(async move {
                runtime.run_task(id).await;
                runtime.inner.active.fetch_sub(1, Ordering::AcqRel);
                runtime.kick();
            });
        }
        self.inner.scheduling.store(false, Ordering::Release);
        // re-kick if capacity appeared while we were winding down
        let limit = self.inner.settings.read().await.max_active_tasks;
        let has_capacity = self.inner.active.load(Ordering::Acquire) < limit;
        let has_queued = self
            .inner
            .tasks
            .read()
            .await
            .values()
            .any(|value| value.status == TaskStatus::Queued);
        if has_capacity && has_queued {
            self.kick();
        }
    }
}
