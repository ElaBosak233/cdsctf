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
        .route("/", axum::routing::get(get_game_poster))
        .route("/metadata", axum::routing::get(get_game_poster_metadata))
}

pub async fn get_game_poster(
    State(s): State<Arc<AppState>>,

    Path(game_id): Path<i64>,
) -> Result<impl IntoResponse, WebError> {
    let path = format!("games/{}/poster", game_id);

    util::media::get_first_file(s.media.clone(), path).await
}

pub async fn get_game_poster_metadata(
    State(s): State<Arc<AppState>>,

    Path(game_id): Path<i64>,
) -> Result<WebResponse<Metadata>, WebError> {
    let path = format!("games/{}/poster", game_id);

    util::media::get_first_file_metadata(s.media.clone(), path).await
}
