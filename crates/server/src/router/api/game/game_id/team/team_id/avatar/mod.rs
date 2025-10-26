use axum::{Router, response::IntoResponse};

use crate::{
    extract::Path,
    model::Metadata,
    traits::{WebError, WebResponse},
    util,
};

pub fn router() -> Router {
    Router::new()
        .route("/", axum::routing::get(get_team_avatar))
        .route("/metadata", axum::routing::get(get_team_avatar_metadata))
}

pub async fn get_team_avatar(
    Path((game_id, team_id)): Path<(i64, i64)>,
) -> Result<impl IntoResponse, WebError> {
    let path = format!("games/{game_id}/teams/{team_id}/avatar");

    util::media::get_first_file(path).await
}

pub async fn get_team_avatar_metadata(
    Path((game_id, team_id)): Path<(i64, i64)>,
) -> Result<WebResponse<Metadata>, WebError> {
    let path = format!("games/{game_id}/teams/{team_id}/avatar");

    util::media::get_first_file_metadata(path).await
}
