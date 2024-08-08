use crate::util::jwt::Group;
use crate::web::handler;
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
            get(handler::submission::get).layer(from_fn(auth::jwt(Group::User))),
        )
        .route(
            "/:id",
            get(handler::submission::get_by_id).layer(from_fn(auth::jwt(Group::User))),
        )
        .route(
            "/",
            post(handler::submission::create).layer(from_fn(auth::jwt(Group::User))),
        )
        .route(
            "/:id",
            delete(handler::submission::delete).layer(from_fn(auth::jwt(Group::Admin))),
        );
}
