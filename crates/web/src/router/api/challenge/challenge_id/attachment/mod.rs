//! HTTP routing for `attachment` — Axum router wiring and OpenAPI route
//! registration.

/// Defines the `filename` submodule (see sibling `*.rs` files).
mod filename;

use std::sync::Arc;

use axum::{Json, Router, extract::State};
use serde_json::json;
use utoipa_axum::{
    router::{OpenApiRouter, UtoipaMethodRouterExt},
    routes,
};

use crate::{
    extract::{Extension, Path},
    model::Metadata,
    traits::{AppState, AuthPrincipal, WebError},
};

/// Builds the Axum router fragment for this module.

pub fn router(state: Arc<AppState>) -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::from(Router::new().with_state(state.clone()))
        .routes(routes!(get_challenge_attachment).with_state(state.clone()))
        .nest("/{filename}", filename::router(state.clone()))
}

#[derive(Clone, Debug, serde::Serialize, utoipa::ToSchema)]
pub struct ChallengeAttachmentsListResponse {
    pub attachments: Vec<Metadata>,
    pub total: u64,
}

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

/// Returns challenge attachment.
pub async fn get_challenge_attachment(
    State(s): State<Arc<AppState>>,
    Extension(ext): Extension<AuthPrincipal>,
    Path(challenge_id): Path<i64>,
) -> Result<Json<ChallengeAttachmentsListResponse>, WebError> {
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
