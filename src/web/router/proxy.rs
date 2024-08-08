use axum::{routing::get, Router};

use crate::web::handler;

pub fn router() -> Router {
    return Router::new().route("/:token", get(handler::proxy::link));
}
