use axum::{Router, response::IntoResponse};

use crate::{
    extract::Path,
    model::Metadata,
    traits::{WebError, WebResponse},
    util,
};

pub fn router() -> Router {
    Router::new()
        .route("/", axum::routing::get(get_user_avatar))
        .route("/metadata", axum::routing::get(get_user_avatar_metadata))
}

pub async fn get_user_avatar(Path(user_id): Path<i64>) -> Result<impl IntoResponse, WebError> {
    let path = format!("users/{}/avatar", user_id);

    util::media::get_first_file(path).await
}

pub async fn get_user_avatar_metadata(
    Path(user_id): Path<i64>,
) -> Result<WebResponse<Metadata>, WebError> {
    let path = format!("users/{}/avatar", user_id);

    util::media::get_first_file_metadata(path).await
}
