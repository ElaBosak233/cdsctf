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
        .route(
            "/",
            axum::routing::post(save_game_icon)
                .layer(DefaultBodyLimit::max(3 * 1024 * 1024 /* MB */)),
        )
        .route("/", axum::routing::delete(delete_game_icon))
}

pub async fn save_game_icon(
    Path(game_id): Path<i64>, multipart: Multipart,
) -> Result<WebResponse<()>, WebError> {
    let path = format!("games/{}/icon", game_id);

    util::media::save_img(path, multipart).await
}

pub async fn delete_game_icon(Path(game_id): Path<i64>) -> Result<WebResponse<()>, WebError> {
    let path = format!("games/{}/icon", game_id);

    util::media::delete_img(path).await
}
