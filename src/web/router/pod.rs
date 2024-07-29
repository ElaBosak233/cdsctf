use axum::{
    middleware::from_fn,
    Router,
    routing::{delete, get, post, put},
};
use crate::web::controller;
use crate::web::middleware::auth;
use crate::util::jwt::Group;

pub fn router() -> Router {
    return Router::new()
        .route("/", get(controller::pod::find).layer(from_fn(auth::jwt(Group::User))))
        .route("/", post(controller::pod::create).layer(from_fn(auth::jwt(Group::User))))
        .route("/:id", put(controller::pod::update).layer(from_fn(auth::jwt(Group::User))))
        .route("/:id", delete(controller::pod::delete).layer(from_fn(auth::jwt(Group::User))));
}
