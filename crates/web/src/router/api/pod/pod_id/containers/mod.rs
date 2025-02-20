use axum::{Router, http::StatusCode};

use crate::traits::{WebError, WebResponse};

pub mod container_id;

pub fn router() -> Router {
    Router::new()
        .route("/", axum::routing::get(get_container))
        .nest("/{container_id}", container_id::router())
}

pub async fn get_container() -> Result<WebResponse<()>, WebError> {
    Ok(WebResponse {
        code: StatusCode::OK.as_u16(),
        ..Default::default()
    })
}
