use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use yaya_task_runtime::{CreateTasksRequest, TaskSnapshot};

use crate::{error::ApiError, WebState};

pub(crate) async fn create(
    State(state): State<Arc<WebState>>,
    Json(mut request): Json<CreateTasksRequest>,
) -> Result<Json<Vec<TaskSnapshot>>, ApiError> {
    // each browser request gets an isolated temp dir, cleaned up after serving
    request.output_dir = state.web_output_dir.join(uuid::Uuid::new_v4().to_string());
    Ok(Json(state.core.create_tasks(request).await?))
}

pub(crate) async fn list(State(state): State<Arc<WebState>>) -> Json<Vec<TaskSnapshot>> {
    Json(state.core.list_tasks().await)
}

type ActionResult = Result<StatusCode, ApiError>;

pub(crate) async fn pause(State(s): State<Arc<WebState>>, Path(id): Path<String>) -> ActionResult {
    s.core.pause_task(&id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub(crate) async fn resume(State(s): State<Arc<WebState>>, Path(id): Path<String>) -> ActionResult {
    s.core.resume_task(&id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub(crate) async fn retry(State(s): State<Arc<WebState>>, Path(id): Path<String>) -> ActionResult {
    s.core.retry_task(&id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub(crate) async fn cancel(State(s): State<Arc<WebState>>, Path(id): Path<String>) -> ActionResult {
    s.core.cancel_task(&id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub(crate) async fn remove(State(s): State<Arc<WebState>>, Path(id): Path<String>) -> ActionResult {
    s.core.delete_task(&id).await?;
    Ok(StatusCode::NO_CONTENT)
}
