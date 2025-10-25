use crate::{extract::Path, traits::WebError, util};
use axum::response::IntoResponse;
use axum::Router;

pub fn router() -> Router {
    Router::new()
        .route(
            "/",
            axum::routing::get(get_team_write_up)
        )
}

pub async fn get_team_write_up(
    Path((game_id, team_id)): Path<(i64, i64)>,
) -> Result<impl IntoResponse, WebError> {
    let path = format!("games/{}/teams/{}/writeup", game_id, team_id);

    util::media::get_first_file(path).await
}