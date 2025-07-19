use axum::Router;
use cds_media::config::email::EmailType;
use serde::Deserialize;

use crate::{
    extract::{Json, Query},
    traits::{WebError, WebResponse},
};

pub fn router() -> Router {
    Router::new()
        .route("/", axum::routing::get(get_email))
        .route("/", axum::routing::post(save_email))
}

#[derive(Deserialize)]
pub struct GetEmailRequest {
    #[serde(rename = "type")]
    pub type_: EmailType,
}

pub async fn get_email(
    Query(params): Query<GetEmailRequest>,
) -> Result<WebResponse<String>, WebError> {
    Ok(WebResponse {
        data: Some(cds_media::config::email::get_email(params.type_).await?),
        ..Default::default()
    })
}

#[derive(Deserialize)]
pub struct SaveEmailRequest {
    #[serde(rename = "type")]
    pub type_: EmailType,
    pub data: String,
}

pub async fn save_email(Json(body): Json<SaveEmailRequest>) -> Result<WebResponse<()>, WebError> {
    cds_media::config::email::save_email(body.type_, body.data).await?;
    Ok(WebResponse {
        ..Default::default()
    })
}
