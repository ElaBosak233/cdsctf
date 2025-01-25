use axum::{
    Router,
    body::Body,
    http::{Response, StatusCode},
    response::{IntoResponse, Redirect},
};
use sea_orm::ActiveModelTrait;
use serde::{Deserialize, Serialize};

use crate::traits::{WebError, WebResponse};

pub fn router() -> Router {
    Router::new()
        .route("/meta", axum::routing::get(get_meta))
        .route("/icon", axum::routing::get(get_icon))
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Meta {
    pub title: String,
    pub description: String,
}

pub async fn get_meta() -> Result<WebResponse<Meta>, WebError> {
    Ok(WebResponse {
        code: StatusCode::OK.as_u16(),
        data: Some(Meta {
            title: cds_config::get_config().meta.title,
            description: cds_config::get_config().meta.description,
        }),
        ..WebResponse::default()
    })
}

pub async fn get_icon() -> impl IntoResponse {
    let path = String::from("configs");
    let filename = String::from("icon.webp");
    match cds_media::get(path, filename).await {
        Ok(data) => Response::builder().body(Body::from(data)).unwrap(),
        Err(_) => {
            Redirect::to("/icon.svg").into_response() // default frontend icon
        }
    }
}
