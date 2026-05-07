//! HTTP routing for `avatar` — Axum router wiring and OpenAPI route
//! registration.

use std::sync::Arc;

use axum::{
    extract::State,
    response::{IntoResponse, Redirect},
};
use serde_json::json;

use crate::{
    extract::Path,
    traits::{AppState, WebError},
};

/// Returns team avatar (redirects to cached media URL).
#[utoipa::path(
    get,
    path = "/",
    tag = "game",
    params(
        ("game_id" = i64, Path, description = "Game id"),
        ("team_id" = i64, Path, description = "Team id"),
    ),
    responses(
        (status = 302, description = "Redirect to cached avatar URL"),
        (status = 404, description = "Not found", body = crate::traits::ErrorResponse),
    )
)]
#[tracing::instrument(skip_all, fields(handler = "get_team_avatar"))]
pub async fn get_team_avatar(
    State(s): State<Arc<AppState>>,
    Path((game_id, team_id)): Path<(i64, i64)>,
) -> Result<impl IntoResponse, WebError> {
    let team = cds_db::team::find_by_id::<cds_db::Team>(&s.db.conn, team_id, game_id)
        .await?
        .ok_or(WebError::NotFound(json!("team_not_found")))?;
    match team.avatar_hash {
        Some(hash) => Ok(Redirect::to(&format!("/api/media?hash={}", hash))),
        None => Err(WebError::NotFound(json!("avatar_not_found"))),
    }
}
