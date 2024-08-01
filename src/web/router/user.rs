use crate::util::jwt::Group;
use crate::web::controller;
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
            get(controller::user::get).layer(from_fn(auth::jwt(Group::Guest))),
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
        .route(
            "/:id/teams",
            get(controller::user::get_teams).layer(from_fn(auth::jwt(Group::User))),
        )
        .route("/login", post(controller::user::login))
        .route("/register", post(controller::user::register))
        .route("/:id/avatar", get(controller::user::get_avatar))
        .route(
            "/:id/avatar/metadata",
            get(controller::user::get_avatar_metadata),
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
