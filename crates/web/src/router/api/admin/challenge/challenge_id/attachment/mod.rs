use axum::{
    Router,
    body::Body,
    extract::{DefaultBodyLimit, Multipart},
    http::{Response, StatusCode, header},
    response::IntoResponse,
};
use serde_json::json;

use crate::{
    extract::Path,
    model::Metadata,
    traits::{WebError, WebResponse},
};

pub fn router() -> Router {
    Router::new()
        .route(
            "/",
            axum::routing::post(save_challenge_attachment)
                .layer(DefaultBodyLimit::max(512 * 1024 * 1024 /* MB */)),
        )
        .route("/", axum::routing::delete(delete_challenge_attachment))
}

pub async fn save_challenge_attachment(
    Path(challenge_id): Path<uuid::Uuid>,
    mut multipart: Multipart,
) -> Result<WebResponse<()>, WebError> {
    let challenge = crate::util::loader::prepare_challenge(challenge_id).await?;

    let path = format!("challenges/{}/attachment", challenge.id);
    let mut filename = String::new();
    let mut data = Vec::<u8>::new();
    while let Some(field) = multipart.next_field().await.unwrap() {
        if field.name() == Some("file") {
            filename = field.file_name().unwrap().to_string();
            data = match field.bytes().await {
                Ok(bytes) => bytes.to_vec(),
                Err(_err) => {
                    return Err(WebError::BadRequest(json!("size_too_large")));
                }
            };
        }
    }

    cds_media::delete_dir(path.clone()).await?;

    cds_media::save(path, filename, data)
        .await
        .map_err(|_| WebError::InternalServerError(json!("")))?;

    Ok(WebResponse {
        code: StatusCode::OK,
        ..Default::default()
    })
}

pub async fn delete_challenge_attachment(
    Path(challenge_id): Path<uuid::Uuid>,
) -> Result<WebResponse<()>, WebError> {
    let challenge = crate::util::loader::prepare_challenge(challenge_id).await?;

    let path = format!("challenges/{}/attachment", challenge.id);

    cds_media::delete_dir(path)
        .await
        .map_err(|_| WebError::InternalServerError(json!("")))?;

    Ok(WebResponse {
        code: StatusCode::OK,
        ..Default::default()
    })
}
