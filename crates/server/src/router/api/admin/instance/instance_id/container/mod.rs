mod container_id;

use std::sync::Arc;

use axum::Router;

use crate::traits::{AppState, WebError, WebResponse};

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", axum::routing::get(get_container))
        .nest("/{container_id}", container_id::router())
}

pub async fn get_container() -> Result<WebResponse<()>, WebError> {
    Ok(WebResponse::ok())
}
