mod captcha;
mod logo;

use axum::Router;
use serde::{Deserialize, Serialize};

use crate::traits::{WebError, WebResponse};

pub fn router() -> Router {
    Router::new()
        .route("/", axum::routing::get(get_config))
        .nest("/logo", logo::router())
        .nest("/captcha", captcha::router())
        .route("/version", axum::routing::get(get_version))
}

pub async fn get_config() -> Result<WebResponse<cds_db::entity::config::Model>, WebError> {
    Ok(WebResponse {
        data: Some(cds_db::get_config().await.desensitize()),
        ..Default::default()
    })
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Version {
    pub tag: String,
    pub commit: String,
}

pub async fn get_version() -> Result<WebResponse<Version>, WebError> {
    Ok(WebResponse {
        data: Some(Version {
            tag: cds_config::get_version(),
            commit: cds_config::get_commit(),
        }),
        ..Default::default()
    })
}
