pub mod calculator;

use crate::util::jwt::Group;
use crate::web::handler;
use crate::web::middleware::auth;
use axum::{
    extract::DefaultBodyLimit,
    middleware::from_fn,
    routing::{delete, get, post, put},
    Router,
};

pub async fn router() -> Router {
    calculator::init().await;

    return Router::new()
        .route(
            "/",
            get(handler::game::get).layer(from_fn(auth::jwt(Group::User))),
        )
        .route(
            "/",
            post(handler::game::create).layer(from_fn(auth::jwt(Group::Admin))),
        )
        .route(
            "/:id",
            put(handler::game::update).layer(from_fn(auth::jwt(Group::Admin))),
        )
        .route(
            "/:id",
            delete(handler::game::delete).layer(from_fn(auth::jwt(Group::Admin))),
        )
        .route(
            "/:id/challenges",
            get(handler::game::get_challenge).layer(from_fn(auth::jwt(Group::User))),
        )
        .route(
            "/:id/challenges",
            post(handler::game::create_challenge).layer(from_fn(auth::jwt(Group::Admin))),
        )
        .route(
            "/:id/challenges/:challenge_id",
            put(handler::game::update_challenge).layer(from_fn(auth::jwt(Group::Admin))),
        )
        .route(
            "/:id/challenges/:challenge_id",
            delete(handler::game::delete_challenge).layer(from_fn(auth::jwt(Group::Admin))),
        )
        .route(
            "/:id/teams",
            get(handler::game::get_team).layer(from_fn(auth::jwt(Group::User))),
        )
        .route(
            "/:id/teams",
            post(handler::game::create_team).layer(from_fn(auth::jwt(Group::User))),
        )
        .route(
            "/:id/teams/:team_id",
            put(handler::game::update_team).layer(from_fn(auth::jwt(Group::Admin))),
        )
        .route(
            "/:id/teams/:team_id",
            delete(handler::game::delete_team).layer(from_fn(auth::jwt(Group::Admin))),
        )
        .route(
            "/:id/notices",
            get(handler::game::get_notice).layer(from_fn(auth::jwt(Group::User))),
        )
        .route(
            "/:id/notices",
            post(handler::game::create_notice).layer(from_fn(auth::jwt(Group::Admin))),
        )
        .route(
            "/:id/notices/:notice_id",
            put(handler::game::update_notice).layer(from_fn(auth::jwt(Group::Admin))),
        )
        .route(
            "/:id/notices/:notice_id",
            delete(handler::game::delete_notice).layer(from_fn(auth::jwt(Group::Admin))),
        )
        .route(
            "/:id/calculate",
            post(handler::game::calculate).layer(from_fn(auth::jwt(Group::Admin))),
        )
        // .route(
        //     "/:id/submissions",
        //     get(handler::game::get_submission).layer(from_fn(auth::jwt(Group::User))),
        // )
        .route("/:id/poster", get(handler::game::get_poster))
        .route(
            "/:id/poster",
            post(handler::game::save_poster)
                .layer(DefaultBodyLimit::max(3 * 1024 * 1024 /* MB */))
                .layer(from_fn(auth::jwt(Group::Admin))),
        )
        .route(
            "/:id/poster/metadata",
            get(handler::game::get_poster_metadata),
        )
        .route(
            "/:id/poster",
            delete(handler::game::delete_poster).layer(from_fn(auth::jwt(Group::Admin))),
        );
}
