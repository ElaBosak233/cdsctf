//! HTTP routing for `team_id` — Axum router wiring and OpenAPI route
//! registration.

/// Defines the `avatar` submodule (see sibling `*.rs` files).
mod avatar;

use std::sync::Arc;

use axum::{Json, Router, extract::State};
use cds_db::{TeamUser, UserMini, sea_orm::ActiveValue::Set, team::State as TState};
use serde::{Deserialize, Serialize};
use serde_json::json;
use utoipa_axum::{
    router::{OpenApiRouter, UtoipaMethodRouterExt},
    routes,
};

use crate::{
    extract::{Extension, Json as ReqJson, Path},
    traits::{AppState, AuthPrincipal, EmptyJson, WebError},
};

/// Builds the Axum router fragment for this module.

pub fn router(state: Arc<AppState>) -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::from(Router::new().with_state(state.clone()))
        .nest(
            "/avatar",
            OpenApiRouter::from(Router::new().with_state(state.clone()))
                .routes(routes!(avatar::get_team_avatar).with_state(state.clone())),
        )
        .routes(routes!(get_team_members).with_state(state.clone()))
        .routes(routes!(join_team).with_state(state.clone()))
}

#[derive(Clone, Debug, Serialize, utoipa::ToSchema)]
pub struct TeamMembersListResponse {
    pub items: Vec<UserMini>,
    pub total: u64,
}

#[utoipa::path(
    get,
    path = "/members",
    tag = "game",
    params(
        ("game_id" = i64, Path, description = "Game id"),
        ("team_id" = i64, Path, description = "Team id"),
    ),
    responses(
        (status = 200, description = "Members", body = TeamMembersListResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]

/// Returns team members.
pub async fn get_team_members(
    State(s): State<Arc<AppState>>,
    Path((_game_id, team_id)): Path<(i64, i64)>,
) -> Result<Json<TeamMembersListResponse>, WebError> {
    let users = cds_db::user::find_by_team_id(&s.db.conn, team_id).await?;
    let total = users.len() as u64;

    Ok(Json(TeamMembersListResponse {
        items: users,
        total,
    }))
}

#[derive(Clone, Debug, Serialize, Deserialize, utoipa::ToSchema)]
pub struct JoinTeamRequest {
    pub team_id: i64,
    pub token: String,
}

#[utoipa::path(
    post,
    path = "/join",
    tag = "game",
    params(
        ("game_id" = i64, Path, description = "Game id"),
        ("team_id" = i64, Path, description = "Team id"),
    ),
    request_body = JoinTeamRequest,
    responses(
        (status = 200, description = "Joined", body = EmptyJson),
        (status = 400, description = "Bad request", body = crate::traits::ErrorResponse),
        (status = 401, description = "Unauthorized", body = crate::traits::ErrorResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]

/// Adds the caller to a team within a game.
pub async fn join_team(
    State(s): State<Arc<AppState>>,
    Extension(ext): Extension<AuthPrincipal>,
    Path((game_id, team_id)): Path<(i64, i64)>,
    ReqJson(body): ReqJson<JoinTeamRequest>,
) -> Result<Json<EmptyJson>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    let game = crate::util::loader::prepare_game(&s.db.conn, game_id).await?;
    let team = crate::util::loader::prepare_team(&s.db.conn, game_id, team_id).await?;

    if team.state != TState::Preparing {
        return Err(WebError::BadRequest(json!("team_not_preparing")));
    }

    if cds_db::util::is_user_in_game(&s.db.conn, operator.id, game.id, None).await? {
        return Err(WebError::BadRequest(json!("user_already_in_game")));
    }

    if body.team_id != team.id {
        return Err(WebError::BadRequest(json!("invalid_team")));
    }

    let criteria = s
        .cache
        .get::<String>(format!("team:{}:invite", team.id))
        .await?
        .ok_or(WebError::BadRequest(json!("no_invite_token")))?;

    if criteria != body.token {
        return Err(WebError::BadRequest(json!("invalid_invite_token")));
    }

    let _ = cds_db::team_user::create::<TeamUser>(
        &s.db.conn,
        cds_db::team_user::ActiveModel {
            team_id: Set(team.id),
            user_id: Set(operator.id),
        },
    )
    .await?;

    Ok(Json(EmptyJson::default()))
}
