use std::sync::Arc;

use axum::{
    body::Body,
    extract::State,
    http::Response,
    response::{IntoResponse, Redirect},
};

use crate::traits::{AppState, WebError};


#[utoipa::path(
    get,
    path = "/",
    tag = "config",
    responses(
        (status = 200, description = "Custom logo bytes (Content-Type varies)"),
        (status = 302, description = "Redirect to default /logo.svg when unset"),
        (status = 500, description = "Server error", body = crate::traits::ApiJsonError),
    )
)]
pub async fn get_logo(State(s): State<Arc<AppState>>) -> Result<impl IntoResponse, WebError> {
    match s.media.config().logo().get_logo().await {
        Ok(buffer) => Ok(Response::builder().body(Body::from(buffer))?),
        _ => Ok(Redirect::to("/logo.svg").into_response()),
    }
}
