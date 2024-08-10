use crate::util::jwt::Group;
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
        .route(
            "/",
            get(handler::challenge::get).layer(from_fn(auth::jwt(Group::User))),
        )
        .route(
            "/",
            post(handler::challenge::create).layer(from_fn(auth::jwt(Group::Admin))),
        )
        .route(
            "/status",
            post(handler::challenge::get_status).layer(from_fn(auth::jwt(Group::User))),
        )
        .route(
            "/:id",
            put(handler::challenge::update).layer(from_fn(auth::jwt(Group::Admin))),
        )
        .route(
            "/:id",
            delete(handler::challenge::delete).layer(from_fn(auth::jwt(Group::Admin))),
        )
        .route("/:id/attachment", get(handler::challenge::get_attachment))
        .route(
            "/:id/attachment/metadata",
            get(handler::challenge::get_attachment_metadata),
        )
        .route(
            "/:id/attachment",
            post(handler::challenge::save_attachment)
                .layer(DefaultBodyLimit::max(512 * 1024 * 1024 /* MB */))
                .layer(from_fn(auth::jwt(Group::Admin))),
        )
        .route(
            "/:id/attachment",
            delete(handler::challenge::delete_attachment).layer(from_fn(auth::jwt(Group::Admin))),
        );
}
