use axum::{
    Router,
    body::Body,
    http::Response,
    response::{IntoResponse, Redirect},
};

use crate::traits::WebError;

pub fn router() -> Router {
    Router::new().route("/", axum::routing::get(get_logo))
}

pub async fn get_logo() -> Result<impl IntoResponse, WebError> {
    match cds_media::config::logo::get_logo().await {
        Ok(buffer) => Ok(Response::builder().body(Body::from(buffer)).unwrap()),
        _ => Ok(Redirect::to("/logo.svg").into_response()),
    }
}
