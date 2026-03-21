//! HTTP routing for `attachment` — Axum router wiring and OpenAPI route
//! registration.

/// Defines the `filename` submodule (see sibling `*.rs` files).
mod filename;

use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{Multipart, State},
};
use serde_json::json;
use utoipa_axum::{
    router::{OpenApiRouter, UtoipaMethodRouterExt},
    routes,
};

use crate::{
    extract::Path,
    model::Metadata,
    traits::{AppState, EmptyJson, WebError},
};

/// Builds the Axum router fragment for this module.

pub fn router(state: Arc<AppState>) -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::from(Router::new().with_state(state.clone()))
        .routes(routes!(get_challenge_attachment).with_state(state.clone()))
        .routes(routes!(save_challenge_attachment).with_state(state.clone()))
        .nest("/{filename}", filename::router(state.clone()))
}

#[derive(Clone, Debug, serde::Serialize, utoipa::ToSchema)]
pub struct AdminChallengeAttachmentsListResponse {
    pub attachments: Vec<Metadata>,
    pub total: u64,
}

#[utoipa::path(
    get,
    path = "/",
    tag = "admin-challenge",
    params(
        ("challenge_id" = i64, Path, description = "Challenge id"),
    ),
    responses(
        (status = 200, description = "Attachments", body = AdminChallengeAttachmentsListResponse),
        (status = 404, description = "Not found", body = crate::traits::ErrorResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]

/// Returns challenge attachment.
pub async fn get_challenge_attachment(
    State(s): State<Arc<AppState>>,
    Path(challenge_id): Path<i64>,
) -> Result<Json<AdminChallengeAttachmentsListResponse>, WebError> {
    let _ = crate::util::loader::prepare_challenge(&s.db.conn, challenge_id)
        .await?
        .has_attachment
        .then_some(())
        .ok_or_else(|| WebError::NotFound(json!("challenge_has_not_attachment")))?;

    let path = crate::util::media::build_challenge_attachment_path(challenge_id);
    let metadata = s
        .media
        .scan_dir(path.clone())
        .await?
        .into_iter()
        .map(|(filename, size)| Metadata { filename, size })
        .collect::<Vec<Metadata>>();
    let total = metadata.len() as u64;

    Ok(Json(AdminChallengeAttachmentsListResponse {
        attachments: metadata,
        total,
    }))
}

#[utoipa::path(
    post,
    path = "/",
    tag = "admin-challenge",
    params(
        ("challenge_id" = i64, Path, description = "Challenge id"),
    ),
    responses(
        (status = 200, description = "Uploaded", body = EmptyJson),
        (status = 400, description = "Bad request", body = crate::traits::ErrorResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]

/// Uploads an administrator attachment for a challenge.
pub async fn save_challenge_attachment(
    State(s): State<Arc<AppState>>,
    Path(challenge_id): Path<i64>,
    mut multipart: Multipart,
) -> Result<Json<EmptyJson>, WebError> {
    let _ = crate::util::loader::prepare_challenge(&s.db.conn, challenge_id).await?;

    let path = crate::util::media::build_challenge_attachment_path(challenge_id);
    let mut filename = String::new();
    let mut data = Vec::<u8>::new();
    while let Some(field) = multipart.next_field().await? {
        if let Some(name) = field.file_name() {
            filename = name.to_string();
            data = match field.bytes().await {
                Ok(bytes) => bytes.to_vec(),
                _ => return Err(WebError::BadRequest(json!("size_too_large"))),
            };
            break;
        }
    }

    s.media
        .save(path, filename, data)
        .await
        .map_err(|_| WebError::InternalServerError(json!("")))?;

    Ok(Json(EmptyJson::default()))
}
