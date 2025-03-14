use axum::{
    Router,
    extract::{DefaultBodyLimit, Multipart},
    response::IntoResponse,
};
use cds_db::entity::user::Group;
use serde_json::json;

use crate::{
    extract::{Extension, Path},
    model::Metadata,
    traits::{Ext, WebError, WebResponse},
    util,
};

pub fn router() -> Router {
    Router::new()
        .route("/", axum::routing::get(get_game_poster))
        .route("/metadata", axum::routing::get(get_game_poster_metadata))
}

pub async fn get_game_poster(Path(game_id): Path<i64>) -> Result<impl IntoResponse, WebError> {
    let path = format!("games/{}/poster", game_id);

    util::media::get_img(path).await
}

pub async fn get_game_poster_metadata(
    Path(game_id): Path<i64>,
) -> Result<WebResponse<Metadata>, WebError> {
    let path = format!("games/{}/poster", game_id);

    util::media::get_img_metadata(path).await
}
