use std::sync::Arc;

use crate::traits::{AppState, WebError};
use axum::{
    body::Body,
    extract::State,
    http::Response,
    response::{IntoResponse, Redirect},
    Router,
};

pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/", axum::routing::get(get_logo))
}

pub async fn get_logo(State(s): State<Arc<AppState>>) -> Result<impl IntoResponse, WebError> {
    match s.media.config().logo().get_logo().await {
        Ok(buffer) => Ok(Response::builder().body(Body::from(buffer))?),
        _ => Ok(Redirect::to("/logo.svg").into_response()),
    }
}
