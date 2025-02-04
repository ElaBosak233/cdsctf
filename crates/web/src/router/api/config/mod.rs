use axum::{
    Router,
    body::Body,
    http::{Response, StatusCode},
    response::{IntoResponse, Redirect},
};
use sea_orm::ActiveModelTrait;
use serde_json::json;

use crate::traits::{WebError, WebResponse};

pub fn router() -> Router {
    Router::new()
        .route("/", axum::routing::get(get_config))
        .route("/icon", axum::routing::get(get_icon))
        .route("/captcha", axum::routing::get(get_captcha))
}

pub type ClientConfig = serde_json::Value;
pub async fn get_config() -> Result<WebResponse<ClientConfig>, WebError> {
    Ok(WebResponse {
        code: StatusCode::OK.as_u16(),
        data: Some(json!({
            "meta": {
                "title": cds_config::get_config().meta.title,
                "description": cds_config::get_config().meta.description,
            },
            "captcha": {
                "provider": cds_config::get_config().captcha.provider,
                "turnstile": {
                    "site_key": cds_config::get_config().captcha.turnstile.site_key,
                },
                "hcaptcha": {
                    "site_key": cds_config::get_config().captcha.hcaptcha.site_key,
                }
            }
        })),
        ..WebResponse::default()
    })
}

pub async fn get_icon() -> impl IntoResponse {
    let path = &cds_config::get_config().meta.logo_path;
    match tokio::fs::read(path).await {
        Ok(data) => Response::builder().body(Body::from(data)).unwrap(),
        Err(_) => {
            Redirect::to("/logo.svg").into_response() // default frontend icon
        }
    }
}

pub async fn get_captcha() -> Result<WebResponse<cds_captcha::Captcha>, WebError> {
    let captcha = cds_captcha::generate()
        .await?
        .ok_or(WebError::BadRequest(json!("dont_need_generate_captcha")))?;

    Ok(WebResponse {
        code: StatusCode::OK.as_u16(),
        data: Some(captcha.desensitize()),
        ..Default::default()
    })
}
