//! HTTP routing for `captcha` — Axum router wiring and OpenAPI route
//! registration.

use std::sync::Arc;

use axum::{Json, extract::State};
use serde_json::json;

use crate::traits::{AppState, WebError};

/// Issues a captcha challenge for unauthenticated flows.
#[utoipa::path(
    get,
    path = "/generate",
    tag = "config",
    responses(
        (status = 200, description = "Captcha challenge", body = cds_captcha::CaptchaChallenge),
        (status = 400, description = "Captcha not required or bad request", body = crate::traits::ErrorResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
#[tracing::instrument(skip_all, fields(handler = "generate_captcha"))]
pub async fn generate_captcha(
    State(s): State<Arc<AppState>>,
) -> Result<Json<cds_captcha::CaptchaChallenge>, WebError> {
    let captcha = s
        .captcha
        .generate()
        .await?
        .ok_or(WebError::BadRequest(json!("dont_need_generate_captcha")))?;

    Ok(Json(captcha.desensitize()))
}
