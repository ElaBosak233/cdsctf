pub mod daemon;

use axum::{
    routing::{get, post},
    Router,
};

use crate::web::handler;

pub async fn router() -> Router {
    daemon::init().await;

    return Router::new()
        .route("/", get(handler::pod::get))
        .route("/", post(handler::pod::create))
        .route("/:id/renew", post(handler::pod::renew))
        .route("/:id/stop", post(handler::pod::stop));
}
