use crate::web::handler;
use axum::{
    extract::DefaultBodyLimit,
    routing::{delete, get, post, put},
    Router,
};

pub fn router() -> Router {
    return Router::new()
        .route("/", get(handler::user::get))
        .route("/", post(handler::user::create))
        .route("/:id", put(handler::user::update))
        .route("/:id", delete(handler::user::delete))
        .route("/:id/teams", get(handler::user::get_teams))
        .route("/login", post(handler::user::login))
        .route("/register", post(handler::user::register))
        .route("/:id/avatar", get(handler::user::get_avatar))
        .route(
            "/:id/avatar/metadata",
            get(handler::user::get_avatar_metadata),
        )
        .route(
            "/:id/avatar",
            post(handler::user::save_avatar)
                .layer(DefaultBodyLimit::max(3 * 1024 * 1024 /* MB */)),
        )
        .route("/:id/avatar", delete(handler::user::delete_avatar));
}
