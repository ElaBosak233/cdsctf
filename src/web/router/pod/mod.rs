pub mod daemon;

use crate::util::jwt::Group;
use crate::web::handler;
use crate::web::middleware::auth;
use axum::{
    middleware::from_fn,
    routing::{delete, get, post, put},
    Router,
};

pub async fn router() -> Router {
    daemon::init().await;

    return Router::new()
        .route(
            "/",
            get(handler::pod::get).layer(from_fn(auth::jwt(Group::User))),
        )
        .route(
            "/",
            post(handler::pod::create).layer(from_fn(auth::jwt(Group::User))),
        )
        .route(
            "/:id",
            put(handler::pod::update).layer(from_fn(auth::jwt(Group::User))),
        )
        .route(
            "/:id",
            delete(handler::pod::delete).layer(from_fn(auth::jwt(Group::User))),
        );
}
