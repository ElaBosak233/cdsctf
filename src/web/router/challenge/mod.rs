use crate::web::handler;
use crate::web::middleware::auth;
use axum::{
    extract::DefaultBodyLimit,
    middleware::from_fn,
    routing::{delete, get, post, put},
    Router,
};

pub fn router() -> Router {
    return Router::new()
        .route("/", get(handler::challenge::get))
        .route("/", post(handler::challenge::create))
        .route("/status", post(handler::challenge::get_status))
        .route("/:id", put(handler::challenge::update))
        .route("/:id", delete(handler::challenge::delete))
        .route("/:id/attachment", get(handler::challenge::get_attachment))
        .route(
            "/:id/attachment/metadata",
            get(handler::challenge::get_attachment_metadata),
        )
        .route(
            "/:id/attachment",
            post(handler::challenge::save_attachment)
                .layer(DefaultBodyLimit::max(512 * 1024 * 1024 /* MB */)),
        )
        .route(
            "/:id/attachment",
            delete(handler::challenge::delete_attachment),
        );
}
