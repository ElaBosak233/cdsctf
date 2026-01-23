use std::sync::Arc;

use axum::{
    Router,
    extract::{Multipart, State},
};

use crate::{
    traits::{AppState, WebError, WebResponse},
    util,
};

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", axum::routing::post(save_logo))
        .route("/", axum::routing::delete(delete_logo))
}

pub async fn save_logo(
    State(s): State<Arc<AppState>>,

    multipart: Multipart,
) -> Result<WebResponse<()>, WebError> {
    util::media::save_img(s.media.clone(), "configs/logo".to_owned(), multipart).await
}

pub async fn delete_logo(State(s): State<Arc<AppState>>) -> Result<WebResponse<()>, WebError> {
    util::media::delete_img(s.media.clone(), "configs/logo".to_owned()).await
}
