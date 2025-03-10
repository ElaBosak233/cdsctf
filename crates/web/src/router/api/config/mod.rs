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

pub async fn get_config() -> Result<WebResponse<serde_json::Value>, WebError> {
    Ok(WebResponse {
        code: StatusCode::OK,
        data: Some(json!({
            "meta": {
                "title": cds_config::get_variable().meta.title,
                "description": cds_config::get_variable().meta.description,
            },
            "auth": {
                "is_registration_enabled": cds_config::get_variable().auth.is_registration_enabled,
            },
            "captcha": {
                "provider": cds_config::get_variable().captcha.provider,
                "turnstile": {
                    "site_key": cds_config::get_variable().captcha.turnstile.site_key,
                },
                "hcaptcha": {
                    "site_key": cds_config::get_variable().captcha.hcaptcha.site_key,
                }
            },
            "version": {
                "tag": cds_config::get_version(),
                "commit": cds_config::get_commit(),
            }
        })),
        ..Default::default()
    })
}

pub async fn get_icon() -> impl IntoResponse {
    match cds_media::get(".".to_owned(), "logo.webp".to_owned()).await {
        Ok(buffer) => Response::builder().body(Body::from(buffer)).unwrap(),
        Err(_) => Redirect::to("/logo.svg").into_response(),
    }
}

pub async fn get_captcha() -> Result<WebResponse<cds_captcha::Captcha>, WebError> {
    let captcha = cds_captcha::generate()
        .await?
        .ok_or(WebError::BadRequest(json!("dont_need_generate_captcha")))?;

    Ok(WebResponse {
        code: StatusCode::OK,
        data: Some(captcha.desensitize()),
        ..Default::default()
    })
}
