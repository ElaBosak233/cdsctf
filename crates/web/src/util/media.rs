use axum::{
    body::Body,
    extract::Multipart,
    http::{HeaderValue, Response},
    response::IntoResponse,
};
use cds_media::Media;
use mime::Mime;
use serde_json::json;

use crate::traits::WebError;

pub fn build_challenge_attachment_path(challenge_id: i64) -> String {
    format!("challenges/{}/attachments", challenge_id)
}

pub async fn get_write_up(
    media: Media,

    game_id: i64,
    team_id: i64,
) -> Result<impl IntoResponse, WebError> {
    let path = format!("games/{}/teams/{}/writeup", game_id, team_id);
    match media.scan_dir(path.clone()).await?.first() {
        Some((filename, _size)) => {
            let buffer = media.get(path, filename.to_string()).await?;
            let filename = format!("writeup-{}-{}.pdf", game_id, team_id);
            Ok(Response::builder()
                .header(
                    "Content-Disposition",
                    &format!("inline; filename=\"{}\"", filename),
                )
                .header("Content-Type", HeaderValue::from_static("application/pdf"))
                .body(Body::from(buffer))?)
        }
        None => Err(WebError::NotFound(json!(""))),
    }
}

pub async fn handle_multipart(
    mut multipart: Multipart,
    mime_type: mime::Name<'_>,
) -> Result<Vec<u8>, WebError> {
    while let Some(field) = multipart.next_field().await? {
        if field.file_name().is_some() {
            let content_type = match field.content_type() {
                Some(ct) => ct.to_string(),
                None => {
                    return Err(WebError::BadRequest(json!("missing_content_type")));
                }
            };

            let mime: Mime = match content_type.parse() {
                Ok(m) => m,
                Err(_) => {
                    return Err(WebError::BadRequest(json!("invalid_mime_type")));
                }
            };

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
