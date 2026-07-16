use axum::{http::StatusCode, response::IntoResponse, Json};
use yaya_app_core::AppError;

/// Newtype so we can implement `IntoResponse` for the shared error.
pub(crate) struct ApiError(pub AppError);

impl<E: Into<AppError>> From<E> for ApiError {
    fn from(error: E) -> Self {
        Self(error.into())
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let status = match self.0.code.as_str() {
            "not_found" => StatusCode::NOT_FOUND,
            "auth_required" => StatusCode::UNAUTHORIZED,
            _ => StatusCode::BAD_REQUEST,
        };
        (status, Json(self.0)).into_response()
    }
}
