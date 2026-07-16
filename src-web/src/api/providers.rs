use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use yaya_app_core::{AuthQrPoll, AuthQrSession, AuthStatus};
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

pub(crate) async fn auth_qr_start(
    State(state): State<Arc<WebState>>,
    Path(id): Path<String>,
) -> Result<Json<AuthQrSession>, ApiError> {
    Ok(Json(state.core.provider_auth_qr_start(&id).await?))
}

#[derive(Deserialize)]
pub(crate) struct QrPollRequest {
    key: String,
}

pub(crate) async fn auth_qr_poll(
    State(state): State<Arc<WebState>>,
    Path(id): Path<String>,
    Json(request): Json<QrPollRequest>,
) -> Result<Json<AuthQrPoll>, ApiError> {
    Ok(Json(
        state.core.provider_auth_qr_poll(&id, &request.key).await?,
    ))
}

pub(crate) async fn auth_status(
    State(state): State<Arc<WebState>>,
    Path(id): Path<String>,
) -> Result<Json<AuthStatus>, ApiError> {
    Ok(Json(state.core.provider_auth_status(&id).await?))
}

pub(crate) async fn auth_logout(
    State(state): State<Arc<WebState>>,
    Path(id): Path<String>,
) -> Result<StatusCode, ApiError> {
    state.core.provider_auth_logout(&id).await?;
    Ok(StatusCode::NO_CONTENT)
}
