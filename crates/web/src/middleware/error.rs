use axum::{body::Body, http::Response, response::IntoResponse};
use serde_json::json;
use tower_governor::GovernorError;

use crate::traits::WebError;

pub async fn validation_error(err: validator::ValidationError) -> impl IntoResponse {}

pub async fn box_error(err: axum::BoxError) -> WebError {
    WebError::InternalServerError(json!(format!("{:?}", err)))
}

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
