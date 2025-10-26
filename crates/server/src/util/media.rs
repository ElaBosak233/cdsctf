use axum::{
    body::Body,
    extract::Multipart,
    http::{HeaderValue, Response, StatusCode},
    response::IntoResponse,
};
use cds_media::util::hash;
use mime::Mime;
use serde_json::json;

use crate::{
    model::Metadata,
    traits::{WebError, WebResponse},
};

pub fn build_challenge_attachment_path(challenge_id: i64) -> String {
    format!("challenges/{}/attachments", challenge_id)
}

pub async fn get_write_up(game_id: i64, team_id: i64) -> Result<impl IntoResponse, WebError> {
    let path = format!("games/{}/teams/{}/writeup", game_id, team_id);
    match cds_media::scan_dir(path.clone()).await?.first() {
        Some((filename, _size)) => {
            let buffer = cds_media::get(path, filename.to_string()).await?;
            let filename = format!("writeup-{}-{}.pdf", game_id, team_id);
            Ok(Response::builder()
                .header(
                    "Content-Disposition",
                    &format!("inline; filename=\"{}\"", filename),
                )
                .header("Content-Type", HeaderValue::from_static("application/pdf"))
                .body(Body::from(buffer))
                .unwrap())
        }
        None => Err(WebError::NotFound(json!(""))),
    }
}

pub async fn get_first_file(path: String) -> Result<impl IntoResponse, WebError> {
    match cds_media::scan_dir(path.clone()).await?.first() {
        Some((filename, _size)) => {
            let buffer = cds_media::get(path, filename.to_string()).await?;
            Ok(Response::builder().body(Body::from(buffer)).unwrap())
        }
        None => Err(WebError::NotFound(json!(""))),
    }
}

pub async fn get_first_file_metadata(path: String) -> Result<WebResponse<Metadata>, WebError> {
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
    let data = handle_multipart(multipart, mime::IMAGE).await?;

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

pub async fn handle_multipart(
    mut multipart: Multipart,
    mime_type: mime::Name<'_>,
) -> Result<Vec<u8>, WebError> {
    while let Some(field) = multipart.next_field().await.unwrap() {
        if field.file_name().is_some() {
            let content_type = field.content_type().unwrap().to_string();
            let mime: Mime = content_type.parse().unwrap();

            if mime.type_() != mime_type && mime.subtype() != mime_type {
                return Err(WebError::BadRequest(json!("forbidden_file_type")));
            }
            let data = match field.bytes().await {
                Ok(bytes) => bytes.to_vec(),
                Err(_err) => {
                    return Err(WebError::BadRequest(json!("size_too_large")));
                }
            };
            return Ok(data);
        }
    }

    Err(WebError::BadRequest(json!("no_file")))
}
