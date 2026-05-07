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

/// Returns user avatar (redirects to cached media URL).
#[utoipa::path(
    get,
    path = "/",
    tag = "user",
    params(
        ("user_id" = i64, Path, description = "User id"),
    ),
    responses(
        (status = 302, description = "Redirect to cached avatar URL"),
        (status = 404, description = "Not found", body = crate::traits::ErrorResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
#[tracing::instrument(skip_all, fields(handler = "get_user_avatar"))]
pub async fn get_user_avatar(
    State(s): State<Arc<AppState>>,
    Path(user_id): Path<i64>,
) -> Result<impl IntoResponse, WebError> {
    let user = cds_db::user::find_by_id::<cds_db::User>(&s.db.conn, user_id)
        .await?
        .ok_or(WebError::NotFound(json!("user_not_found")))?;
    match user.avatar_hash {
        Some(hash) => Ok(Redirect::to(&format!("/api/media?hash={}", hash))),
        None => Err(WebError::NotFound(json!("avatar_not_found"))),
    }
}
