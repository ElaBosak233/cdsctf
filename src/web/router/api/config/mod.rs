use std::path::PathBuf;

use axum::{
    http::{Response, StatusCode},
    response::IntoResponse,
    Extension, Json, Router,
};
use sea_orm::{ActiveModelTrait, ActiveValue::Set};
use tokio::{fs::File, io::AsyncReadExt};

use crate::{
    config::get_config,
    db::get_db,
    web::traits::{Ext, WebError, WebResult},
};

pub fn router() -> Router {
    Router::new()
        .route("/", axum::routing::get(get))
        .route("/", axum::routing::put(update))
        .route("/favicon", axum::routing::get(get_favicon))
}

pub async fn get(
    Extension(ext): Extension<Ext>,
) -> Result<WebResult<crate::config::Config>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(String::new()))?;
    if operator.group != crate::model::user::group::Group::Admin {
        return Err(WebError::Forbidden(String::new()));
    }

    Ok(WebResult {
        code: StatusCode::OK.as_u16(),
        data: Some(get_config().await),
        ..WebResult::default()
    })
}

pub async fn update(
    Extension(ext): Extension<Ext>, Json(mut body): Json<crate::config::Config>,
) -> Result<WebResult<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(String::new()))?;
    if operator.group != crate::model::user::group::Group::Admin {
        return Err(WebError::Forbidden(String::new()));
    }

    let config = crate::model::config::ActiveModel {
        id: Set(1),
        auth: Set(body.auth),
        site: Set(body.site),
        cluster: Set(body.cluster),
        ..Default::default()
    };

    config.update(&get_db()).await?;

    Ok(WebResult {
        code: StatusCode::OK.as_u16(),
        ..WebResult::default()
    })
}

pub async fn get_favicon() -> impl IntoResponse {
    let path = PathBuf::from(get_config().await.site.favicon.clone());

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
