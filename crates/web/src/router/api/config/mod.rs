mod captcha;
mod logo;

use axum::{Router, response::IntoResponse};
use cds_db::entity::user::Group;
use sea_orm::ActiveModelTrait;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    extract::{Extension, Json, Query},
    traits::{Ext, WebError, WebResponse},
};

pub fn router() -> Router {
    Router::new()
        .route("/", axum::routing::get(get_config))
        .route("/", axum::routing::put(update_config))
        .nest("/logo", logo::router())
        .nest("/captcha", captcha::router())
        .route("/version", axum::routing::get(get_version))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetConfigRequest {
    pub is_desensitized: bool,
}

pub async fn get_config(
    Extension(ext): Extension<Ext>, Query(params): Query<GetConfigRequest>,
) -> Result<WebResponse<cds_config::variable::Variable>, WebError> {
    if !params.is_desensitized {
        let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
        if operator.group != Group::Admin {
            return Err(WebError::Forbidden(json!("")));
        }
    }

    Ok(WebResponse {
        data: Some(if params.is_desensitized {
            cds_config::get_variable().desensitize()
        } else {
            cds_config::get_variable()
        }),
        ..Default::default()
    })
}

pub async fn update_config(
    Extension(ext): Extension<Ext>, Json(body): Json<cds_config::variable::Variable>,
) -> Result<WebResponse<cds_config::variable::Variable>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    if operator.group != Group::Admin {
        return Err(WebError::Forbidden(json!("")));
    }

    cds_config::variable::set_variable(body.clone())?;
    cds_config::variable::save().await?;

    Ok(WebResponse {
        data: Some(body),
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
