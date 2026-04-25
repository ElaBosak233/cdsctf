//! HTTP routing for `challenge_id` — Axum router wiring and OpenAPI route
//! registration.

use std::sync::Arc;

use axum::{Json, Router, extract::State};
use cds_db::{
    GameChallenge,
    sea_orm::{
        ActiveValue::{Set, Unchanged},
        NotSet,
    },
};
use cds_event::types::game_challenge::{GameChallengeEvent, GameChallengeEventType};
use cds_worker::calculator::Payload;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use utoipa_axum::{
    router::{OpenApiRouter, UtoipaMethodRouterExt},
    routes,
};

use super::GameChallengeResponse;
use crate::{
    extract::{Json as ReqJson, Path},
    traits::{AppState, EmptyJson, WebError},
};

/// Builds the Axum router fragment for this module.

pub fn router(state: Arc<AppState>) -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::from(Router::new().with_state(state.clone()))
        .routes(routes!(update_game_challenge).with_state(state.clone()))
        .routes(routes!(delete_game_challenge).with_state(state.clone()))
}

#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize, utoipa::ToSchema)]
pub struct UpdateGameChallengeRequest {
    pub challenge_id: Option<i64>,
    pub enabled: Option<bool>,
    pub difficulty: Option<i64>,
    pub max_pts: Option<i64>,
    pub min_pts: Option<i64>,
    pub bonus_ratios: Option<Vec<i64>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    pub frozen_at: Option<Option<i64>>,
}

/// Updates game challenge.
#[utoipa::path(
    put,
    path = "/",
    tag = "admin-game",
    params(
        ("game_id" = i64, Path, description = "Game id"),
        ("challenge_id" = i64, Path, description = "Challenge id"),
    ),
    request_body = UpdateGameChallengeRequest,
    responses(
        (status = 200, description = "Updated link", body = GameChallengeResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
#[tracing::instrument(skip_all, fields(handler = "update_game_challenge"))]
pub async fn update_game_challenge(
    State(s): State<Arc<AppState>>,
    Path((game_id, challenge_id)): Path<(i64, i64)>,
    ReqJson(body): ReqJson<UpdateGameChallengeRequest>,
) -> Result<Json<GameChallengeResponse>, WebError> {
    let game_challenge =
        crate::util::loader::prepare_game_challenge(&s.db.conn, game_id, challenge_id).await?;

    let new_game_challenge = cds_db::game_challenge::update::<GameChallenge>(
        &s.db.conn,
        cds_db::game_challenge::ActiveModel {
            game_id: Unchanged(game_challenge.game_id),
            challenge_id: Unchanged(game_challenge.challenge_id),
            enabled: body.enabled.map_or(NotSet, Set),
            difficulty: body.difficulty.map_or(NotSet, Set),
            max_pts: body.max_pts.map_or(NotSet, Set),
            min_pts: body.min_pts.map_or(NotSet, Set),
            bonus_ratios: body.bonus_ratios.map_or(NotSet, Set),
            frozen_at: body.frozen_at.map_or(NotSet, Set),
            ..Default::default()
        },
    )
    .await?;

    if game_challenge.difficulty != new_game_challenge.difficulty
        || game_challenge.max_pts != new_game_challenge.max_pts
        || game_challenge.min_pts != new_game_challenge.min_pts
        || game_challenge.bonus_ratios != new_game_challenge.bonus_ratios
    {
        s.queue
            .publish(
                "calculator",
                Payload {
                    game_id: Some(new_game_challenge.game_id),
                },
            )
            .await?;
    }

    if new_game_challenge.enabled != game_challenge.enabled {
        s.event
            .push(cds_event::types::Event::GameChallenge(GameChallengeEvent {
                type_: if new_game_challenge.enabled {
                    GameChallengeEventType::Up
                } else {
                    GameChallengeEventType::Down
                },
            }))
            .await?;
    }

    Ok(Json(GameChallengeResponse {
        game_challenge: new_game_challenge,
    }))
}

/// Deletes game challenge.
#[utoipa::path(
    delete,
    path = "/",
    tag = "admin-game",
    params(
        ("game_id" = i64, Path, description = "Game id"),
        ("challenge_id" = i64, Path, description = "Challenge id"),
    ),
    responses(
        (status = 200, description = "Removed link", body = EmptyJson),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
#[tracing::instrument(skip_all, fields(handler = "delete_game_challenge"))]
pub async fn delete_game_challenge(
    State(s): State<Arc<AppState>>,
    Path((game_id, challenge_id)): Path<(i64, i64)>,
) -> Result<Json<EmptyJson>, WebError> {
    let game_challenge =
        crate::util::loader::prepare_game_challenge(&s.db.conn, game_id, challenge_id).await?;

    cds_db::game_challenge::delete(
        &s.db.conn,
        game_challenge.game_id,
        game_challenge.challenge_id,
    )
    .await?;

    Ok(Json(EmptyJson::default()))
}
