pub mod checker;

use axum::{
    middleware::from_fn,
    routing::{delete, get, post},
    Router,
};

use crate::web::{handler, middleware::auth};

pub async fn router() -> Router {
    checker::init().await;

    return Router::new()
        .route("/", get(handler::submission::get))
        .route("/:id", get(handler::submission::get_by_id))
        .route("/", post(handler::submission::create))
        .route("/:id", delete(handler::submission::delete));
}
