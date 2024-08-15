use crate::web::handler;
use crate::web::middleware::auth;
use axum::{
    middleware::from_fn,
    routing::{delete, get, post, put},
    Router,
};

pub fn router() -> Router {
    return Router::new()
        .route("/", get(handler::category::get))
        .route("/", post(handler::category::create))
        .route("/:id", put(handler::category::update))
        .route("/:id", delete(handler::category::delete));
}
