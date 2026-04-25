//! HTTP routing for `user` — Axum router wiring and OpenAPI route registration.

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
    extract::{Json as ReqJson, Path},
    traits::{AppState, EmptyJson, WebError},
};

/// Builds the Axum router fragment for this module.

pub fn router(state: Arc<AppState>) -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::from(Router::new().with_state(state.clone()))
        .routes(routes!(get_team_user).with_state(state.clone()))
        .routes(routes!(create_team_user).with_state(state.clone()))
        .routes(routes!(delete_team_user).with_state(state.clone()))
}

#[derive(Clone, Debug, Serialize, utoipa::ToSchema)]
pub struct AdminTeamUsersListResponse {
    pub users: Vec<UserMini>,
    pub total: u64,
}

/// Returns team user.
#[utoipa::path(
    get,
    path = "/",
    tag = "admin-game",
    params(
        ("game_id" = i64, Path, description = "Game id"),
        ("team_id" = i64, Path, description = "Team id"),
    ),
    responses(
        (status = 200, description = "Members", body = AdminTeamUsersListResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
#[tracing::instrument(skip_all, fields(handler = "get_team_user"))]
pub async fn get_team_user(
    State(s): State<Arc<AppState>>,
    Path((game_id, team_id)): Path<(i64, i64)>,
) -> Result<Json<AdminTeamUsersListResponse>, WebError> {
    let team = crate::util::loader::prepare_team(&s.db.conn, game_id, team_id).await?;

    let team_users = cds_db::user::find_by_team_id(&s.db.conn, team.id).await?;
    let total = team_users.len() as u64;

    Ok(Json(AdminTeamUsersListResponse {
        users: team_users,
        total,
    }))
}

#[derive(Clone, Debug, Serialize, Deserialize, utoipa::ToSchema)]
pub struct CreateTeamUserRequest {
    pub user_id: i64,
}

/// Creates team user.
#[utoipa::path(
    post,
    path = "/",
    tag = "admin-game",
    params(
        ("game_id" = i64, Path, description = "Game id"),
        ("team_id" = i64, Path, description = "Team id"),
    ),
    request_body = CreateTeamUserRequest,
    responses(
        (status = 200, description = "User added", body = EmptyJson),
        (status = 400, description = "Bad request", body = crate::traits::ErrorResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
#[tracing::instrument(skip_all, fields(handler = "create_team_user"))]
pub async fn create_team_user(
    State(s): State<Arc<AppState>>,
    Path((game_id, team_id)): Path<(i64, i64)>,
    ReqJson(body): ReqJson<CreateTeamUserRequest>,
) -> Result<Json<EmptyJson>, WebError> {
    let user = crate::util::loader::prepare_user(&s.db.conn, body.user_id).await?;
    let game = crate::util::loader::prepare_game(&s.db.conn, game_id).await?;
    let team = crate::util::loader::prepare_team(&s.db.conn, game_id, team_id).await?;

    if team.state != TState::Preparing {
        return Err(WebError::BadRequest(json!("team_not_preparing")));
    }

    if cds_db::util::is_user_in_game(&s.db.conn, user.id, game.id, None).await? {
        return Err(WebError::BadRequest(json!("user_already_in_game")));
    }

    let _ = cds_db::team_user::create::<TeamUser>(
        &s.db.conn,
        cds_db::team_user::ActiveModel {
            user_id: Set(body.user_id),
            team_id: Set(team.id),
        },
    )
    .await?;

    Ok(Json(EmptyJson::default()))
}

/// Deletes team user.
#[utoipa::path(
    delete,
    path = "/{user_id}",
    tag = "admin-game",
    params(
        ("game_id" = i64, Path, description = "Game id"),
        ("team_id" = i64, Path, description = "Team id"),
        ("user_id" = i64, Path, description = "User id"),
    ),
    responses(
        (status = 200, description = "Kicked", body = EmptyJson),
        (status = 400, description = "Bad request", body = crate::traits::ErrorResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
#[tracing::instrument(skip_all, fields(handler = "delete_team_user"))]
pub async fn delete_team_user(
    State(s): State<Arc<AppState>>,
    Path((game_id, team_id, user_id)): Path<(i64, i64, i64)>,
) -> Result<Json<EmptyJson>, WebError> {
    let team = crate::util::loader::prepare_team(&s.db.conn, game_id, team_id).await?;

    if team.state != TState::Preparing {
        return Err(WebError::BadRequest(json!("team_not_preparing")));
    }

    cds_db::team_user::delete(&s.db.conn, team_id, user_id).await?;

    Ok(Json(EmptyJson::default()))
}
