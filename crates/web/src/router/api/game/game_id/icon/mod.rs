//! HTTP routing for `icon` — Axum router wiring and OpenAPI route registration.

use std::sync::Arc;

use axum::{
    Router,
    extract::State,
    response::{IntoResponse, Redirect},
};
use serde_json::json;
use utoipa_axum::{
    router::{OpenApiRouter, UtoipaMethodRouterExt},
    routes,
};

use crate::{
    extract::Path,
    traits::{AppState, WebError},
};

/// Builds the Axum router fragment for this module.

pub fn router(state: Arc<AppState>) -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::from(Router::new().with_state(state.clone()))
        .routes(routes!(get_game_icon).with_state(state.clone()))
}

/// Returns game icon (redirects to cached media URL).
#[utoipa::path(
    get,
    path = "/",
    tag = "game",
    params(
        ("game_id" = i64, Path, description = "Game id"),
    ),
    responses(
        (status = 302, description = "Redirect to cached icon URL"),
        (status = 404, description = "Not found", body = crate::traits::ErrorResponse),
    )
)]
#[tracing::instrument(skip_all, fields(handler = "get_game_icon"))]
pub async fn get_game_icon(
    State(s): State<Arc<AppState>>,
    Path(game_id): Path<i64>,
) -> Result<impl IntoResponse, WebError> {
    let game = cds_db::game::find_by_id::<cds_db::Game>(&s.db.conn, game_id)
        .await?
        .ok_or(WebError::NotFound(json!("game_not_found")))?;
    match game.icon_hash {
        Some(hash) => Ok(Redirect::to(&format!("/api/media?hash={}", hash))),
        None => Err(WebError::NotFound(json!("icon_not_found"))),
    }
}
