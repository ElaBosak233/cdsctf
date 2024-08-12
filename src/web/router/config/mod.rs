use axum::{routing::get, Router};

use crate::web::handler;

pub fn router() -> Router {
    return Router::new()
        .route("/", get(handler::config::get))
        .route("/favicon", get(handler::config::get_favicon));
}
