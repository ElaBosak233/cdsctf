//! HTTP routing for `team_id` — Axum router wiring and OpenAPI route
//! registration.

/// Defines the `token` submodule (see sibling `*.rs` files).
mod token;

/// Defines the `user` submodule (see sibling `*.rs` files).
mod user;

/// Defines the `writeup` submodule (see sibling `*.rs` files).
mod writeup;

use std::sync::Arc;

use axum::{Json, Router, extract::State};
use cds_db::{
    TeamView,
    sea_orm::{
        ActiveValue::{Set, Unchanged},
        NotSet, TransactionTrait,
    },
    team::State as TState,
};
use cds_worker::calculator;
use serde::{Deserialize, Serialize};
use serde_json::json;
use utoipa_axum::{
    router::{OpenApiRouter, UtoipaMethodRouterExt},
    routes,
};

use crate::{
    extract::{Json as ReqJson, Path},
    router::api::admin::game::game_id::team::AdminTeamResponse,
    traits::{AppState, EmptyJson, WebError},
};

/// Builds the Axum router fragment for this module.

pub fn router(state: Arc<AppState>) -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::from(Router::new().with_state(state.clone()))
        .routes(routes!(update_team).with_state(state.clone()))
        .routes(routes!(delete_team).with_state(state.clone()))
        .nest("/users", user::router(state.clone()))
        .nest("/token", token::router(state.clone()))
        .nest("/writeup", writeup::router(state.clone()))
}

#[derive(Clone, Debug, Serialize, Deserialize, utoipa::ToSchema)]
pub struct UpdateTeamRequest {
    pub name: Option<String>,
    pub email: Option<String>,
    pub state: Option<TState>,
    pub slogan: Option<String>,
    pub description: Option<String>,
}

/// Updates team.
#[utoipa::path(
    put,
    path = "/",
    tag = "admin-game",
    params(
        ("game_id" = i64, Path, description = "Game id"),
        ("team_id" = i64, Path, description = "Team id"),
    ),
    request_body = UpdateTeamRequest,
    responses(
        (status = 200, description = "Updated team", body = AdminTeamResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
#[tracing::instrument(skip_all, fields(handler = "update_team"))]
pub async fn update_team(
    State(s): State<Arc<AppState>>,
    Path((game_id, team_id)): Path<(i64, i64)>,
    ReqJson(body): ReqJson<UpdateTeamRequest>,
) -> Result<Json<AdminTeamResponse>, WebError> {
    let team = crate::util::loader::prepare_team(&s.db.conn, game_id, team_id).await?;

    let transaction = s.db.conn.begin().await.map_err(cds_db::DbError::from)?;
    let new_team = cds_db::team::update::<TeamView>(
        &transaction,
        cds_db::team::ActiveModel {
            id: Unchanged(team.id),
            game_id: Unchanged(team.game_id),
            name: body.name.map_or(NotSet, Set),
            state: body.state.map_or(NotSet, Set),
            slogan: body.slogan.map_or(NotSet, |v| Set(Some(v))),
            email: body.email.map_or(NotSet, |v| Set(Some(v))),
            ..Default::default()
        },
    )
    .await?;

    let score_changed = team.state != new_team.state;
    if score_changed {
        cds_db::game::request_score_recalculation(&transaction, game_id).await?;
    }
    transaction.commit().await.map_err(cds_db::DbError::from)?;

    if score_changed {
        calculator::notify(&s.queue, game_id).await;
    }

    Ok(Json(AdminTeamResponse { team: new_team }))
}

/// Deletes team.
#[utoipa::path(
    delete,
    path = "/",
    tag = "admin-game",
    params(
        ("game_id" = i64, Path, description = "Game id"),
        ("team_id" = i64, Path, description = "Team id"),
    ),
    responses(
        (status = 200, description = "Deleted", body = EmptyJson),
        (status = 400, description = "Bad request", body = crate::traits::ErrorResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
#[tracing::instrument(skip_all, fields(handler = "delete_team"))]
pub async fn delete_team(
    State(s): State<Arc<AppState>>,
    Path((game_id, team_id)): Path<(i64, i64)>,
) -> Result<Json<EmptyJson>, WebError> {
    let team = crate::util::loader::prepare_team(&s.db.conn, game_id, team_id).await?;

    if team.state != TState::Preparing {
        return Err(WebError::BadRequest(json!("team_not_preparing")));
    }

    cds_db::team_user::delete_by_team_id(&s.db.conn, team.id).await?;

    cds_db::team::delete(&s.db.conn, team.id).await?;

    Ok(Json(EmptyJson::default()))
}
