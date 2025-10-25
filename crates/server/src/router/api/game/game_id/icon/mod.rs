use axum::{Router, response::IntoResponse};

use crate::{
    extract::Path,
    model::Metadata,
    traits::{WebError, WebResponse},
    util,
};

pub fn router() -> Router {
    Router::new()
        .route("/", axum::routing::get(get_game_icon))
        .route("/metadata", axum::routing::get(get_game_icon_metadata))
}

pub async fn get_game_icon(Path(game_id): Path<i64>) -> Result<impl IntoResponse, WebError> {
    let path = format!("games/{}/icon", game_id);

    util::media::get_first_file(path).await
}

pub async fn get_game_icon_metadata(
    Path(game_id): Path<i64>,
) -> Result<WebResponse<Metadata>, WebError> {
    let path = format!("games/{}/icon", game_id);

    util::media::get_first_file_metadata(path).await
}
