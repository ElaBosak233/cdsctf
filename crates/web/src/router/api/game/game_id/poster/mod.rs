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
        .route(
            "/",
            axum::routing::post(save_game_poster)
                .layer(DefaultBodyLimit::max(3 * 1024 * 1024 /* MB */)),
        )
        .route("/metadata", axum::routing::get(get_game_poster_metadata))
        .route("/", axum::routing::delete(delete_game_poster))
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

pub async fn save_game_poster(
    Extension(ext): Extension<Ext>, Path(game_id): Path<i64>, multipart: Multipart,
) -> Result<WebResponse<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    if operator.group != Group::Admin {
        return Err(WebError::Forbidden(json!("")));
    }

    let path = format!("games/{}/poster", game_id);

    util::media::save_img(path, multipart).await
}

pub async fn delete_game_poster(
    Extension(ext): Extension<Ext>, Path(game_id): Path<i64>,
) -> Result<WebResponse<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    if operator.group != Group::Admin {
        return Err(WebError::Forbidden(json!("")));
    }

    let path = format!("games/{}/poster", game_id);

    util::media::delete_img(path).await
}
