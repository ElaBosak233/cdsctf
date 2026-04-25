//! HTTP routing for `us` — Axum router wiring and OpenAPI route registration.

/// Defines the `avatar` submodule (see sibling `*.rs` files).
mod avatar;

/// Defines the `token` submodule (see sibling `*.rs` files).
pub mod token;

/// Defines the `user` submodule (see sibling `*.rs` files).
mod user;

/// Defines the `writeup` submodule (see sibling `*.rs` files).
mod writeup;

use std::sync::Arc;

use axum::{Json, Router, extract::State};
use cds_db::{
    TeamUser,
    sea_orm::{
        ActiveValue::{Set, Unchanged},
        NotSet,
    },
    team::State as TState,
    team_user::FindTeamUserOptions,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use utoipa_axum::{
    router::{OpenApiRouter, UtoipaMethodRouterExt},
    routes,
};

use super::TeamResponse;
use crate::{
    extract::{Extension, Json as ReqJson, Path},
    traits::{AppState, AuthPrincipal, EmptyJson, WebError},
};

/// Builds the Axum router fragment for this module.

pub fn router(state: Arc<AppState>) -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::from(Router::new().with_state(state.clone()))
        .routes(routes!(get_team).with_state(state.clone()))
        .routes(routes!(update_team).with_state(state.clone()))
        .routes(routes!(delete_team).with_state(state.clone()))
        .routes(routes!(set_team_ready).with_state(state.clone()))
        .nest("/avatar", avatar::router(state.clone()))
        .nest("/users", user::router(state.clone()))
        .nest("/token", token::router(state.clone()))
        .nest("/writeup", writeup::router(state.clone()))
}

/// Returns team.
#[utoipa::path(
    get,
    path = "/",
    tag = "game",
    params(
        ("game_id" = i64, Path, description = "Game id"),
    ),
    responses(
        (status = 200, description = "Current team", body = TeamResponse),
        (status = 401, description = "Unauthorized", body = crate::traits::ErrorResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
#[tracing::instrument(skip_all, fields(handler = "get_team"))]
pub async fn get_team(
    State(s): State<Arc<AppState>>,
    Extension(ext): Extension<AuthPrincipal>,
    Path(game_id): Path<i64>,
) -> Result<Json<TeamResponse>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    let team = crate::util::loader::prepare_self_team(&s.db.conn, game_id, operator.id).await?;

    Ok(Json(TeamResponse { team }))
}

#[derive(Clone, Debug, Serialize, Deserialize, utoipa::ToSchema)]
pub struct UpdateTeamRequest {
    pub name: Option<String>,
    pub email: Option<String>,
    pub slogan: Option<String>,
    pub description: Option<String>,
}

/// Updates team.
#[utoipa::path(
    put,
    path = "/",
    tag = "game",
    params(
        ("game_id" = i64, Path, description = "Game id"),
    ),
    request_body = UpdateTeamRequest,
    responses(
        (status = 200, description = "Updated team", body = TeamResponse),
        (status = 401, description = "Unauthorized", body = crate::traits::ErrorResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
#[tracing::instrument(skip_all, fields(handler = "update_team"))]
pub async fn update_team(
    State(s): State<Arc<AppState>>,
    Extension(ext): Extension<AuthPrincipal>,
    Path(game_id): Path<i64>,
    ReqJson(body): ReqJson<UpdateTeamRequest>,
) -> Result<Json<TeamResponse>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;

    let team = crate::util::loader::prepare_self_team(&s.db.conn, game_id, operator.id).await?;

    let team = cds_db::team::update(
        &s.db.conn,
        cds_db::team::ActiveModel {
            id: Unchanged(team.id),
            game_id: Unchanged(team.game_id),
            name: body.name.map_or(NotSet, Set),
            slogan: body.slogan.map_or(NotSet, |v| Set(Some(v))),
            email: body.email.map_or(NotSet, |v| Set(Some(v))),
            ..Default::default()
        },
    )
    .await?;

    Ok(Json(TeamResponse { team }))
}

/// Deletes team.
#[utoipa::path(
    delete,
    path = "/",
    tag = "game",
    params(
        ("game_id" = i64, Path, description = "Game id"),
    ),
    responses(
        (status = 200, description = "Team deleted", body = EmptyJson),
        (status = 400, description = "Bad request", body = crate::traits::ErrorResponse),
        (status = 401, description = "Unauthorized", body = crate::traits::ErrorResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
#[tracing::instrument(skip_all, fields(handler = "delete_team"))]
pub async fn delete_team(
    State(s): State<Arc<AppState>>,
    Extension(ext): Extension<AuthPrincipal>,
    Path(game_id): Path<i64>,
) -> Result<Json<EmptyJson>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    let team = crate::util::loader::prepare_self_team(&s.db.conn, game_id, operator.id).await?;

    if team.state != TState::Preparing {
        return Err(WebError::BadRequest(json!("team_not_preparing")));
    }

    cds_db::team_user::delete_by_team_id(&s.db.conn, team.id).await?;

    cds_db::team::delete(&s.db.conn, team.id).await?;

    Ok(Json(EmptyJson::default()))
}

/// Updates team ready.
#[utoipa::path(
    post,
    path = "/ready",
    tag = "game",
    params(
        ("game_id" = i64, Path, description = "Game id"),
    ),
    responses(
        (status = 200, description = "Team ready", body = TeamResponse),
        (status = 400, description = "Bad request", body = crate::traits::ErrorResponse),
        (status = 401, description = "Unauthorized", body = crate::traits::ErrorResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
#[tracing::instrument(skip_all, fields(handler = "set_team_ready"))]
pub async fn set_team_ready(
    State(s): State<Arc<AppState>>,
    Extension(ext): Extension<AuthPrincipal>,
    Path(game_id): Path<i64>,
) -> Result<Json<TeamResponse>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    let game = crate::util::loader::prepare_game(&s.db.conn, game_id).await?;
    let team = crate::util::loader::prepare_self_team(&s.db.conn, game_id, operator.id).await?;

    if team.state != TState::Preparing {
        return Err(WebError::BadRequest(json!("team_not_preparing")));
    }

    let (_, team_users) = cds_db::team_user::find::<TeamUser>(
        &s.db.conn,
        FindTeamUserOptions {
            team_id: Some(team.id),
            ..Default::default()
        },
    )
    .await?;

    if team_users < game.member_limit_min as u64 || team_users > game.member_limit_max as u64 {
        return Err(WebError::BadRequest(json!("member_limit_not_satisfied")));
    }

    let team = cds_db::team::update(
        &s.db.conn,
        cds_db::team::ActiveModel {
            id: Unchanged(team.id),
            game_id: Unchanged(team.game_id),
            state: Set(if game.public {
                TState::Passed
            } else {
                TState::Pending
            }),
            ..Default::default()
        },
    )
    .await?;

    Ok(Json(TeamResponse { team }))
}
