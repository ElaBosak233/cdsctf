use axum::{
    middleware::from_fn,
    Router,
    routing::{delete, get, post},
};
use crate::web::controller;
use crate::web::middleware::auth;
use crate::util::jwt::Group;

pub fn router() -> Router {
    return Router::new()
        .route("/", get(controller::submission::find).layer(from_fn(auth::jwt(Group::User))))
        .route("/", post(controller::submission::create).layer(from_fn(auth::jwt(Group::User))))
        .route("/:id", delete(controller::submission::delete).layer(from_fn(auth::jwt(Group::Admin))));
}
