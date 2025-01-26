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
        .route("/", axum::routing::get(get))
        .route("/icon", axum::routing::get(get_icon))
}

pub type ClientConfig = serde_json::Value;
pub async fn get() -> Result<WebResponse<ClientConfig>, WebError> {
    Ok(WebResponse {
        code: StatusCode::OK.as_u16(),
        data: Some(json!({
            "meta": {
                "title": cds_config::get_config().meta.title,
                "description": cds_config::get_config().meta.description,
            },
        })),
        ..WebResponse::default()
    })
}

pub async fn get_icon() -> impl IntoResponse {
    let path = cds_config::get_config().meta.logo_path;
    match tokio::fs::read(path).await {
        Ok(data) => Response::builder().body(Body::from(data)).unwrap(),
        Err(_) => {
            Redirect::to("/logo.svg").into_response() // default frontend icon
        }
    }
}
