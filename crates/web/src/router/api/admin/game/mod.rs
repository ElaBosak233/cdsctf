//! HTTP routing for `game` — Axum router wiring and OpenAPI route registration.

/// Defines the `game_id` submodule (see sibling `*.rs` files).
mod game_id;

use std::sync::Arc;

use axum::{Json, Router, extract::State, http::StatusCode};
use cds_db::{
    Game,
    game::FindGameOptions,
    sea_orm::ActiveValue::{NotSet, Set},
};
use serde::{Deserialize, Serialize};
use tracing::info;
use utoipa_axum::{
    router::{OpenApiRouter, UtoipaMethodRouterExt},
    routes,
};
use validator::Validate;

use crate::{
    extract::{Query, VJson},
    router::api::game::game_id::GameDetailResponse,
    traits::{AppState, WebError},
};

/// Builds the Axum router fragment for this module.

pub fn router(state: Arc<AppState>) -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::from(Router::new().with_state(state.clone()))
        .routes(routes!(get_games).with_state(state.clone()))
        .routes(routes!(create_game).with_state(state.clone()))
        .nest("/{game_id}", game_id::router(state.clone()))
}

#[derive(Clone, Debug, Serialize, Deserialize, utoipa::ToSchema, utoipa::IntoParams)]
#[into_params(parameter_in = Query)]
pub struct GetGameRequest {
    pub id: Option<i64>,
    pub title: Option<String>,
    pub enabled: Option<bool>,
    pub page: Option<u64>,
    pub size: Option<u64>,
    pub sorts: Option<String>,
}

#[derive(Clone, Debug, Serialize, utoipa::ToSchema)]
pub struct AdminGamesListResponse {
    pub games: Vec<Game>,
    pub total: u64,
}

/// Returns games.
#[utoipa::path(
    get,
    path = "/",
    tag = "admin-game",
    params(GetGameRequest),
    responses(
        (status = 200, description = "Games", body = AdminGamesListResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
#[tracing::instrument(skip_all, fields(handler = "get_games"))]
pub async fn get_games(
    State(s): State<Arc<AppState>>,
    Query(params): Query<GetGameRequest>,
) -> Result<Json<AdminGamesListResponse>, WebError> {
    let page = params.page.unwrap_or(1);
    let size = params.size.unwrap_or(10).min(100);

    let (games, total) = cds_db::game::find(
        &s.db.conn,
        FindGameOptions {
            id: params.id,
            title: params.title,
            enabled: params.enabled,
            page: Some(page),
            size: Some(size),
            sorts: params.sorts,
        },
    )
    .await?;

    Ok(Json(AdminGamesListResponse { games, total }))
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate, utoipa::ToSchema)]
pub struct CreateGameRequest {
    pub title: String,
    pub sketch: Option<String>,
    pub description: Option<String>,
    pub enabled: Option<bool>,
    pub public: Option<bool>,
    pub writeup_required: Option<bool>,
    pub member_limit_min: Option<i64>,
    pub member_limit_max: Option<i64>,
    pub timeslots: Option<Vec<cds_db::game::Timeslot>>,
    pub started_at: i64,
    pub ended_at: i64,
}

/// Creates game.
#[utoipa::path(
    post,
    path = "/",
    tag = "admin-game",
    request_body = CreateGameRequest,
    responses(
        (status = 201, description = "Created game", body = GameDetailResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
#[tracing::instrument(skip_all, fields(handler = "create_game"))]
pub async fn create_game(
    State(s): State<Arc<AppState>>,
    VJson(body): VJson<CreateGameRequest>,
) -> Result<(StatusCode, Json<GameDetailResponse>), WebError> {
    let game = cds_db::game::create::<Game>(
        &s.db.conn,
        cds_db::game::ActiveModel {
            title: Set(body.title),
            sketch: Set(body.sketch),
            description: Set(body.description),

            enabled: Set(body.enabled.unwrap_or(false)),
            public: Set(body.public.unwrap_or(false)),
            writeup_required: Set(body.writeup_required.unwrap_or(false)),

            member_limit_min: body.member_limit_min.map_or(NotSet, Set),
            member_limit_max: body.member_limit_max.map_or(NotSet, Set),

            timeslots: Set(body.timeslots.unwrap_or(vec![])),
            started_at: Set(body.started_at),
            ended_at: Set(body.ended_at),
            frozen_at: Set(body.ended_at),
            ..Default::default()
        },
    )
    .await?;
    info!(
        game_id = game.id,
        title = %game.title,
        enabled = game.enabled,
        public = game.public,
        "admin created game"
    );

    Ok((StatusCode::CREATED, Json(GameDetailResponse { game })))
}
