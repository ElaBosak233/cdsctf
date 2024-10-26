use axum::response::IntoResponse;

pub async fn validation_error(err: validator::ValidationError) -> impl IntoResponse {}
