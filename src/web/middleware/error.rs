use axum::response::IntoResponse;

use crate::web::traits::WebError;

pub async fn validation_error(err: validator::ValidationError) -> impl IntoResponse {}

pub async fn box_error(err: axum::BoxError) -> WebError {
    WebError::InternalServerError(format!("{:?}", err))
}
