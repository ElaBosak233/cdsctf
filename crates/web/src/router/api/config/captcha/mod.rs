use axum::Router;
use serde_json::json;

use crate::traits::{WebError, WebResponse};

pub fn router() -> Router {
    Router::new().route("/generate", axum::routing::get(generate_captcha))
}

pub async fn generate_captcha() -> Result<WebResponse<cds_captcha::Captcha>, WebError> {
    let captcha = cds_captcha::generate()
        .await?
        .ok_or(WebError::BadRequest(json!("dont_need_generate_captcha")))?;

    Ok(WebResponse {
        data: Some(captcha.desensitize()),
        ..Default::default()
    })
}
