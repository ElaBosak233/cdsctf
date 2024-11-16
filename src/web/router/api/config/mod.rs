use axum::{
    body::Body,
    extract::Multipart,
    http::{Response, StatusCode},
    response::{IntoResponse, Redirect},
    Extension, Json, Router,
};
use sea_orm::{ActiveModelTrait, ActiveValue::Set};

use crate::{
    config::get_config,
    db::get_db,
    model::user::group::Group,
    web::{
        traits::{Ext, WebError, WebResult},
        util::handle_image_multipart,
    },
};

pub fn router() -> Router {
    Router::new()
        .route("/", axum::routing::get(get))
        .route("/", axum::routing::put(update))
        .route("/icon", axum::routing::get(get_icon))
        .route("/icon", axum::routing::post(save_icon))
        .route("/icon", axum::routing::delete(delete_icon))
}

pub async fn get(
    Extension(ext): Extension<Ext>,
) -> Result<WebResult<crate::config::Config>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(String::new()))?;
    if operator.group != Group::Admin {
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
    if operator.group != Group::Admin {
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

pub async fn get_icon() -> impl IntoResponse {
    let path = String::from("configs");
    let filename = String::from("icon.webp");
    match crate::media::get(path, filename).await {
        Ok(data) => Response::builder().body(Body::from(data)).unwrap(),
        Err(_) => {
            Redirect::to("/icon.webp").into_response() // default frontend icon
        }
    }
}

pub async fn save_icon(
    Extension(ext): Extension<Ext>, multipart: Multipart,
) -> Result<WebResult<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(String::new()))?;
    if operator.group != Group::Admin {
        return Err(WebError::Forbidden(String::new()));
    }
    let path = String::from("configs");
    let filename = String::from("icon.webp");
    let data = handle_image_multipart(multipart).await?;
    crate::media::delete(path.clone(), filename.clone())
        .await
        .unwrap();
    let data = crate::media::util::img_convert_to_webp(data).await?;
    let _ = crate::media::save(path, filename, data)
        .await
        .map_err(|_| WebError::InternalServerError(String::new()))?;

    Ok(WebResult {
        code: StatusCode::OK.as_u16(),
        ..WebResult::default()
    })
}

pub async fn delete_icon(Extension(ext): Extension<Ext>) -> Result<WebResult<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(String::new()))?;
    if operator.group != Group::Admin {
        return Err(WebError::Forbidden(String::new()));
    }
    let path = String::from("configs");
    let filename = String::from("icon.webp");
    crate::media::delete(path.clone(), filename.clone())
        .await
        .unwrap();

    Ok(WebResult {
        code: StatusCode::OK.as_u16(),
        ..WebResult::default()
    })
}
