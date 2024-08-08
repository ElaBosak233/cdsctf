use std::path::PathBuf;

use axum::{
    http::{Response, StatusCode},
    response::IntoResponse,
    Json,
};
use serde_json::json;
use tokio::{fs::File, io::AsyncReadExt};

use crate::config::get_config;

pub async fn find() -> impl IntoResponse {
    return (
        StatusCode::OK,
        Json(json!({
            "code": StatusCode::OK.as_u16(),
            "data": {
                "site": get_config().site,
                "auth": {
                    "registration": get_config().auth.registration,
                },
                "container": {
                    "parallel_limit": get_config().container.strategy.parallel_limit,
                    "request_limit": get_config().container.strategy.request_limit,
                },
                "captcha": {
                    "provider": get_config().captcha.provider,
                    "turnstile": {
                        "site_key": get_config().captcha.turnstile.site_key
                    },
                    "recaptcha": {
                        "site_key": get_config().captcha.recaptcha.site_key
                    }
                }
            }
        })),
    );
}

pub async fn get_favicon() -> impl IntoResponse {
    let path = PathBuf::from(get_config().site.favicon.clone());

    match File::open(&path).await {
        Ok(mut file) => {
            let mut buffer = Vec::new();
            if let Err(_) = file.read_to_end(&mut buffer).await {
                return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
            }
            return Response::builder().body(buffer.into()).unwrap();
        }
        Err(_) => return (StatusCode::NOT_FOUND).into_response(),
    }
}
