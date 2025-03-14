mod container_id;

use axum::{Router, http::StatusCode};

use crate::traits::{WebError, WebResponse};

pub fn router() -> Router {
    Router::new()
        .route("/", axum::routing::get(get_container))
        .nest("/{container_id}", container_id::router())
}

pub async fn get_container() -> Result<WebResponse<()>, WebError> {
    Ok(WebResponse {
        code: StatusCode::OK,
        ..Default::default()
    })
}
