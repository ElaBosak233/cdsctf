//! HTTP routing for `notice` — Axum router wiring and OpenAPI route
//! registration.

use std::sync::Arc;

use axum::{Json, Router, extract::State};
use cds_db::{GameNoticeView, team::State as TState};
use serde_json::json;
use utoipa_axum::{
    router::{OpenApiRouter, UtoipaMethodRouterExt},
    routes,
};

use crate::{
    extract::{Extension, Path},
    middleware::auth::basic_operator,
    traits::{AppState, AuthPrincipal, WebError},
};

/// Builds the Axum router fragment for this module.

pub fn router(state: Arc<AppState>) -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::from(Router::new().with_state(state.clone()))
        .routes(routes!(list_game_notices).with_state(state.clone()))
}

#[derive(Clone, Debug, serde::Serialize, utoipa::ToSchema)]
pub struct GameNoticesListResponse {
    pub notices: Vec<GameNoticeView>,
    pub total: u64,
}

/// Lists notices for active players in a game (ordered newest first).
#[utoipa::path(
    get,
    path = "/",
    tag = "game",
    params(
        ("game_id" = i64, Path, description = "Game id"),
    ),
    responses(
        (status = 200, description = "Notices", body = GameNoticesListResponse),
        (status = 401, description = "Unauthorized", body = crate::traits::ErrorResponse),
        (status = 403, description = "Forbidden", body = crate::traits::ErrorResponse),
        (status = 404, description = "Not found", body = crate::traits::ErrorResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
#[tracing::instrument(skip_all, fields(handler = "list_game_notices"))]
pub async fn list_game_notices(
    State(s): State<Arc<AppState>>,
    Extension(ext): Extension<AuthPrincipal>,
    Path(game_id): Path<i64>,
) -> Result<Json<GameNoticesListResponse>, WebError> {
    let operator = basic_operator(&ext)?;

    let game = crate::util::loader::prepare_game(&s.db.conn, game_id).await?;
    if !game.enabled {
        return Err(WebError::NotFound(json!("not_found")));
    }

    let now = time::OffsetDateTime::now_utc();
    let in_game =
        cds_db::team::contains_user_in_game(&s.db.conn, game.id, operator.id, Some(TState::Passed))
            .await?;
    if !in_game {
        return Err(WebError::Forbidden(json!("forbidden")));
    }
    // Notices stay readable while paused so players can see maintenance or
    // resume announcements, while still gated to the active competition window.
    crate::util::loader::ensure_game_ongoing(&game, now)?;

    let game_notices =
        cds_db::game_notice::find_player_views_by_game_id(&s.db.conn, game.id).await?;
    let total = game_notices.len() as u64;

    Ok(Json(GameNoticesListResponse {
        notices: game_notices,
        total,
    }))
}
