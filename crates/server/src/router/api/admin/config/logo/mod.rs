use std::sync::Arc;

use axum::{
    Router,
    extract::{Multipart, State},
};

use crate::{
    traits::{AppState, WebError, WebResponse},
    util::media::handle_multipart,
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
    let data = handle_multipart(multipart, mime::IMAGE).await?;

    s.media
        .save("configs".to_owned(), "logo".to_owned(), data)
        .await?;

    Ok(WebResponse::default())
}

pub async fn delete_logo(State(s): State<Arc<AppState>>) -> Result<WebResponse<()>, WebError> {
    s.media
        .delete("configs".to_owned(), "logo".to_owned())
        .await?;

    Ok(WebResponse::default())
}
