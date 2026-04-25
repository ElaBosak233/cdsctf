//! HTTP routing for `avatar` — Axum router wiring and OpenAPI route
//! registration.

use std::sync::Arc;

use axum::{body::Body, extract::State, http::Response, response::IntoResponse};

use crate::{
    extract::Path,
    traits::{AppState, WebError},
};

/// Returns team avatar.
#[utoipa::path(
    get,
    path = "/",
    tag = "game",
    params(
        ("game_id" = i64, Path, description = "Game id"),
        ("team_id" = i64, Path, description = "Team id"),
    ),
    responses(
        (status = 200, description = "Avatar bytes"),
        (status = 404, description = "Not found", body = crate::traits::ErrorResponse),
    )
)]
#[tracing::instrument(skip_all, fields(handler = "get_team_avatar"))]
pub async fn get_team_avatar(
    State(s): State<Arc<AppState>>,

    Path((game_id, team_id)): Path<(i64, i64)>,
) -> Result<impl IntoResponse, WebError> {
    let path = format!("games/{game_id}/teams/{team_id}");

    let buffer = s.media.get(path, "avatar".to_owned()).await?;

    Ok(Response::builder().body(Body::from(buffer))?)
}
