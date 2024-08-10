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
            get(handler::user::get).layer(from_fn(auth::jwt(Group::Guest))),
        )
        .route(
            "/",
            post(handler::user::create).layer(from_fn(auth::jwt(Group::Admin))),
        )
        .route(
            "/:id",
            put(handler::user::update).layer(from_fn(auth::jwt(Group::User))),
        )
        .route(
            "/:id",
            delete(handler::user::delete).layer(from_fn(auth::jwt(Group::Admin))),
        )
        .route(
            "/:id/teams",
            get(handler::user::get_teams).layer(from_fn(auth::jwt(Group::User))),
        )
        .route("/login", post(handler::user::login))
        .route("/register", post(handler::user::register))
        .route("/:id/avatar", get(handler::user::get_avatar))
        .route(
            "/:id/avatar/metadata",
            get(handler::user::get_avatar_metadata),
        )
        .route(
            "/:id/avatar",
            post(handler::user::save_avatar)
                .layer(DefaultBodyLimit::max(3 * 1024 * 1024 /* MB */))
                .layer(from_fn(auth::jwt(Group::User))),
        )
        .route(
            "/:id/avatar",
            delete(handler::user::delete_avatar).layer(from_fn(auth::jwt(Group::User))),
        );
}
