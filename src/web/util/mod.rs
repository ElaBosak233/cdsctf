use axum::extract::Multipart;
use mime::Mime;
use serde_json::json;
use crate::web::traits::WebError;

pub mod jwt;
pub mod math;

pub async fn handle_image_multipart(mut multipart: Multipart) -> Result<Vec<u8>, WebError> {
    while let Some(field) = multipart.next_field().await.unwrap() {
        if field.name() == Some("file") {
            let content_type = field.content_type().unwrap().to_string();
            let mime: Mime = content_type.parse().unwrap();
            if mime.type_() != mime::IMAGE {
                return Err(WebError::BadRequest(json!(
                    "forbidden_file_type"
                )));
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
