use crate::util::jwt::Group;
use crate::web::controller;
use crate::web::middleware::auth;
use axum::{
    middleware::from_fn,
    routing::{delete, get, post},
    Router,
};

pub fn router() -> Router {
    return Router::new()
        .route(
            "/",
            get(controller::submission::get).layer(from_fn(auth::jwt(Group::User))),
        )
        .route(
            "/",
            post(controller::submission::create).layer(from_fn(auth::jwt(Group::User))),
        )
        .route(
            "/:id",
            delete(controller::submission::delete).layer(from_fn(auth::jwt(Group::Admin))),
        );
}
