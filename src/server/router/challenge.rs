use axum::{
    extract::DefaultBodyLimit,
    middleware::from_fn,
    routing::{delete, get, post, put},
    Router,
};

use crate::server::{controller, middleware::auth};
use crate::util::jwt::Group;

pub fn router() -> Router {
    return Router::new()
        .route(
            "/",
            get(controller::challenge::find).layer(from_fn(auth::jwt(Group::User))),
        )
        .route(
            "/",
            post(controller::challenge::create).layer(from_fn(auth::jwt(Group::Admin))),
        )
        .route(
            "/status",
            post(controller::challenge::status).layer(from_fn(auth::jwt(Group::User))),
        )
        .route(
            "/:id",
            put(controller::challenge::update).layer(from_fn(auth::jwt(Group::Admin))),
        )
        .route(
            "/:id",
            delete(controller::challenge::delete).layer(from_fn(auth::jwt(Group::Admin))),
        )
        .route(
            "/:id/attachment",
            get(controller::challenge::find_attachment),
        )
        .route(
            "/:id/attachment/metadata",
            get(controller::challenge::find_attachment_metadata),
        )
        .route(
            "/:id/attachment",
            post(controller::challenge::save_attachment)
                .layer(DefaultBodyLimit::max(512 * 1024 * 1024 /* MB */))
                .layer(from_fn(auth::jwt(Group::Admin))),
        )
        .route(
            "/:id/attachment",
            delete(controller::challenge::delete_attachment)
                .layer(from_fn(auth::jwt(Group::Admin))),
        );
}
