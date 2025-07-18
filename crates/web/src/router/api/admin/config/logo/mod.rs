use axum::{Router, extract::Multipart};

use crate::{
    traits::{WebError, WebResponse},
    util,
};

pub fn router() -> Router {
    Router::new()
        .route("/", axum::routing::post(save_logo))
        .route("/", axum::routing::delete(delete_logo))
}

pub async fn save_logo(multipart: Multipart) -> Result<WebResponse<()>, WebError> {
    util::media::save_img("configs/logo".to_owned(), multipart).await
}

pub async fn delete_logo() -> Result<WebResponse<()>, WebError> {
    util::media::delete_img("configs/logo".to_owned()).await
}
