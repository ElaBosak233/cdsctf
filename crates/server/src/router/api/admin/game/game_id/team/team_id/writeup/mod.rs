use std::sync::Arc;

use axum::{Router, extract::State, response::IntoResponse};

use crate::{
    extract::Path,
    traits::{AppState, WebError},
    util,
};

pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/", axum::routing::get(get_team_write_up))
}

pub async fn get_team_write_up(
    State(s): State<Arc<AppState>>,

    Path((game_id, team_id)): Path<(i64, i64)>,
) -> Result<impl IntoResponse, WebError> {
    util::media::get_write_up(s.media.clone(), game_id, team_id).await
}
