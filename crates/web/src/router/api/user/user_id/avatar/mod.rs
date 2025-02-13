use axum::{
    Router,
    extract::{DefaultBodyLimit, Multipart},
    response::IntoResponse,
};
use cds_db::entity::user::Group;

use crate::{
    extract::{Extension, Path},
    model::Metadata,
    traits::{Ext, WebError, WebResponse},
    util,
};

pub fn router() -> Router {
    Router::new()
        .route("/", axum::routing::get(get_user_avatar))
        .route("/metadata", axum::routing::get(get_user_avatar_metadata))
        .route(
            "/",
            axum::routing::post(save_user_avatar)
                .layer(DefaultBodyLimit::max(3 * 1024 * 1024 /* MB */)),
        )
        .route("/", axum::routing::delete(delete_user_avatar))
}

pub async fn get_user_avatar(Path(user_id): Path<i64>) -> Result<impl IntoResponse, WebError> {
    let path = format!("users/{}/avatar", user_id);

    util::media::get_img(path).await
}

pub async fn get_user_avatar_metadata(
    Path(user_id): Path<i64>,
) -> Result<WebResponse<Metadata>, WebError> {
    let path = format!("users/{}/avatar", user_id);

    util::media::get_img_metadata(path).await
}

pub async fn save_user_avatar(
    Extension(ext): Extension<Ext>, Path(user_id): Path<i64>, multipart: Multipart,
) -> Result<WebResponse<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized("".into()))?;
    if operator.group != Group::Admin && operator.id != user_id {
        return Err(WebError::Forbidden("".into()));
    }

    let path = format!("users/{}/avatar", user_id);

    util::media::save_img(path, multipart).await
}

pub async fn delete_user_avatar(
    Extension(ext): Extension<Ext>, Path(user_id): Path<i64>,
) -> Result<WebResponse<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized("".into()))?;
    if operator.group != Group::Admin && operator.id != user_id {
        return Err(WebError::Forbidden("".into()));
    }

    let path = format!("users/{}/avatar", user_id);

    util::media::delete_img(path).await
}
