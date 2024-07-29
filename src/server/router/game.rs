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
        .route(
            "/",
            get(controller::game::find).layer(from_fn(auth::jwt(Group::User))),
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
            get(controller::game::find_challenge).layer(from_fn(auth::jwt(Group::User))),
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
            get(controller::game::find_team).layer(from_fn(auth::jwt(Group::User))),
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
            get(controller::game::find_notice).layer(from_fn(auth::jwt(Group::User))),
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
        .route("/:id/poster", get(controller::game::find_poster))
        .route(
            "/:id/poster",
            post(controller::game::save_poster)
                .layer(DefaultBodyLimit::max(3 * 1024 * 1024 /* MB */))
                .layer(from_fn(auth::jwt(Group::Admin))),
        )
        .route(
            "/:id/poster",
            delete(controller::game::delete_poster).layer(from_fn(auth::jwt(Group::Admin))),
        );
}