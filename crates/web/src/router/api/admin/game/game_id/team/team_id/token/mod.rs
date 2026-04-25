//! HTTP routing for `token` — Axum router wiring and OpenAPI route
//! registration.

use std::sync::Arc;

use axum::{Json, Router, extract::State};
use nanoid::nanoid;
use utoipa_axum::{
    router::{OpenApiRouter, UtoipaMethodRouterExt},
    routes,
};

use crate::{
    extract::Path,
    router::api::game::game_id::team::us::token::InviteTokenResponse,
    traits::{AppState, WebError},
};

/// Builds the Axum router fragment for this module.

pub fn router(state: Arc<AppState>) -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::from(Router::new().with_state(state.clone()))
        .routes(routes!(create_token).with_state(state.clone()))
        .routes(routes!(get_token).with_state(state.clone()))
        .routes(routes!(delete_token).with_state(state.clone()))
}

/// Creates token.
#[utoipa::path(
    post,
    path = "/",
    tag = "admin-game",
    params(
        ("game_id" = i64, Path, description = "Game id"),
        ("team_id" = i64, Path, description = "Team id"),
    ),
    responses(
        (status = 200, description = "Token created", body = InviteTokenResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
#[tracing::instrument(skip_all, fields(handler = "create_token"))]
pub async fn create_token(
    State(s): State<Arc<AppState>>,
    Path((game_id, team_id)): Path<(i64, i64)>,
) -> Result<Json<InviteTokenResponse>, WebError> {
    let team = crate::util::loader::prepare_team(&s.db.conn, game_id, team_id).await?;

    let token = nanoid!(16);
    s.cache
        .set_ex(format!("team:{}:invite", team.id), token.clone(), 60 * 60)
        .await?;

    Ok(Json(InviteTokenResponse { token: Some(token) }))
}

/// Returns token.
#[utoipa::path(
    get,
    path = "/",
    tag = "admin-game",
    params(
        ("game_id" = i64, Path, description = "Game id"),
        ("team_id" = i64, Path, description = "Team id"),
    ),
    responses(
        (status = 200, description = "Current token", body = InviteTokenResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
#[tracing::instrument(skip_all, fields(handler = "get_token"))]
pub async fn get_token(
    State(s): State<Arc<AppState>>,
    Path((game_id, team_id)): Path<(i64, i64)>,
) -> Result<Json<InviteTokenResponse>, WebError> {
    let team = crate::util::loader::prepare_team(&s.db.conn, game_id, team_id).await?;
    let token = s
        .cache
        .get::<String>(format!("team:{}:invite", team.id))
        .await?;

    Ok(Json(InviteTokenResponse { token }))
}

/// Deletes token.
#[utoipa::path(
    delete,
    path = "/",
    tag = "admin-game",
    params(
        ("game_id" = i64, Path, description = "Game id"),
        ("team_id" = i64, Path, description = "Team id"),
    ),
    responses(
        (status = 200, description = "Token removed", body = InviteTokenResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
#[tracing::instrument(skip_all, fields(handler = "delete_token"))]
pub async fn delete_token(
    State(s): State<Arc<AppState>>,
    Path((game_id, team_id)): Path<(i64, i64)>,
) -> Result<Json<InviteTokenResponse>, WebError> {
    let team = crate::util::loader::prepare_team(&s.db.conn, game_id, team_id).await?;
    let token = s
        .cache
        .get_del::<String>(format!("team:{}:invite", team.id))
        .await?;

    Ok(Json(InviteTokenResponse { token }))
}
