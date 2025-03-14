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
