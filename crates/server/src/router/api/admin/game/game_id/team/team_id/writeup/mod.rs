use axum::{Router, response::IntoResponse};

use crate::{extract::Path, traits::WebError, util};

pub fn router() -> Router {
    Router::new().route("/", axum::routing::get(get_team_write_up))
}

pub async fn get_team_write_up(
    Path((game_id, team_id)): Path<(i64, i64)>,
) -> Result<impl IntoResponse, WebError> {
    util::media::get_write_up(game_id, team_id).await
}
