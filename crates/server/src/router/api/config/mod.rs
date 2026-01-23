mod captcha;
mod logo;

use std::sync::Arc;

use axum::{Router, extract::State};
use serde::{Deserialize, Serialize};

use crate::traits::{AppState, WebError, WebResponse};

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", axum::routing::get(get_config))
        .nest("/logo", logo::router())
        .nest("/captcha", captcha::router())
        .route("/version", axum::routing::get(get_version))
}

pub async fn get_config(
    State(s): State<Arc<AppState>>,
) -> Result<WebResponse<cds_db::config::Model>, WebError> {
    Ok(WebResponse {
        data: Some(cds_db::get_config(&s.db.conn).await.desensitize()),
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
            tag: cds_env::get_version().to_owned(),
            commit: cds_env::get_commit_hash().to_owned(),
        }),
        ..Default::default()
    })
}
