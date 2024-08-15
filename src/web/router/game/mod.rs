pub mod calculator;

use crate::web::handler;
use axum::{
    extract::DefaultBodyLimit,
    routing::{delete, get, post, put},
    Router,
};

pub async fn router() -> Router {
    calculator::init().await;

    return Router::new()
        .route("/", get(handler::game::get))
        .route("/", post(handler::game::create))
        .route("/:id", put(handler::game::update))
        .route("/:id", delete(handler::game::delete))
        .route("/:id/challenges", get(handler::game::get_challenge))
        .route("/:id/challenges", post(handler::game::create_challenge))
        .route(
            "/:id/challenges/:challenge_id",
            put(handler::game::update_challenge),
        )
        .route(
            "/:id/challenges/:challenge_id",
            delete(handler::game::delete_challenge),
        )
        .route("/:id/teams", get(handler::game::get_team))
        .route("/:id/teams", post(handler::game::create_team))
        .route("/:id/teams/:team_id", put(handler::game::update_team))
        .route("/:id/teams/:team_id", delete(handler::game::delete_team))
        .route("/:id/notices", get(handler::game::get_notice))
        .route("/:id/notices", post(handler::game::create_notice))
        .route("/:id/notices/:notice_id", put(handler::game::update_notice))
        .route(
            "/:id/notices/:notice_id",
            delete(handler::game::delete_notice),
        )
        .route("/:id/calculate", post(handler::game::calculate))
        // .route(
        //     "/:id/submissions",
        //     get(handler::game::get_submission).layer(from_fn(auth::jwt(Group::User))),
        // )
        .route("/:id/poster", get(handler::game::get_poster))
        .route(
            "/:id/poster",
            post(handler::game::save_poster)
                .layer(DefaultBodyLimit::max(3 * 1024 * 1024 /* MB */)),
        )
        .route(
            "/:id/poster/metadata",
            get(handler::game::get_poster_metadata),
        )
        .route("/:id/poster", delete(handler::game::delete_poster));
}
