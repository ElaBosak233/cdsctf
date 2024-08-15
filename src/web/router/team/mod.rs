use crate::web::handler;
use axum::{
    extract::DefaultBodyLimit,
    routing::{delete, get, post, put},
    Router,
};

pub fn router() -> Router {
    return Router::new()
        .route("/", get(handler::team::get))
        .route("/", post(handler::team::create))
        .route("/:id", put(handler::team::update))
        .route("/:id", delete(handler::team::delete))
        .route("/:id/users", post(handler::team::create_user))
        .route("/:id/users/:user_id", delete(handler::team::delete_user))
        .route("/:id/invite", get(handler::team::get_invite_token))
        .route("/:id/invite", put(handler::team::update_invite_token))
        .route("/:id/join", post(handler::team::join))
        .route("/:id/leave", delete(handler::team::leave))
        .route("/:id/avatar", get(handler::team::get_avatar))
        .route(
            "/:id/avatar/metadata",
            get(handler::team::get_avatar_metadata),
        )
        .route(
            "/:id/avatar",
            post(handler::team::save_avatar)
                .layer(DefaultBodyLimit::max(3 * 1024 * 1024 /* MB */)),
        )
        .route("/:id/avatar", delete(handler::team::delete_avatar));
}
