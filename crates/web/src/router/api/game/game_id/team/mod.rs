//! HTTP routing for `team` — Axum router wiring and OpenAPI route registration.

/// Defines the `team_id` submodule (see sibling `*.rs` files).
mod team_id;

/// Defines the `us` submodule (see sibling `*.rs` files).
pub mod us;

use std::sync::Arc;

use axum::{Json, Router, extract::State, http::StatusCode};
use cds_db::{
    TeamUser,
    sea_orm::ActiveValue::Set,
    team::{State as TState, Team},
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use utoipa_axum::{
    router::{OpenApiRouter, UtoipaMethodRouterExt},
    routes,
};

use crate::{
    extract::{Extension, Json as ReqJson, Path},
    traits::{AppState, AuthPrincipal, WebError},
};

/// Builds the Axum router fragment for this module.

pub fn router(state: Arc<AppState>) -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::from(Router::new().with_state(state.clone()))
        .routes(routes!(create_team).with_state(state.clone()))
        .nest("/us", us::router(state.clone()))
        .nest("/{team_id}", team_id::router(state.clone()))
}

#[derive(Clone, Debug, Serialize, utoipa::ToSchema)]
pub struct TeamResponse {
    pub team: Team,
}

#[derive(Clone, Debug, Serialize, Deserialize, utoipa::ToSchema)]
pub struct CreateTeamRequest {
    pub name: String,
    pub email: Option<String>,
    pub slogan: Option<String>,
    pub description: Option<String>,
}

/// Creates a team in the given game (collection `POST`).
#[utoipa::path(
    post,
    path = "/",
    tag = "game",
    params(
        ("game_id" = i64, Path, description = "Game id"),
    ),
    request_body = CreateTeamRequest,
    responses(
        (status = 201, description = "Team created", body = TeamResponse),
        (status = 400, description = "Bad request", body = crate::traits::ErrorResponse),
        (status = 401, description = "Unauthorized", body = crate::traits::ErrorResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
#[tracing::instrument(skip_all, fields(handler = "create_team"))]
pub async fn create_team(
    State(s): State<Arc<AppState>>,
    Extension(ext): Extension<AuthPrincipal>,
    Path(game_id): Path<i64>,
    ReqJson(body): ReqJson<CreateTeamRequest>,
) -> Result<(StatusCode, Json<TeamResponse>), WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;

    let game = crate::util::loader::prepare_game(&s.db.conn, game_id).await?;

    if cds_db::util::is_user_in_game(&s.db.conn, operator.id, game.id, None).await? {
        return Err(WebError::BadRequest(json!("user_already_in_game")));
    }

    let team = cds_db::team::create::<Team>(
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

    let _ = cds_db::team_user::create::<TeamUser>(
        &s.db.conn,
        cds_db::team_user::ActiveModel {
            team_id: Set(team.id),
            user_id: Set(operator.id),
        },
    )
    .await?;

    let team = cds_db::team::find_by_id(&s.db.conn, team.id, team.game_id)
        .await?
        .ok_or(WebError::NotFound(json!("")))?;

    Ok((StatusCode::CREATED, Json(TeamResponse { team })))
}
