//! HTTP routing for `team` — Axum router wiring and OpenAPI route registration.

/// Defines the `team_id` submodule (see sibling `*.rs` files).
mod team_id;

use std::sync::Arc;

use axum::{Json, Router, extract::State};
use cds_db::{
    sea_orm::ActiveValue::Set,
    team::{FindTeamOptions, State as TState, Team},
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use utoipa_axum::{
    router::{OpenApiRouter, UtoipaMethodRouterExt},
    routes,
};

use crate::{
    extract::{Extension, Json as ReqJson, Path, Query},
    router::api::game::game_id::team::TeamResponse,
    traits::{AppState, AuthPrincipal, WebError},
};

/// Builds the Axum router fragment for this module.

pub fn router(state: Arc<AppState>) -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::from(Router::new().with_state(state.clone()))
        .routes(routes!(get_team).with_state(state.clone()))
        .routes(routes!(create_team).with_state(state.clone()))
        .nest("/{team_id}", team_id::router(state.clone()))
}

#[derive(Clone, Debug, Serialize, Deserialize, utoipa::ToSchema, utoipa::IntoParams)]
#[into_params(parameter_in = Query)]
pub struct GetTeamRequest {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub state: Option<TState>,
    pub has_writeup: Option<bool>,
    pub user_id: Option<i64>,
    pub page: Option<u64>,
    pub size: Option<u64>,
    pub sorts: Option<String>,
}

#[derive(Clone, Debug, Serialize, utoipa::ToSchema)]
pub struct AdminTeamsListResponse {
    pub teams: Vec<Team>,
    pub total: u64,
}

#[utoipa::path(
    get,
    path = "/",
    tag = "admin-game",
    params(
        ("game_id" = i64, Path, description = "Game id"),
        GetTeamRequest,
    ),
    responses(
        (status = 200, description = "Teams", body = AdminTeamsListResponse),
        (status = 401, description = "Unauthorized", body = crate::traits::ErrorResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]

/// Returns team.
pub async fn get_team(
    State(s): State<Arc<AppState>>,
    Extension(ext): Extension<AuthPrincipal>,
    Path(game_id): Path<i64>,
    Query(params): Query<GetTeamRequest>,
) -> Result<Json<AdminTeamsListResponse>, WebError> {
    let _ = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;

    let (teams, total) = cds_db::team::find(
        &s.db.conn,
        FindTeamOptions {
            id: params.id,
            name: params.name,
            state: params.state,
            has_writeup: params.has_writeup,
            game_id: Some(game_id),
            user_id: params.user_id,
            page: params.page,
            size: params.size,
            sorts: params.sorts,
        },
    )
    .await?;

    Ok(Json(AdminTeamsListResponse {
        teams,
        total,
    }))
}

#[derive(Clone, Debug, Serialize, Deserialize, utoipa::ToSchema)]
pub struct CreateTeamRequest {
    pub name: String,
    pub email: Option<String>,
    pub slogan: Option<String>,
}

#[utoipa::path(
    post,
    path = "/",
    tag = "admin-game",
    params(
        ("game_id" = i64, Path, description = "Game id"),
    ),
    request_body = CreateTeamRequest,
    responses(
        (status = 200, description = "Team created", body = TeamResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]

/// Creates team.
pub async fn create_team(
    State(s): State<Arc<AppState>>,
    Path(game_id): Path<i64>,
    ReqJson(body): ReqJson<CreateTeamRequest>,
) -> Result<Json<TeamResponse>, WebError> {
    let game = crate::util::loader::prepare_game(&s.db.conn, game_id).await?;

    let team = cds_db::team::create(
        &s.db.conn,
        cds_db::team::ActiveModel {
            name: Set(body.name),
            email: Set(body.email),
            slogan: Set(body.slogan),
            game_id: Set(game.id),
            state: Set(TState::Preparing),
            ..Default::default()
        },
    )
    .await?;

    Ok(Json(TeamResponse { team }))
}
