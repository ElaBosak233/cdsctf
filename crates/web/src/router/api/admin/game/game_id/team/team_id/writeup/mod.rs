//! HTTP routing for `writeup` — Axum router wiring and OpenAPI route
//! registration.

use std::sync::Arc;

use axum::{Router, extract::State, response::IntoResponse};
use utoipa_axum::{
    router::{OpenApiRouter, UtoipaMethodRouterExt},
    routes,
};

use crate::{
    extract::{Extension, Path},
    traits::{AppState, AuthPrincipal, WebError},
    util,
};

/// Builds the Axum router fragment for this module.

pub fn router(state: Arc<AppState>) -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::from(Router::new().with_state(state.clone()))
        .routes(routes!(get_team_write_up).with_state(state.clone()))
}

/// Returns team write up.
#[utoipa::path(
    get,
    path = "/",
    tag = "game",
    params(
        ("game_id" = i64, Path, description = "Game id"),
        ("team_id" = i64, Path, description = "Team id"),
    ),
    responses(
        (status = 200, description = "Write-up file"),
        (status = 404, description = "Not found", body = crate::traits::ErrorResponse),
    )
)]
#[tracing::instrument(skip_all, fields(handler = "get_team_write_up"))]
pub async fn get_team_write_up(
    State(s): State<Arc<AppState>>,
    Extension(principal): Extension<AuthPrincipal>,
    Path((game_id, team_id)): Path<(i64, i64)>,
) -> Result<impl IntoResponse, WebError> {
    crate::middleware::auth::require_admin(&principal)?;
    util::media::get_write_up(s.media.clone(), game_id, team_id).await
}
