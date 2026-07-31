use std::sync::Arc;

use axum::{
    extract::{Path, State},
    Json,
};
use serde::Deserialize;
use yaya_provider_api::{
    ProviderAuthActionRequest, ProviderAuthPage, ProviderSettingsActionRequest,
    ProviderSettingsActionResult, ProviderSettingsState, ProviderSettingsView,
};
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

pub(crate) async fn auth_describe(
    State(state): State<Arc<WebState>>,
    Path(id): Path<String>,
) -> Result<Json<ProviderAuthPage>, ApiError> {
    Ok(Json(state.core.provider_auth_describe(&id).await?))
}

pub(crate) async fn auth_invoke(
    State(state): State<Arc<WebState>>,
    Path(id): Path<String>,
    Json(request): Json<ProviderAuthActionRequest>,
) -> Result<Json<serde_json::Value>, ApiError> {
    Ok(Json(state.core.provider_auth_invoke(&id, request).await?))
}

pub(crate) async fn settings_describe(
    State(state): State<Arc<WebState>>,
    Path(id): Path<String>,
) -> Result<Json<ProviderSettingsView>, ApiError> {
    Ok(Json(state.core.provider_settings_describe(&id).await?))
}

pub(crate) async fn settings_get(
    State(state): State<Arc<WebState>>,
    Path(id): Path<String>,
) -> Result<Json<ProviderSettingsState>, ApiError> {
    Ok(Json(state.core.provider_settings_get(&id).await?))
}

pub(crate) async fn settings_update(
    State(state): State<Arc<WebState>>,
    Path(id): Path<String>,
    Json(settings): Json<ProviderSettingsState>,
) -> Result<Json<ProviderSettingsState>, ApiError> {
    Ok(Json(
        state.core.provider_settings_update(&id, settings).await?,
    ))
}

pub(crate) async fn settings_invoke(
    State(state): State<Arc<WebState>>,
    Path(id): Path<String>,
    Json(request): Json<ProviderSettingsActionRequest>,
) -> Result<Json<ProviderSettingsActionResult>, ApiError> {
    Ok(Json(
        state.core.provider_settings_invoke(&id, request).await?,
    ))
}
