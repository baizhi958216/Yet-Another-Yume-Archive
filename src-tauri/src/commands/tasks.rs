use tauri::State;
use yaya_app_core::{AppCore, AppError};
use yaya_task_runtime::{CreateTasksRequest, TaskSnapshot};

#[tauri::command]
pub async fn create_tasks(
    core: State<'_, AppCore>,
    request: CreateTasksRequest,
) -> Result<Vec<TaskSnapshot>, AppError> {
    core.create_tasks(request).await
}

#[tauri::command]
pub async fn list_tasks(core: State<'_, AppCore>) -> Result<Vec<TaskSnapshot>, AppError> {
    Ok(core.list_tasks().await)
}

#[tauri::command]
pub async fn pause_task(core: State<'_, AppCore>, id: String) -> Result<(), AppError> {
    core.pause_task(&id).await
}

#[tauri::command]
pub async fn resume_task(core: State<'_, AppCore>, id: String) -> Result<(), AppError> {
    core.resume_task(&id).await
}

#[tauri::command]
pub async fn retry_task(core: State<'_, AppCore>, id: String) -> Result<(), AppError> {
    core.retry_task(&id).await
}

#[tauri::command]
pub async fn cancel_task(core: State<'_, AppCore>, id: String) -> Result<(), AppError> {
    core.cancel_task(&id).await
}

#[tauri::command]
pub async fn delete_task(core: State<'_, AppCore>, id: String) -> Result<(), AppError> {
    core.delete_task(&id).await
}
