use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    Json,
};
use serde::Deserialize;
use yaya_provider_api::{BinaryAsset, ProviderView};

use crate::{error::ApiError, WebState};

#[derive(Deserialize)]
pub(crate) struct ResolveRequest {
    source: String,
}

pub(crate) async fn inspect_source(
    State(state): State<Arc<WebState>>,
    Json(request): Json<ResolveRequest>,
) -> Result<Json<ProviderView>, ApiError> {
    Ok(Json(state.core.inspect_source(request.source).await?))
}

#[derive(Deserialize)]
pub(crate) struct AssetQuery {
    url: String,
}

pub(crate) async fn provider_asset(
    State(state): State<Arc<WebState>>,
    Path(id): Path<String>,
    Query(query): Query<AssetQuery>,
) -> Result<Json<BinaryAsset>, ApiError> {
    Ok(Json(
        state.core.fetch_provider_asset(&id, &query.url).await?,
    ))
}
