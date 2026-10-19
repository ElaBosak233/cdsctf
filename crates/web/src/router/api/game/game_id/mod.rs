//! HTTP routing for `game_id` — Axum router wiring and OpenAPI route
//! registration.

/// Defines the `challenge` submodule (see sibling `*.rs` files).
pub mod challenge;

/// Defines the `icon` submodule (see sibling `*.rs` files).
mod icon;

/// Defines the `notice` submodule (see sibling `*.rs` files).
mod notice;

/// Defines the `poster` submodule (see sibling `*.rs` files).
mod poster;

/// Defines the `team` submodule (see sibling `*.rs` files).
pub mod team;

use std::{convert::Infallible, sync::Arc};

use axum::{
    Json, Router,
    extract::State,
    response::{
        IntoResponse, Sse,
        sse::{Event as SseEvent, KeepAlive},
    },
};
use cds_db::{GameView, ScoreboardEntry, game_challenge::FindGameChallengeOptions};
use cds_event::SubscribeOptions;
use futures_util::StreamExt as _;
use serde::{Deserialize, Serialize};
use serde_json::json;
use utoipa_axum::{
    router::{OpenApiRouter, UtoipaMethodRouterExt},
    routes,
};

use crate::{
    extract::{Path, Query},
    traits::{AppState, WebError},
};

/// Builds the Axum router fragment for this module.

pub fn router(state: Arc<AppState>) -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::from(Router::new().with_state(state.clone()))
        .routes(routes!(get_game).with_state(state.clone()))
        .routes(routes!(get_game_scoreboard).with_state(state.clone()))
        .routes(routes!(get_events).with_state(state.clone()))
        .nest("/challenges", challenge::router(state.clone()))
        .nest("/teams", team::router(state.clone()))
        .nest("/notices", notice::router(state.clone()))
        .nest("/icon", icon::router(state.clone()))
        .nest("/poster", poster::router(state.clone()))
}

#[derive(Clone, Debug, Serialize, utoipa::ToSchema)]
pub struct GameDetailResponse {
    pub game: GameView,
}

