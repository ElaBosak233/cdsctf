//! HTTP routing for `media` — Axum router wiring and OpenAPI route
//! registration.

use std::sync::Arc;

use axum::{
    Json, Router,
    body::Body,
    extract::{Multipart, State},
    http::{Response, StatusCode, header::CACHE_CONTROL},
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use utoipa_axum::{
    router::{OpenApiRouter, UtoipaMethodRouterExt},
    routes,
};

use crate::{
    extract::{Extension, Query},
    traits::{AppState, AuthPrincipal, WebError},
    util::media::handle_multipart,
};

/// Builds the Axum router fragment for this module.

pub fn router(state: Arc<AppState>) -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::from(Router::new().with_state(state.clone()))
        .routes(routes!(get_media).with_state(state.clone()))
        .routes(routes!(upload_media).with_state(state.clone()))
}

#[derive(Clone, Debug, Serialize, Deserialize, utoipa::ToSchema, utoipa::IntoParams)]
#[into_params(parameter_in = Query)]
pub struct GetMediaRequest {
    pub hash: String,
}

/// Returns stored media bytes by `?hash=` (kept for backward compatibility with
/// stored URLs).
#[utoipa::path(
    get,
    path = "/",
    tag = "media",
    params(GetMediaRequest),
    responses(
        (status = 200, description = "Cached media bytes", body = Vec<u8>),
        (status = 404, description = "Not found", body = crate::traits::ErrorResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
#[tracing::instrument(skip_all, fields(handler = "get_media"))]
pub async fn get_media(
    State(s): State<Arc<AppState>>,
    Query(params): Query<GetMediaRequest>,
) -> Result<impl IntoResponse, WebError> {
    let buffer = s.media.get("media".to_owned(), params.hash).await?;

    Ok(Response::builder()
        .header(CACHE_CONTROL, "public, max-age=31536000, immutable")
        .body(Body::from(buffer))?)
}

#[derive(Clone, Debug, Serialize, utoipa::ToSchema)]
pub struct UploadMediaResponse {
    pub hash: String,
}

/// Accepts a multipart upload and stores it in configured media backend.
#[utoipa::path(
    post,
    path = "/",
    tag = "media",
    request_body(content_type = "multipart/form-data"),
    responses(
        (status = 201, description = "Stored object hash", body = UploadMediaResponse),
        (status = 400, description = "Bad request", body = crate::traits::ErrorResponse),
        (status = 401, description = "Unauthorized", body = crate::traits::ErrorResponse),
        (status = 429, description = "Rate limited", body = crate::traits::ErrorResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
#[tracing::instrument(skip_all, fields(handler = "upload_media"))]
pub async fn upload_media(
    State(s): State<Arc<AppState>>,

    Extension(ap): Extension<AuthPrincipal>,
    multipart: Multipart,
) -> Result<(StatusCode, Json<UploadMediaResponse>), WebError> {
    let operator = ap.operator.ok_or(WebError::Unauthorized("".into()))?;
    let token_10m = format!("media:upload:10m:{}", operator.id);
    let token_24h = format!("media:upload:24h:{}", operator.id);

    if let Some(limit) = s.cache.get::<i32>(&token_10m).await? {
        if limit >= 10 {
            return Err(WebError::TooManyRequests(json!("upload_media_10m")));
        } else {
            s.cache.incr(&token_10m).await?;
        }
    } else {
        s.cache.set_ex(&token_10m, 1, 600).await?;
    }

    if let Some(limit) = s.cache.get::<i32>(&token_24h).await? {
        if limit >= 50 {
            return Err(WebError::TooManyRequests(json!("upload_media_24h")));
        } else {
            s.cache.incr(&token_24h).await?;
        }
    } else {
        s.cache.set_ex(&token_24h, 1, 86400).await?;
    }

    let data = handle_multipart(multipart, mime::IMAGE).await?;
    let data = cds_media::util::img_convert_to_webp(data).await?;

    let hash = cds_media::util::hash(data.clone());

    s.media.save("media".to_owned(), hash.clone(), data).await?;

    Ok((StatusCode::CREATED, Json(UploadMediaResponse { hash })))
}
