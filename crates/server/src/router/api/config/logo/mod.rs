use std::sync::Arc;

use axum::{
    Router,
    body::Body,
    extract::State,
    http::Response,
    response::{IntoResponse, Redirect},
};

use crate::traits::{AppState, WebError};

pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/", axum::routing::get(get_logo))
}

pub async fn get_logo(State(ref s): State<Arc<AppState>>) -> Result<impl IntoResponse, WebError> {
    match s.media.config().logo().get_logo().await {
        Ok(buffer) => Ok(Response::builder().body(Body::from(buffer))?),
        _ => Ok(Redirect::to("/logo.svg").into_response()),
    }
}
