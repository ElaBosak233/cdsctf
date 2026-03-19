use std::sync::Arc;

use axum::{Router, body::Body, extract::State, http::Response, response::IntoResponse};

use crate::{
    extract::Path,
    traits::{AppState, WebError},
};

pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/", axum::routing::get(get_game_poster))
}

pub async fn get_game_poster(
    State(s): State<Arc<AppState>>,

    Path(game_id): Path<i64>,
) -> Result<impl IntoResponse, WebError> {
    let path = format!("games/{}", game_id);

    let buffer = s.media.get(path, "poster".to_owned()).await?;

    Ok(Response::builder().body(Body::from(buffer))?)
}
