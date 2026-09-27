//! HTTP routing for `filename` — Axum router wiring and OpenAPI route
//! registration.

use std::sync::Arc;

use axum::{
    Json, Router,
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
    traits::{AppState, AuthPrincipal, EmptyJson, WebError},
};

/// Builds the Axum router fragment for this module.

pub fn router(state: Arc<AppState>) -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::from(Router::new().with_state(state.clone()))
        .routes(routes!(get_attachment).with_state(state.clone()))
        .routes(routes!(delete_attachment).with_state(state))
}

/// Returns attachment.
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
#[tracing::instrument(skip_all, fields(handler = "get_attachment"))]
pub async fn get_attachment(
    State(s): State<Arc<AppState>>,

    Extension(ext): Extension<AuthPrincipal>,
    Path((challenge_id, filename)): Path<(i64, String)>,
) -> Result<impl IntoResponse, WebError> {
    let operator = ext
        .operator
        .ok_or(WebError::Unauthorized(json!("unauthorized")))?;
    let _ = crate::util::loader::prepare_challenge(&s.db.conn, challenge_id)
        .await?
        .has_attachment
        .then_some(())
        .ok_or_else(|| WebError::NotFound(json!("challenge_has_not_attachment")))?;

    let is_admin = operator.group >= cds_db::user::Group::Admin;
    if !is_admin
        && !cds_db::challenge::can_user_access(&s.db.conn, operator.id, challenge_id).await?
    {
        return Err(WebError::Forbidden(json!("forbidden")));
    }

    let path = crate::util::media::build_challenge_attachment_path(challenge_id);

    if s.media.presigned_enabled() {
        let url = s
            .media
            .presign_get(&path, &filename, 3600)
            .await
            .map_err(|_| WebError::NotFound(json!("not_found")))?;
        return Ok(Redirect::temporary(&url).into_response());
    }

    let buffer = s
        .media
        .get(path, filename.clone())
        .await
        .map_err(|_| WebError::NotFound(json!("not_found")))?;

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/octet-stream")
        .header(
            header::CONTENT_DISPOSITION,
            format!("attachment; filename=\"{}\"", filename),
        )
        .body(Body::from(buffer))?)
}

/// Deletes an attachment. Only a global administrator may mutate attachments.
#[utoipa::path(
    delete,
    path = "/",
    tag = "challenge",
    params(
        ("challenge_id" = i64, Path, description = "Challenge id"),
        ("filename" = String, Path, description = "File name"),
    ),
    responses(
        (status = 200, description = "Deleted", body = EmptyJson),
        (status = 401, description = "Unauthorized", body = crate::traits::ErrorResponse),
        (status = 403, description = "Forbidden", body = crate::traits::ErrorResponse),
        (status = 404, description = "Not found", body = crate::traits::ErrorResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
#[tracing::instrument(skip_all, fields(handler = "delete_attachment"))]
pub async fn delete_attachment(
    State(s): State<Arc<AppState>>,
    Extension(principal): Extension<AuthPrincipal>,
    Path((challenge_id, filename)): Path<(i64, String)>,
) -> Result<Json<EmptyJson>, WebError> {
    crate::middleware::auth::require_admin(&principal)?;
    let _ = crate::util::loader::prepare_challenge(&s.db.conn, challenge_id)
        .await?
        .has_attachment
        .then_some(())
        .ok_or_else(|| WebError::NotFound(json!("challenge_has_not_attachment")))?;

    let path = crate::util::media::build_challenge_attachment_path(challenge_id);
    s.media.delete(path, filename).await?;
    Ok(Json(EmptyJson::default()))
}
