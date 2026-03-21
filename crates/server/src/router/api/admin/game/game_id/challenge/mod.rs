use std::sync::Arc;

use axum::{Json, Router, extract::State};
use cds_db::{
    GameChallenge,
    game_challenge::FindGameChallengeOptions,
    sea_orm::{ActiveValue::Set, NotSet},
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use utoipa_axum::{
    router::{OpenApiRouter, UtoipaMethodRouterExt},
    routes,
};

use crate::{
    extract::{Json as ReqJson, Path, Query},
    traits::{AppState, WebError},
};

mod challenge_id;


pub fn openapi_router(state: Arc<AppState>) -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::from(Router::new().with_state(state.clone()))
        .routes(routes!(get_game_challenge).with_state(state.clone()))
        .routes(routes!(create_game_challenge).with_state(state.clone()))
        .nest(
            "/{challenge_id}",
            challenge_id::openapi_router(state.clone()),
        )
}

#[derive(Clone, Debug, Serialize, Deserialize, utoipa::ToSchema, utoipa::IntoParams)]
#[into_params(parameter_in = Query)]
pub struct GetGameChallengeRequest {
    pub game_id: Option<i64>,
    pub challenge_id: Option<i64>,
    pub category: Option<i32>,
}

#[derive(Clone, Debug, Serialize, utoipa::ToSchema)]
pub struct AdminGameChallengesListResponse {
    pub items: Vec<GameChallenge>,
    pub total: u64,
}

#[utoipa::path(
    get,
    path = "/",
    tag = "admin-game",
    params(
        ("game_id" = i64, Path, description = "Game id"),
        GetGameChallengeRequest,
    ),
    responses(
        (status = 200, description = "Game challenges", body = AdminGameChallengesListResponse),
        (status = 500, description = "Server error", body = crate::traits::ApiJsonError),
    )
)]
pub async fn get_game_challenge(
    State(s): State<Arc<AppState>>,
    Path(game_id): Path<i64>,
    Query(params): Query<GetGameChallengeRequest>,
) -> Result<Json<AdminGameChallengesListResponse>, WebError> {
    let (game_challenges, total) = cds_db::game_challenge::find(
        &s.db.conn,
        FindGameChallengeOptions {
            game_id: Some(game_id),
            challenge_id: params.challenge_id,
            category: params.category,
            ..Default::default()
        },
    )
    .await?;

    Ok(Json(AdminGameChallengesListResponse {
        items: game_challenges,
        total,
    }))
}

#[derive(Clone, Debug, Serialize, Deserialize, utoipa::ToSchema)]
pub struct CreateGameChallengeRequest {
    pub challenge_id: i64,
    pub enabled: Option<bool>,
    pub difficulty: Option<i64>,
    pub max_pts: Option<i64>,
    pub min_pts: Option<i64>,
    pub bonus_ratios: Option<Vec<i64>>,
    pub frozen_at: Option<Option<i64>>,
}

#[derive(Clone, Debug, Serialize, utoipa::ToSchema)]
pub struct GameChallengeResponse {
    pub game_challenge: GameChallenge,
}

#[utoipa::path(
    post,
    path = "/",
    tag = "admin-game",
    params(
        ("game_id" = i64, Path, description = "Game id"),
    ),
    request_body = CreateGameChallengeRequest,
    responses(
        (status = 200, description = "Linked challenge", body = GameChallengeResponse),
        (status = 409, description = "Conflict", body = crate::traits::ApiJsonError),
        (status = 500, description = "Server error", body = crate::traits::ApiJsonError),
    )
)]
pub async fn create_game_challenge(
    State(s): State<Arc<AppState>>,
    Path(game_id): Path<i64>,
    ReqJson(body): ReqJson<CreateGameChallengeRequest>,
) -> Result<Json<GameChallengeResponse>, WebError> {
    let game = crate::util::loader::prepare_game(&s.db.conn, game_id).await?;

    let challenge = crate::util::loader::prepare_challenge(&s.db.conn, body.challenge_id).await?;

    if cds_db::util::is_challenge_in_game(&s.db.conn, challenge.id, game.id).await? {
        return Err(WebError::Conflict(json!("challenge_already_in_game")));
    }

    let game_challenge = cds_db::game_challenge::create(
        &s.db.conn,
        cds_db::game_challenge::ActiveModel {
            game_id: Set(game.id),
            challenge_id: Set(challenge.id),
            difficulty: body.difficulty.map_or(NotSet, Set),
            enabled: body.enabled.map_or(Set(false), Set),
            max_pts: body.max_pts.map_or(NotSet, Set),
            min_pts: body.min_pts.map_or(NotSet, Set),
            bonus_ratios: body.bonus_ratios.map_or(Set(vec![]), Set),
            frozen_at: body.frozen_at.map_or(NotSet, Set),
            ..Default::default()
        },
    )
    .await?;

    s.queue
        .publish(
            "calculator",
            crate::worker::game_calculator::Payload {
                game_id: Some(game.id),
            },
        )
        .await?;

    Ok(Json(GameChallengeResponse { game_challenge }))
}
