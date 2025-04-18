use axum::{
    Router,
    extract::{DefaultBodyLimit, Multipart},
};
use cds_db::transfer::User;

use crate::{
    extract::{Extension, Path},
    traits::{WebError, WebResponse},
    util,
};

pub fn router() -> Router {
    Router::new()
        .route(
            "/",
            axum::routing::post(save_user_avatar)
                .layer(DefaultBodyLimit::max(3 * 1024 * 1024 /* MB */)),
        )
        .route("/", axum::routing::delete(delete_user_avatar))
}

pub async fn save_user_avatar(
    Path(user_id): Path<i64>,
    multipart: Multipart,
) -> Result<WebResponse<()>, WebError> {
    let path = format!("users/{}/avatar", user_id);

    util::media::save_img(path, multipart).await
}

pub async fn delete_user_avatar(
    Extension(user): Extension<User>,
) -> Result<WebResponse<()>, WebError> {
    let path = format!("users/{}/avatar", user.id);

    util::media::delete_img(path).await
}
