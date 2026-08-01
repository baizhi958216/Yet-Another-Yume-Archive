use std::sync::Arc;

use axum::{
    extract::{Path, State},
    Json,
};
use serde::Deserialize;
use yaya_provider_api::{ProviderUiActionRequest, ProviderUiBundle};
use yaya_provider_host::ProviderInfo;

use crate::{error::ApiError, WebState};

pub(crate) async fn list(State(state): State<Arc<WebState>>) -> Json<Vec<ProviderInfo>> {
    Json(state.core.list_providers())
}

#[derive(Deserialize)]
pub(crate) struct EnabledRequest {
    enabled: bool,
}

pub(crate) async fn set_enabled(
    State(state): State<Arc<WebState>>,
    Path(id): Path<String>,
    Json(request): Json<EnabledRequest>,
) -> Result<Json<ProviderInfo>, ApiError> {
    Ok(Json(state.core.set_provider_enabled(&id, request.enabled)?))
}

pub(crate) async fn ui_bundle(
    State(state): State<Arc<WebState>>,
    Path(id): Path<String>,
) -> Result<Json<ProviderUiBundle>, ApiError> {
    Ok(Json(state.core.provider_ui_bundle(&id)?))
}

pub(crate) async fn ui_invoke(
    State(state): State<Arc<WebState>>,
    Path(id): Path<String>,
    Json(request): Json<ProviderUiActionRequest>,
) -> Result<Json<serde_json::Value>, ApiError> {
    Ok(Json(state.core.provider_ui_invoke(&id, request).await?))
}
