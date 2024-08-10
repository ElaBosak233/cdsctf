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
            get(handler::team::get).layer(from_fn(auth::jwt(Group::Guest))),
        )
        .route(
            "/",
            post(handler::team::create).layer(from_fn(auth::jwt(Group::User))),
        )
        .route(
            "/:id",
            put(handler::team::update).layer(from_fn(auth::jwt(Group::User))),
        )
        .route(
            "/:id",
            delete(handler::team::delete).layer(from_fn(auth::jwt(Group::User))),
        )
        .route(
            "/:id/users",
            post(handler::team::create_user).layer(from_fn(auth::jwt(Group::User))),
        )
        .route(
            "/:id/users/:user_id",
            delete(handler::team::delete_user).layer(from_fn(auth::jwt(Group::User))),
        )
        .route(
            "/:id/invite",
            get(handler::team::get_invite_token).layer(from_fn(auth::jwt(Group::User))),
        )
        .route(
            "/:id/invite",
            put(handler::team::update_invite_token).layer(from_fn(auth::jwt(Group::User))),
        )
        .route(
            "/:id/join",
            post(handler::team::join).layer(from_fn(auth::jwt(Group::User))),
        )
        .route(
            "/:id/leave",
            delete(handler::team::leave).layer(from_fn(auth::jwt(Group::User))),
        )
        .route("/:id/avatar", get(handler::team::get_avatar))
        .route(
            "/:id/avatar/metadata",
            get(handler::team::get_avatar_metadata),
        )
        .route(
            "/:id/avatar",
            post(handler::team::save_avatar)
                .layer(DefaultBodyLimit::max(3 * 1024 * 1024 /* MB */))
                .layer(from_fn(auth::jwt(Group::User))),
        )
        .route(
            "/:id/avatar",
            delete(handler::team::delete_avatar).layer(from_fn(auth::jwt(Group::User))),
        );
}
