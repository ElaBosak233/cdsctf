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
}

pub async fn get_config() -> Result<WebResponse<cds_config::variable::Variable>, WebError> {
    Ok(WebResponse {
        data: Some(cds_config::get_variable()),
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
