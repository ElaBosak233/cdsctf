use std::sync::Arc;

use axum::{Router, extract::State, response::IntoResponse};

use crate::{
    extract::Path,
    model::Metadata,
    traits::{AppState, WebError, WebResponse},
    util,
};

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", axum::routing::get(get_team_avatar))
        .route("/metadata", axum::routing::get(get_team_avatar_metadata))
}

pub async fn get_team_avatar(
    State(s): State<Arc<AppState>>,

    Path((game_id, team_id)): Path<(i64, i64)>,
) -> Result<impl IntoResponse, WebError> {
    let path = format!("games/{game_id}/teams/{team_id}/avatar");

    util::media::get_first_file(s.media.clone(), path).await
}

pub async fn get_team_avatar_metadata(
    State(s): State<Arc<AppState>>,

    Path((game_id, team_id)): Path<(i64, i64)>,
) -> Result<WebResponse<Metadata>, WebError> {
    let path = format!("games/{game_id}/teams/{team_id}/avatar");

    util::media::get_first_file_metadata(s.media.clone(), path).await
}
