use std::sync::Arc;

use axum::{Router, body::Body, extract::State, http::Response, response::IntoResponse};

use crate::{
    extract::Path,
    traits::{AppState, WebError},
};

pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/", axum::routing::get(get_team_avatar))
}

pub async fn get_team_avatar(
    State(s): State<Arc<AppState>>,

    Path((game_id, team_id)): Path<(i64, i64)>,
) -> Result<impl IntoResponse, WebError> {
    let path = format!("games/{game_id}/teams/{team_id}");

    let buffer = s.media.get(path, "avatar".to_owned()).await?;

    Ok(Response::builder().body(Body::from(buffer))?)
}
