//! HTTP routing for `attachment` — Axum router wiring and OpenAPI route
//! registration.

/// Defines the `filename` submodule (see sibling `*.rs` files).
mod filename;

use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{DefaultBodyLimit, Multipart, State},
};
use serde_json::json;
use utoipa_axum::{
    router::{OpenApiRouter, UtoipaMethodRouterExt},
    routes,
};

use crate::{
    extract::{Extension, Path},
    model::Metadata,
    traits::{AppState, AuthPrincipal, EmptyJson, WebError},
};

/// Builds the Axum router fragment for this module.

pub fn router(state: Arc<AppState>) -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::from(Router::new().with_state(state.clone()))
        .routes(routes!(get_challenge_attachment).with_state(state.clone()))
        .routes(
            routes!(save_challenge_attachment)
                .with_state(state.clone())
                .layer(DefaultBodyLimit::max(200 * 1024 * 1024)),
        )
        .nest("/{filename}", filename::router(state.clone()))
}

#[derive(Clone, Debug, serde::Serialize, utoipa::ToSchema)]
pub struct ChallengeAttachmentsListResponse {
    pub attachments: Vec<Metadata>,
    pub total: u64,
}

/// Returns challenge attachment.
#[utoipa::path(
    get,
    path = "/",
    tag = "challenge",
    params(
        ("challenge_id" = i64, Path, description = "Challenge id"),
    ),
    responses(
        (status = 200, description = "Attachment metadata", body = ChallengeAttachmentsListResponse),
        (status = 401, description = "Unauthorized", body = crate::traits::ErrorResponse),
        (status = 404, description = "Not found", body = crate::traits::ErrorResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
#[tracing::instrument(skip_all, fields(handler = "get_challenge_attachment"))]
pub async fn get_challenge_attachment(
    State(s): State<Arc<AppState>>,
    Extension(ext): Extension<AuthPrincipal>,
    Path(challenge_id): Path<i64>,
) -> Result<Json<ChallengeAttachmentsListResponse>, WebError> {
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
    let metadata = s
        .media
        .scan_dir(path.clone())
        .await?
        .into_iter()
        .map(|(filename, size)| Metadata { filename, size })
        .collect::<Vec<Metadata>>();
    let total = metadata.len() as u64;

    Ok(Json(ChallengeAttachmentsListResponse {
        attachments: metadata,
        total,
    }))
}

/// Uploads an attachment. Only a global administrator may mutate attachments.
#[utoipa::path(
    post,
    path = "/",
    tag = "challenge",
    params(("challenge_id" = i64, Path, description = "Challenge id")),
    responses(
        (status = 200, description = "Uploaded", body = EmptyJson),
        (status = 400, description = "Bad request", body = crate::traits::ErrorResponse),
        (status = 401, description = "Unauthorized", body = crate::traits::ErrorResponse),
        (status = 403, description = "Forbidden", body = crate::traits::ErrorResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
#[tracing::instrument(skip_all, fields(handler = "save_challenge_attachment"))]
pub async fn save_challenge_attachment(
    State(s): State<Arc<AppState>>,
    Extension(principal): Extension<AuthPrincipal>,
    Path(challenge_id): Path<i64>,
    mut multipart: Multipart,
) -> Result<Json<EmptyJson>, WebError> {
    crate::middleware::auth::require_admin(&principal)?;
    let _ = crate::util::loader::prepare_challenge(&s.db.conn, challenge_id).await?;

    let path = crate::util::media::build_challenge_attachment_path(challenge_id);
    let mut filename = String::new();
    let mut data = Vec::<u8>::new();
    while let Some(field) = multipart.next_field().await? {
        if let Some(name) = field.file_name() {
            filename = name.to_owned();
            data = field
                .bytes()
                .await
                .map_err(|_| WebError::BadRequest(serde_json::json!("size_too_large")))?
                .to_vec();
            break;
        }
    }

    if filename.is_empty() {
        return Err(WebError::BadRequest(serde_json::json!("file_required")));
    }

    s.media
        .save(path, filename, data)
        .await
        .map_err(|_| WebError::InternalServerError(serde_json::json!("internal_server_error")))?;

    Ok(Json(EmptyJson::default()))
}
