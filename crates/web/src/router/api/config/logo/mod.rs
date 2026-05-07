//! HTTP routing for `logo` — Axum router wiring and OpenAPI route registration.

use std::sync::Arc;

use axum::{
    extract::State,
    response::{IntoResponse, Redirect},
};

use crate::traits::{AppState, WebError};

/// Returns logo.
#[utoipa::path(
    get,
    path = "/",
    tag = "config",
    responses(
        (status = 302, description = "Redirect to cached media URL or default /logo.svg"),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
#[tracing::instrument(skip_all, fields(handler = "get_logo"))]
pub async fn get_logo(State(s): State<Arc<AppState>>) -> Result<impl IntoResponse, WebError> {
    let config = cds_db::config::get(&s.db.conn).await?;
    match config.logo_hash {
        Some(hash) => Ok(Redirect::to(&format!("/api/media?hash={}", hash))),
        None => Ok(Redirect::to("/logo.svg")),
    }
}
