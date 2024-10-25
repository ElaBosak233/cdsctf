use axum::{
    extract::DefaultBodyLimit,
    middleware::from_fn,
    routing::{delete, get, post, put},
    Router,
};

use crate::web::{handler, middleware::auth};

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
