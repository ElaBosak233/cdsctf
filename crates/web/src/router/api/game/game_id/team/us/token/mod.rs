//! HTTP routing for `token` — Axum router wiring and OpenAPI route
//! registration.

use std::sync::Arc;

use axum::{Json, Router, extract::State};
use nanoid::nanoid;
use serde_json::json;
use utoipa_axum::{
    router::{OpenApiRouter, UtoipaMethodRouterExt},
    routes,
};

use crate::{
    extract::{Extension, Path},
    traits::{AppState, AuthPrincipal, WebError},
};

/// Builds the Axum router fragment for this module.

pub fn router(state: Arc<AppState>) -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::from(Router::new().with_state(state.clone()))
        .routes(routes!(create_token).with_state(state.clone()))
        .routes(routes!(get_token).with_state(state.clone()))
        .routes(routes!(delete_token).with_state(state.clone()))
}

#[derive(Clone, Debug, serde::Serialize, utoipa::ToSchema)]
pub struct InviteTokenResponse {
    pub token: Option<String>,
}

#[utoipa::path(
    post,
    path = "/",
    tag = "game",
    params(
        ("game_id" = i64, Path, description = "Game id"),
    ),
    responses(
        (status = 200, description = "New invite token", body = InviteTokenResponse),
        (status = 401, description = "Unauthorized", body = crate::traits::ErrorResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]

/// Creates token.
pub async fn create_token(
    State(s): State<Arc<AppState>>,
    Extension(ext): Extension<AuthPrincipal>,
    Path(game_id): Path<i64>,
) -> Result<Json<InviteTokenResponse>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    let team = crate::util::loader::prepare_self_team(&s.db.conn, game_id, operator.id).await?;

    let token = nanoid!(16);
    s.cache
        .set_ex(format!("team:{}:invite", team.id), token.clone(), 60 * 60)
        .await?;

    Ok(Json(InviteTokenResponse { token: Some(token) }))
}

#[utoipa::path(
    get,
    path = "/",
    tag = "game",
    params(
        ("game_id" = i64, Path, description = "Game id"),
    ),
    responses(
        (status = 200, description = "Current invite token if any", body = InviteTokenResponse),
        (status = 401, description = "Unauthorized", body = crate::traits::ErrorResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]

/// Returns token.
pub async fn get_token(
    State(s): State<Arc<AppState>>,
    Extension(ext): Extension<AuthPrincipal>,
    Path(game_id): Path<i64>,
) -> Result<Json<InviteTokenResponse>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    let team = crate::util::loader::prepare_self_team(&s.db.conn, game_id, operator.id).await?;
    let token = s
        .cache
        .get::<String>(format!("team:{}:invite", team.id))
        .await?;

    Ok(Json(InviteTokenResponse { token }))
}

#[utoipa::path(
    delete,
    path = "/",
    tag = "game",
    params(
        ("game_id" = i64, Path, description = "Game id"),
    ),
    responses(
        (status = 200, description = "Removed token (value if existed)", body = InviteTokenResponse),
        (status = 401, description = "Unauthorized", body = crate::traits::ErrorResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]

/// Deletes token.
pub async fn delete_token(
    State(s): State<Arc<AppState>>,
    Extension(ext): Extension<AuthPrincipal>,
    Path(game_id): Path<i64>,
) -> Result<Json<InviteTokenResponse>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    let team = crate::util::loader::prepare_self_team(&s.db.conn, game_id, operator.id).await?;
    let token = s
        .cache
        .get_del::<String>(format!("team:{}:invite", team.id))
        .await?;

    Ok(Json(InviteTokenResponse { token }))
}
