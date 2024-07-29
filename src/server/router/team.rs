use axum::{
    extract::DefaultBodyLimit,
    middleware::from_fn,
    routing::{delete, get, post, put},
    Router,
};

use crate::server::{controller, middleware::auth};
use crate::util::jwt::Group;

pub fn router() -> Router {
    return Router::new()
        .route("/", get(controller::team::find).layer(from_fn(auth::jwt(Group::Guest))))
        .route("/", post(controller::team::create).layer(from_fn(auth::jwt(Group::User))))
        .route("/:id", put(controller::team::update).layer(from_fn(auth::jwt(Group::User))))
        .route("/:id", delete(controller::team::delete).layer(from_fn(auth::jwt(Group::User))))
        .route("/:id/users", post(controller::team::create_user).layer(from_fn(auth::jwt(Group::User))))
        .route(
            "/:id/users/:user_id",
            delete(controller::team::delete_user).layer(from_fn(auth::jwt(Group::User))),
        )
        .route("/:id/invite", get(controller::team::get_invite_token).layer(from_fn(auth::jwt(Group::User))))
        .route("/:id/invite", put(controller::team::update_invite_token).layer(from_fn(auth::jwt(Group::User))))
        .route("/:id/join", post(controller::team::join).layer(from_fn(auth::jwt(Group::User))))
        .route("/:id/leave", delete(controller::team::leave).layer(from_fn(auth::jwt(Group::User))))
        .route("/:id/avatar", get(controller::team::find_avatar))
        .route("/:id/avatar/metadata", get(controller::team::find_avatar_metadata))
        .route(
            "/:id/avatar",
            post(controller::team::save_avatar)
                .layer(DefaultBodyLimit::max(3 * 1024 * 1024 /* MB */))
                .layer(from_fn(auth::jwt(Group::User))),
        )
        .route("/:id/avatar", delete(controller::team::delete_avatar).layer(from_fn(auth::jwt(Group::User))));
}
