use yaya_task_runtime::{CreateTasksRequest, TaskSnapshot};

use crate::{AppCore, AppError};

impl AppCore {
    /// Create queued tasks; an empty output dir falls back to the default and
    /// a `group` title becomes a sub-directory keeping the batch together.
    pub async fn create_tasks(
        &self,
        mut request: CreateTasksRequest,
    ) -> Result<Vec<TaskSnapshot>, AppError> {
        if request.output_dir.as_os_str().is_empty() {
            request.output_dir = self.get_settings().await?.default_output_dir;
        }
        if let Some(name) = request.group.as_deref().and_then(dir_name) {
            request.output_dir = request.output_dir.join(name);
        }
        Ok(self.runtime.create_tasks(request).await?)
    }

    pub async fn list_tasks(&self) -> Vec<TaskSnapshot> {
        self.runtime.list_tasks().await
    }

    pub async fn get_task(&self, id: &str) -> Result<TaskSnapshot, AppError> {
        Ok(self.runtime.task(id).await?)
    }

    pub async fn pause_task(&self, id: &str) -> Result<(), AppError> {
        Ok(self.runtime.pause_task(id).await?)
    }

    pub async fn resume_task(&self, id: &str) -> Result<(), AppError> {
        Ok(self.runtime.resume_task(id).await?)
    }

    pub async fn retry_task(&self, id: &str) -> Result<(), AppError> {
        Ok(self.runtime.retry_task(id).await?)
    }

    pub async fn cancel_task(&self, id: &str) -> Result<(), AppError> {
        Ok(self.runtime.cancel_task(id).await?)
    }

    pub async fn delete_task(&self, id: &str) -> Result<(), AppError> {
        Ok(self.runtime.delete_task(id).await?)
    }
}

/// Reduce a provider-supplied title to a safe single directory name.
fn dir_name(value: &str) -> Option<String> {
    let cleaned = value
        .chars()
        .map(|char| match char {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => ' ',
            char if char.is_control() => ' ',
            char => char,
        })
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    let cleaned = cleaned
        .trim_matches(|char: char| char == '.' || char.is_whitespace())
        .chars()
        .take(80)
        .collect::<String>();
    (!cleaned.is_empty()).then_some(cleaned)
}
