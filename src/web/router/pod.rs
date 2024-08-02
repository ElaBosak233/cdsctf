use crate::util::jwt::Group;
use crate::web::controller;
use crate::web::middleware::auth;
use axum::{
    middleware::from_fn,
    routing::{delete, get, post, put},
    Router,
};

pub fn router() -> Router {
    return Router::new()
        .route(
            "/",
            get(controller::pod::get).layer(from_fn(auth::jwt(Group::User))),
        )
        .route(
            "/",
            post(controller::pod::create).layer(from_fn(auth::jwt(Group::User))),
        )
        .route(
            "/:id",
            put(controller::pod::update).layer(from_fn(auth::jwt(Group::User))),
        )
        .route(
            "/:id",
            delete(controller::pod::delete).layer(from_fn(auth::jwt(Group::User))),
        );
}
