use axum::{
    Router,
    body::Body,
    extract::Multipart,
    http::{Response, StatusCode},
    response::{IntoResponse, Redirect},
};
use cds_config::get_config;
use cds_db::{entity::user::Group, get_db};
use sea_orm::{ActiveModelTrait, ActiveValue::Set};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    extract::{Extension, Json, Query},
    traits::{Ext, WebError, WebResponse},
    util::handle_image_multipart,
};

pub fn router() -> Router {
    Router::new()
        .route("/", axum::routing::get(get))
        .route("/", axum::routing::put(update))
        .route("/icon", axum::routing::get(get_icon))
        .route("/icon", axum::routing::post(save_icon))
        .route("/icon", axum::routing::delete(delete_icon))
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetRequest {
    pub is_detailed: Option<bool>,
}

pub async fn get(
    Extension(ext): Extension<Ext>, Query(params): Query<GetRequest>,
) -> Result<WebResponse<cds_config::Config>, WebError> {
    match params.is_detailed {
        Some(true) => {
            let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
            if operator.group != Group::Admin {
                return Err(WebError::Forbidden(json!("")));
            }

            Ok(WebResponse {
                code: StatusCode::OK.as_u16(),
                data: Some(get_config().await),
                ..WebResponse::default()
            })
        }
        _ => {
            let mut config = get_config().await;
            config.desensitize();

            Ok(WebResponse {
                code: StatusCode::OK.as_u16(),
                data: Some(config),
                ..WebResponse::default()
            })
        }
    }
}

pub async fn update(
    Extension(ext): Extension<Ext>, Json(body): Json<cds_config::Config>,
) -> Result<WebResponse<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    if operator.group != Group::Admin {
        return Err(WebError::Forbidden(json!("")));
    }

    let config = cds_db::entity::config::ActiveModel {
        id: Set(1),
        value: Set(json!(body)),
        ..Default::default()
    };

    config.update(get_db()).await?;

    Ok(WebResponse {
        code: StatusCode::OK.as_u16(),
        ..WebResponse::default()
    })
}

pub async fn get_icon() -> impl IntoResponse {
    let path = String::from("configs");
    let filename = String::from("icon.webp");
    match cds_media::get(path, filename).await {
        Ok(data) => Response::builder().body(Body::from(data)).unwrap(),
        Err(_) => {
            Redirect::to("/icon.webp").into_response() // default frontend icon
        }
    }
}

pub async fn save_icon(
    Extension(ext): Extension<Ext>, multipart: Multipart,
) -> Result<WebResponse<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    if operator.group != Group::Admin {
        return Err(WebError::Forbidden(json!("")));
    }
    let path = String::from("configs");
    let filename = String::from("icon.webp");
    let data = handle_image_multipart(multipart).await?;
    cds_media::delete(path.clone(), filename.clone()).await?;
    let data = cds_media::util::img_convert_to_webp(data).await?;
    cds_media::save(path, filename, data)
        .await
        .map_err(|_| WebError::InternalServerError(json!("")))?;

    Ok(WebResponse {
        code: StatusCode::OK.as_u16(),
        ..WebResponse::default()
    })
}

pub async fn delete_icon(Extension(ext): Extension<Ext>) -> Result<WebResponse<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    if operator.group != Group::Admin {
        return Err(WebError::Forbidden(json!("")));
    }
    let path = String::from("configs");
    let filename = String::from("icon.webp");
    cds_media::delete(path.clone(), filename.clone()).await?;

    Ok(WebResponse {
        code: StatusCode::OK.as_u16(),
        ..WebResponse::default()
    })
}
