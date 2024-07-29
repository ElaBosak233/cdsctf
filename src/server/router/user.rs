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
            get(controller::user::find).layer(from_fn(auth::jwt(Group::Guest))),
        )
        .route(
            "/",
            post(controller::user::create).layer(from_fn(auth::jwt(Group::Admin))),
        )
        .route(
            "/:id",
            put(controller::user::update).layer(from_fn(auth::jwt(Group::User))),
        )
        .route(
            "/:id",
            delete(controller::user::delete).layer(from_fn(auth::jwt(Group::Admin))),
        )
        .route("/login", post(controller::user::login))
        .route("/register", post(controller::user::register))
        .route("/:id/avatar", get(controller::user::find_avatar))
        .route(
            "/:id/avatar/metadata",
            get(controller::user::find_avatar_metadata),
        )
        .route(
            "/:id/avatar",
            post(controller::user::save_avatar)
                .layer(DefaultBodyLimit::max(3 * 1024 * 1024 /* MB */))
                .layer(from_fn(auth::jwt(Group::User))),
        )
        .route(
            "/:id/avatar",
            delete(controller::user::delete_avatar).layer(from_fn(auth::jwt(Group::User))),
        );
}