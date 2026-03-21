//! HTTP routing for `filename` — Axum router wiring and OpenAPI route
//! registration.

use std::sync::Arc;

use axum::{
    Router,
    body::Body,
    extract::State,
    http::{Response, StatusCode, header},
    response::{IntoResponse, Redirect},
};
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
        .routes(routes!(get_attachment).with_state(state.clone()))
}

#[utoipa::path(
    get,
    path = "/",
    tag = "challenge",
    params(
        ("challenge_id" = i64, Path, description = "Challenge id"),
        ("filename" = String, Path, description = "File name"),
    ),
    responses(
        (status = 200, description = "File bytes or redirect"),
        (status = 302, description = "Presigned redirect"),
        (status = 401, description = "Unauthorized", body = crate::traits::ErrorResponse),
        (status = 404, description = "Not found", body = crate::traits::ErrorResponse),
    )
)]

/// Returns attachment.
pub async fn get_attachment(
    State(s): State<Arc<AppState>>,

    Extension(ext): Extension<AuthPrincipal>,
    Path((challenge_id, filename)): Path<(i64, String)>,
) -> Result<impl IntoResponse, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    let _ = crate::util::loader::prepare_challenge(&s.db.conn, challenge_id)
        .await?
        .has_attachment
        .then_some(())
        .ok_or_else(|| WebError::NotFound(json!("challenge_has_not_attachment")))?;

    if !cds_db::util::can_user_access_challenge(&s.db.conn, operator.id, challenge_id).await? {
        return Err(WebError::Forbidden(json!("")));
    }

    let path = crate::util::media::build_challenge_attachment_path(challenge_id);

    if s.media.presigned_enabled() {
        let url = s
            .media
            .presign_get(&path, &filename, 3600)
            .await
            .map_err(|_| WebError::NotFound(json!("")))?;
        return Ok(Redirect::temporary(&url).into_response());
    }

    let buffer = s
        .media
        .get(path, filename.clone())
        .await
        .map_err(|_| WebError::NotFound(json!("")))?;

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/octet-stream")
        .header(
            header::CONTENT_DISPOSITION,
            format!("attachment; filename=\"{}\"", filename),
        )
        .body(Body::from(buffer))?)
}
