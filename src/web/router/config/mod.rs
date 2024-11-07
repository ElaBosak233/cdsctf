use std::path::PathBuf;

use axum::{
    http::{Response, StatusCode},
    response::IntoResponse,
    Router,
};
use serde_json::json;
use tokio::{fs::File, io::AsyncReadExt};

use crate::{config::get_config, web::traits::WebResult};

pub fn router() -> Router {
    Router::new()
        .route("/", axum::routing::get(get))
        .route("/favicon", axum::routing::get(get_favicon))
}

pub async fn get() -> WebResult<serde_json::Value> {
    WebResult {
        code: StatusCode::OK.as_u16(),
        data: Some(json!({
            "site": get_config().site,
            "auth": {
                "registration": get_config().auth.registration,
            },
            "cluster": {
                "parallel_limit": get_config().cluster.strategy.parallel_limit,
                "request_limit": get_config().cluster.strategy.request_limit,
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
        })),
        ..WebResult::default()
    }
}

pub async fn get_favicon() -> impl IntoResponse {
    let path = PathBuf::from(get_config().site.favicon.clone());

    match File::open(&path).await {
        Ok(mut file) => {
            let mut buffer = Vec::new();
            if let Err(_) = file.read_to_end(&mut buffer).await {
                return StatusCode::INTERNAL_SERVER_ERROR.into_response();
            }
            Response::builder().body(buffer.into()).unwrap()
        }
        Err(_) => StatusCode::NOT_FOUND.into_response(),
    }
}
