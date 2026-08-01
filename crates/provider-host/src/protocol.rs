use serde::{Deserialize, Serialize};
use yaya_provider_api::ProviderError;

pub const PROVIDER_PROTOCOL_VERSION: u32 = 2;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Request<P> {
    pub(crate) protocol_version: u32,
    pub(crate) method: String,
    pub(crate) params: P,
}

#[derive(Deserialize)]
pub(crate) struct Response<T> {
    pub(crate) result: Option<T>,
    pub(crate) error: Option<ProviderError>,
}
