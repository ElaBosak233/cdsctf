use std::sync::Arc;

use axum::{Router, body::Body, extract::State, http::Response, response::IntoResponse};

use crate::{
    extract::Path,
    traits::{AppState, WebError},
};

pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/", axum::routing::get(get_team_avatar))
}

#[utoipa::path(
    get,
    path = "/",
    tag = "game",
    params(
        ("game_id" = i64, Path, description = "Game id"),
        ("team_id" = i64, Path, description = "Team id"),
    ),
    responses(
        (status = 200, description = "Avatar bytes"),
        (status = 404, description = "Not found", body = crate::traits::ApiJsonError),
    )
)]
pub async fn get_team_avatar(
    State(s): State<Arc<AppState>>,

    Path((game_id, team_id)): Path<(i64, i64)>,
) -> Result<impl IntoResponse, WebError> {
    let path = format!("games/{game_id}/teams/{team_id}");

    let buffer = s.media.get(path, "avatar".to_owned()).await?;

    Ok(Response::builder().body(Body::from(buffer))?)
}
