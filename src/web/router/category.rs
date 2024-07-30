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
        .route("/", get(controller::category::find))
        .route(
            "/",
            post(controller::category::create).layer(from_fn(auth::jwt(Group::Admin))),
        )
        .route(
            "/:id",
            put(controller::category::update).layer(from_fn(auth::jwt(Group::Admin))),
        )
        .route(
            "/:id",
            delete(controller::category::delete).layer(from_fn(auth::jwt(Group::Admin))),
        );
}
