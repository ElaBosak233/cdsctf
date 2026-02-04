use std::sync::Arc;

use axum::{Router, body::Body, extract::State, http::Response, response::IntoResponse};

use crate::{
    extract::Path,
    traits::{AppState, WebError},
};

pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/", axum::routing::get(get_user_avatar))
}

pub async fn get_user_avatar(
    State(s): State<Arc<AppState>>,

    Path(user_id): Path<i64>,
) -> Result<impl IntoResponse, WebError> {
    let path = format!("users/{}", user_id);

    let buffer = s.media.get(path, "avatar".to_owned()).await?;

    Ok(Response::builder().body(Body::from(buffer))?)
}