/// Returns game.
#[utoipa::path(
    get,
    path = "/",
    tag = "game",
    params(
        ("game_id" = i64, Path, description = "Game id"),
    ),
    responses(
        (status = 200, description = "Game", body = GameDetailResponse),
        (status = 404, description = "Not found", body = crate::traits::ErrorResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
#[tracing::instrument(skip_all, fields(handler = "get_game"))]
pub async fn get_game(
    State(s): State<Arc<AppState>>,
    Path(game_id): Path<i64>,
) -> Result<Json<GameDetailResponse>, WebError> {
    let game = crate::util::loader::prepare_game(&s.db.conn, game_id).await?;

    if !game.enabled {
        return Err(WebError::NotFound(json!("not_found")));
    }

    Ok(Json(GameDetailResponse {
        game: GameView::from(&game),
    }))
}

#[derive(Clone, Debug, Serialize, Deserialize, utoipa::ToSchema, utoipa::IntoParams)]
#[into_params(parameter_in = Query)]
pub struct GetGameScoreboardRequest {
    pub size: Option<u64>,
    pub page: Option<u64>,
}

#[derive(Clone, Debug, Serialize, utoipa::ToSchema)]
pub struct GameScoreboardResponse {
    pub records: Vec<ScoreboardEntry>,
    pub total: u64,
    /// Complete enabled challenge directory, independent of team pagination.
    pub challenges: Vec<ScoreboardChallenge>,
}

#[derive(Clone, Debug, Serialize, utoipa::ToSchema)]
pub struct ScoreboardChallenge {
    pub challenge_id: i64,
    pub challenge_title: String,
    pub challenge_category: i32,
    pub pts: i64,
}

impl From<cds_db::GameChallengeView> for ScoreboardChallenge {
    fn from(challenge: cds_db::GameChallengeView) -> Self {
        Self {
            challenge_id: challenge.challenge_id,
            challenge_title: challenge.challenge_title,
            challenge_category: challenge.challenge_category,
            pts: challenge.pts,
        }
    }
}

/// Returns game scoreboard.
#[utoipa::path(
    get,
    path = "/scoreboard",
    tag = "game",
    params(
        ("game_id" = i64, Path, description = "Game id"),
        GetGameScoreboardRequest,
    ),
    responses(
        (status = 200, description = "Scoreboard", body = GameScoreboardResponse),
        (status = 403, description = "Game is blacked out", body = crate::traits::ErrorResponse),
        (status = 404, description = "Not found", body = crate::traits::ErrorResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
#[tracing::instrument(skip_all, fields(handler = "get_game_scoreboard"))]
pub async fn get_game_scoreboard(
    State(s): State<Arc<AppState>>,
    Path(game_id): Path<i64>,
    Query(params): Query<GetGameScoreboardRequest>,
) -> Result<Json<GameScoreboardResponse>, WebError> {
    let game = crate::util::loader::prepare_game(&s.db.conn, game_id).await?;

    if !game.enabled {
        return Err(WebError::NotFound(json!("not_found")));
    }
    if game.blacked_out {
        return Err(WebError::Forbidden(json!("game_blacked_out")));
    }

    let (records, total) =
        cds_db::team::find_scoreboard(&s.db.conn, game.id, params.page, params.size).await?;

    // Spectators can read the scoreboard without joining the game. Expose only
    // column metadata, and never reveal the challenge directory before kickoff.
    let mut challenges = if time::OffsetDateTime::now_utc() >= game.started_at {
        cds_db::game_challenge::find::<ScoreboardChallenge>(
            &s.db.conn,
            FindGameChallengeOptions {
                game_id: Some(game.id),
                enabled: Some(true),
                ..Default::default()
            },
        )
        .await?
        .0
    } else {
        Vec::new()
    };
    challenges.sort_by_key(|challenge| (challenge.challenge_category, challenge.challenge_id));

    Ok(Json(GameScoreboardResponse {
        records,
        total,
        challenges,
    }))
}

#[derive(Clone, Debug, Serialize, Deserialize, utoipa::ToSchema, utoipa::IntoParams)]
#[into_params(parameter_in = Query)]
pub struct GetEventsRequest {
    pub token: String,
}

/// Returns events.
#[utoipa::path(
    get,
    path = "/events",
    tag = "game",
    params(
        ("game_id" = i64, Path, description = "Game id"),
        GetEventsRequest,
    ),
    responses(
        (status = 200, description = "SSE stream", content_type = "text/event-stream"),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
#[tracing::instrument(skip_all, fields(handler = "get_events"))]
pub async fn get_events(
    State(s): State<Arc<AppState>>,
    Path(game_id): Path<i64>,
    Query(params): Query<GetEventsRequest>,
) -> Result<impl IntoResponse, WebError> {
    let stream = s
        .event
        .subscribe(SubscribeOptions {
            game_id: Some(game_id),
            token: Some(params.token),
        })
        .await?;

    let sse_stream = stream.map(|event| {
        let Ok(evt) = event;

        // SAFETY: Infallible.
        Ok::<SseEvent, Infallible>(SseEvent::default().json_data(evt).unwrap())
    });

    Ok(Sse::new(sse_stream).keep_alive(KeepAlive::default()))
}

#[cfg(test)]
mod scoreboard_tests {
    use super::{GameScoreboardResponse, ScoreboardChallenge};

    #[test]
    fn scoreboard_directory_includes_unsolved_challenges_without_private_configuration() {
        let challenge = cds_db::GameChallengeView {
            game_id: 1,
            challenge_id: 2,
            challenge_title: "Unsolved challenge".to_owned(),
            challenge_category: 3,
            difficulty: 7,
            bonus_ratios: vec![10, 5],
            max_pts: 1000,
            min_pts: 100,
            pts: 800,
            enabled: true,
            frozen_at: None,
        };
        let response = GameScoreboardResponse {
            records: vec![],
            total: 0,
            challenges: vec![ScoreboardChallenge::from(challenge)],
        };
        let json = serde_json::to_value(response).unwrap();
        assert_eq!(
            json["challenges"],
            serde_json::json!([{
                "challenge_id": 2,
                "challenge_title": "Unsolved challenge",
                "challenge_category": 3,
                "pts": 800,
            }])
        );
        assert_eq!(json["records"], serde_json::json!([]));
    }
}
