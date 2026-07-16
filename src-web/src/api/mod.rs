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
        .route(
            "/providers/{id}/auth/qr/start",
            post(providers::auth_qr_start),
        )
        .route(
            "/providers/{id}/auth/qr/poll",
            post(providers::auth_qr_poll),
        )
        .route("/providers/{id}/auth/status", get(providers::auth_status))
        .route("/providers/{id}/auth/logout", post(providers::auth_logout))
}
