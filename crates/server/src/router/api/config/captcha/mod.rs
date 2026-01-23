use std::sync::Arc;

use axum::{Router, extract::State};
use serde_json::json;

use crate::traits::{AppState, WebError, WebResponse};

pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/generate", axum::routing::get(generate_captcha))
}

pub async fn generate_captcha(
    State(s): State<Arc<AppState>>,
) -> Result<WebResponse<cds_captcha::CaptchaChallenge>, WebError> {
    let captcha = s
        .captcha
        .generate()
        .await?
        .ok_or(WebError::BadRequest(json!("dont_need_generate_captcha")))?;

    Ok(WebResponse {
        data: Some(captcha.desensitize()),
        ..Default::default()
    })
}
