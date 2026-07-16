use std::sync::Arc;

use axum::{extract::State, Json};
use yaya_app_core::AppSettings;

use crate::{error::ApiError, WebState};

pub(crate) async fn get(State(state): State<Arc<WebState>>) -> Result<Json<AppSettings>, ApiError> {
    Ok(Json(state.core.get_settings().await?))
}

pub(crate) async fn update(
    State(state): State<Arc<WebState>>,
    Json(settings): Json<AppSettings>,
) -> Result<Json<AppSettings>, ApiError> {
    Ok(Json(state.core.update_settings(settings).await?))
}
