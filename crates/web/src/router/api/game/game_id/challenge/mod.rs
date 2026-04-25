//! HTTP routing for `challenge` — Axum router wiring and OpenAPI route
//! registration.

use std::sync::Arc;

use axum::{Json, Router, extract::State};
use cds_db::{GameChallengeMini, game_challenge::FindGameChallengeOptions, team::State as TState};
use serde::{Deserialize, Serialize};
use serde_json::json;
use utoipa_axum::{
    router::{OpenApiRouter, UtoipaMethodRouterExt},
    routes,
};

use crate::{
    extract::{Extension, Path, Query},
    traits::{AppState, AuthPrincipal, WebError},
};

/// Builds the Axum router fragment for this module.

pub fn router(state: Arc<AppState>) -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::from(Router::new().with_state(state.clone()))
        .routes(routes!(get_game_challenge).with_state(state.clone()))
}

#[derive(Clone, Debug, Serialize, Deserialize, utoipa::ToSchema, utoipa::IntoParams)]
#[into_params(parameter_in = Query)]
pub struct GetGameChallengeRequest {
    pub game_id: Option<i64>,
    pub challenge_id: Option<i64>,
    pub category: Option<i32>,
}

#[derive(Clone, Debug, Serialize, utoipa::ToSchema)]
pub struct GameChallengesListResponse {
    pub challenges: Vec<GameChallengeMini>,
    pub total: u64,
}

/// Returns game challenge.
#[utoipa::path(
    get,
    path = "/",
    tag = "game",
    params(
        ("game_id" = i64, Path, description = "Game id"),
        GetGameChallengeRequest,
    ),
    responses(
        (status = 200, description = "Game challenges", body = GameChallengesListResponse),
        (status = 401, description = "Unauthorized", body = crate::traits::ErrorResponse),
        (status = 403, description = "Forbidden", body = crate::traits::ErrorResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
#[tracing::instrument(skip_all, fields(handler = "get_game_challenge"))]
pub async fn get_game_challenge(
    State(s): State<Arc<AppState>>,
    Extension(ext): Extension<AuthPrincipal>,
    Path(game_id): Path<i64>,
    Query(params): Query<GetGameChallengeRequest>,
) -> Result<Json<GameChallengesListResponse>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;

    let game = crate::util::loader::prepare_game(&s.db.conn, game_id).await?;

    let now = time::OffsetDateTime::now_utc().unix_timestamp();
    let in_game =
        cds_db::util::is_user_in_game(&s.db.conn, operator.id, game.id, Some(TState::Passed))
            .await?;

    if !in_game || !(game.started_at..=game.ended_at).contains(&now) {
        return Err(WebError::Forbidden(json!("")));
    }

    let (game_challenges, total) = cds_db::game_challenge::find(
        &s.db.conn,
        FindGameChallengeOptions {
            game_id: Some(game.id),
            challenge_id: params.challenge_id,
            enabled: Some(true),
            category: params.category,
        },
    )
    .await?;

    Ok(Json(GameChallengesListResponse {
        challenges: game_challenges,
        total,
    }))
}
