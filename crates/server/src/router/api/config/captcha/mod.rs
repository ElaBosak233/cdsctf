use std::sync::Arc;

use axum::{Json, Router, extract::State};
use serde_json::json;
use crate::traits::{AppState, WebError};

pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/generate", axum::routing::get(generate_captcha))
}

#[utoipa::path(
    get,
    path = "/generate",
    tag = "config",
    responses(
        (status = 200, description = "Captcha challenge", body = cds_captcha::CaptchaChallenge),
        (status = 400, description = "Captcha not required or bad request", body = crate::traits::ApiJsonError),
        (status = 500, description = "Server error", body = crate::traits::ApiJsonError),
    )
)]
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
