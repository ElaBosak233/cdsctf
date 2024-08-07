use crate::util::jwt::Group;
use crate::web::controller;
use crate::web::middleware::auth;
use axum::{
    extract::DefaultBodyLimit,
    middleware::from_fn,
    routing::{delete, get, post, put},
    Router,
};

pub fn router() -> Router {
    return Router::new()
        .route(
            "/",
            get(controller::game::get).layer(from_fn(auth::jwt(Group::User))),
        )
        .route(
            "/",
            post(controller::game::create).layer(from_fn(auth::jwt(Group::Admin))),
        )
        .route(
            "/:id",
            put(controller::game::update).layer(from_fn(auth::jwt(Group::Admin))),
        )
        .route(
            "/:id",
            delete(controller::game::delete).layer(from_fn(auth::jwt(Group::Admin))),
        )
        .route(
            "/:id/challenges",
            get(controller::game::get_challenge).layer(from_fn(auth::jwt(Group::User))),
        )
        .route(
            "/:id/challenges",
            post(controller::game::create_challenge).layer(from_fn(auth::jwt(Group::Admin))),
        )
        .route(
            "/:id/challenges/:challenge_id",
            put(controller::game::update_challenge).layer(from_fn(auth::jwt(Group::Admin))),
        )
        .route(
            "/:id/challenges/:challenge_id",
            delete(controller::game::delete_challenge).layer(from_fn(auth::jwt(Group::Admin))),
        )
        .route(
            "/:id/teams",
            get(controller::game::get_team).layer(from_fn(auth::jwt(Group::User))),
        )
        .route(
            "/:id/teams",
            post(controller::game::create_team).layer(from_fn(auth::jwt(Group::User))),
        )
        .route(
            "/:id/teams/:team_id",
            put(controller::game::update_team).layer(from_fn(auth::jwt(Group::Admin))),
        )
        .route(
            "/:id/teams/:team_id",
            delete(controller::game::delete_team).layer(from_fn(auth::jwt(Group::Admin))),
        )
        .route(
            "/:id/notices",
            get(controller::game::get_notice).layer(from_fn(auth::jwt(Group::User))),
        )
        .route(
            "/:id/notices",
            post(controller::game::create_notice).layer(from_fn(auth::jwt(Group::Admin))),
        )
        .route(
            "/:id/notices/:notice_id",
            put(controller::game::update_notice).layer(from_fn(auth::jwt(Group::Admin))),
        )
        .route(
            "/:id/notices/:notice_id",
            delete(controller::game::delete_notice).layer(from_fn(auth::jwt(Group::Admin))),
        )
        .route(
            "/:id/submissions",
            get(controller::game::get_submission).layer(from_fn(auth::jwt(Group::User))),
        )
        .route("/:id/poster", get(controller::game::get_poster))
        .route(
            "/:id/poster",
            post(controller::game::save_poster)
                .layer(DefaultBodyLimit::max(3 * 1024 * 1024 /* MB */))
                .layer(from_fn(auth::jwt(Group::Admin))),
        )
        .route(
            "/:id/poster/metadata",
            get(controller::game::get_poster_metadata),
        )
        .route(
            "/:id/poster",
            delete(controller::game::delete_poster).layer(from_fn(auth::jwt(Group::Admin))),
        );
}
