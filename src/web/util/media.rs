use axum::{
    body::Body,
    extract::Multipart,
    http::{Response, StatusCode},
    response::IntoResponse,
};
use serde_json::json;

use crate::{
    media::util::hash,
    web::{
        model::Metadata,
        traits::{WebError, WebResult},
        util::handle_image_multipart,
    },
};

pub async fn get_img(path: String) -> Result<impl IntoResponse, WebError> {
    match crate::media::scan_dir(path.clone()).await?.first() {
        Some((filename, _size)) => {
            let buffer = crate::media::get(path, filename.to_string()).await?;
            Ok(Response::builder().body(Body::from(buffer)).unwrap())
        }
        None => Err(WebError::NotFound(json!(""))),
    }
}

pub async fn get_img_metadata(path: String) -> Result<WebResult<Metadata>, WebError> {
    match crate::media::scan_dir(path.clone()).await?.first() {
        Some((filename, size)) => Ok(WebResult {
            code: StatusCode::OK.as_u16(),
            data: Some(Metadata {
                filename: filename.to_string(),
                size: *size,
            }),
            ..WebResult::default()
        }),
        None => Err(WebError::NotFound(json!(""))),
    }
}

pub async fn save_img(path: String, multipart: Multipart) -> Result<WebResult<()>, WebError> {
    let data = handle_image_multipart(multipart).await?;

    crate::media::delete_dir(path.clone()).await?;

    let data = crate::media::util::img_convert_to_webp(data).await?;
    let filename = format!("{}.webp", hash(data.clone()));

    crate::media::save(path, filename, data)
        .await
        .map_err(|_| WebError::InternalServerError(json!("")))?;

    Ok(WebResult {
        code: StatusCode::OK.as_u16(),
        ..WebResult::default()
    })
}

pub async fn delete_img(path: String) -> Result<WebResult<()>, WebError> {
    crate::media::delete_dir(path)
        .await
        .map_err(|_| WebError::InternalServerError(json!("")))?;

    Ok(WebResult {
        code: StatusCode::OK.as_u16(),
        ..WebResult::default()
    })
}
