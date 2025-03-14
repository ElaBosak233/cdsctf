use axum::{
    body::Body,
    extract::Multipart,
    http::{Response, StatusCode},
    response::IntoResponse,
};
use cds_media::util::hash;
use serde_json::json;

use crate::{
    model::Metadata,
    traits::{WebError, WebResponse},
    util::handle_image_multipart,
};

pub async fn get_img(path: String) -> Result<impl IntoResponse, WebError> {
    match cds_media::scan_dir(path.clone()).await?.first() {
        Some((filename, _size)) => {
            let buffer = cds_media::get(path, filename.to_string()).await?;
            Ok(Response::builder().body(Body::from(buffer)).unwrap())
        }
        None => Err(WebError::NotFound(json!(""))),
    }
}

pub async fn get_img_metadata(path: String) -> Result<WebResponse<Metadata>, WebError> {
    match cds_media::scan_dir(path.clone()).await?.first() {
        Some((filename, size)) => Ok(WebResponse {
            data: Some(Metadata {
                filename: filename.to_string(),
                size: *size,
            }),
            ..Default::default()
        }),
        None => Err(WebError::NotFound(json!(""))),
    }
}

pub async fn save_img(path: String, multipart: Multipart) -> Result<WebResponse<()>, WebError> {
    let data = handle_image_multipart(multipart).await?;

    cds_media::delete_dir(path.clone()).await?;

    let data = cds_media::util::img_convert_to_webp(data).await?;
    let filename = format!("{}.webp", hash(data.clone()));

    cds_media::save(path, filename, data)
        .await
        .map_err(|_| WebError::InternalServerError(json!("")))?;

    Ok(WebResponse {
        code: StatusCode::OK,
        ..Default::default()
    })
}

pub async fn delete_img(path: String) -> Result<WebResponse<()>, WebError> {
    cds_media::delete_dir(path)
        .await
        .map_err(|_| WebError::InternalServerError(json!("")))?;

    Ok(WebResponse {
        code: StatusCode::OK,
        ..Default::default()
    })
}
