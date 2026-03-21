//! Small adapters that map framework errors into [`crate::traits::WebError`] or
//! raw responses.

use axum::{body::Body, http::Response, response::IntoResponse};
use serde_json::json;
use tower_governor::GovernorError;

use crate::traits::WebError;

/// Placeholder for validator integration hooks (currently a no-op response).
pub async fn validation_error(_err: validator::ValidationError) -> impl IntoResponse {}

/// Maps unexpected boxed Axum errors to a generic 500 [`WebError`].
pub async fn box_error(err: axum::BoxError) -> WebError {
    WebError::InternalServerError(json!(format!("{:?}", err)))
}

/// Converts `tower_governor` failures into HTTP responses with matching status
/// codes.
pub fn governor_error(err: GovernorError) -> Response<Body> {
    let web_err = match err {
        GovernorError::TooManyRequests {
            wait_time,
            headers: _,
        } => WebError::TooManyRequests(json!(format!("{:?}", wait_time))),
        GovernorError::UnableToExtractKey => WebError::BadRequest(json!(format!("{:?}", err))),
        _ => WebError::InternalServerError(json!(format!("{:?}", err))),
    };

    web_err.into_response()
}
