pub mod checker;

use crate::web::handler;
use crate::web::middleware::auth;
use axum::{
    middleware::from_fn,
    routing::{delete, get, post},
    Router,
};

pub async fn router() -> Router {
    checker::init().await;

    return Router::new()
        .route("/", get(handler::submission::get))
        .route("/:id", get(handler::submission::get_by_id))
        .route("/", post(handler::submission::create))
        .route("/:id", delete(handler::submission::delete));
}
