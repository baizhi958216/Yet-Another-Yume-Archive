mod input;
mod providers;
mod settings;
mod tasks;

use std::sync::Arc;

use axum::{
    routing::{delete, get, post},
    Router,
};

use crate::{sse, WebState};

pub(crate) fn router() -> Router<Arc<WebState>> {
    Router::new()
        .route("/events", get(sse::task_events))
        .route("/inspect", post(input::inspect_source))
        .route("/tasks", get(tasks::list).post(tasks::create))
        .route("/tasks/{id}/pause", post(tasks::pause))
        .route("/tasks/{id}/resume", post(tasks::resume))
        .route("/tasks/{id}/retry", post(tasks::retry))
        .route("/tasks/{id}/cancel", post(tasks::cancel))
        .route("/tasks/{id}/file", get(crate::files::download_task_file))
        .route("/tasks/{id}", delete(tasks::remove))
        .route("/settings", get(settings::get).put(settings::update))
        .route("/providers", get(providers::list))
        .route("/providers/{id}/enabled", post(providers::set_enabled))
        .route("/providers/{id}/asset", get(input::provider_asset))
        .route("/providers/{id}/ui", get(providers::ui_bundle))
        .route("/providers/{id}/ui/actions", post(providers::ui_invoke))
}
