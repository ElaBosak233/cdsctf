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
    tag = "management-game",
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
        .set_with_ttl(
            format!("team:{}:invite", team.id),
            token.clone(),
            std::time::Duration::from_secs(60 * 60),
        )
        .await?;

    tracing::info!(
        resource = "team_invite",
        action = "create",
        team_id = team.id,
        game_id,
        expires_in_seconds = ?3600,
        "resource step completed",
    );
    Ok(Json(InviteTokenResponse { token: Some(token) }))
}

/// Returns token.
#[utoipa::path(
    get,
    path = "/",
    tag = "management-game",
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
    tag = "management-game",
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
        .take::<String>(format!("team:{}:invite", team.id))
        .await?;

    tracing::info!(
        resource = "team_invite",
        action = "revoke",
        team_id = team.id,
        game_id = game_id,
        existed = token.is_some(),
        "resource step completed"
    );
    Ok(Json(InviteTokenResponse { token }))
}
