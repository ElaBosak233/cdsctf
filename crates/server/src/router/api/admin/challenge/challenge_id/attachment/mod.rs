mod filename;

use axum::{
    Router,
    extract::{DefaultBodyLimit, Multipart},
    http::StatusCode,
};
use serde_json::json;

use crate::{
    extract::Path,
    model::Metadata,
    traits::{WebError, WebResponse},
};

pub fn router() -> Router {
    Router::new()
        .route("/", axum::routing::get(get_challenge_attachment))
        .route(
            "/",
            axum::routing::post(save_challenge_attachment)
                .layer(DefaultBodyLimit::max(512 * 1024 * 1024 /* MB */)),
        )
        .nest("/{filename}", filename::router())
}

pub async fn get_challenge_attachment(
    Path(challenge_id): Path<i64>,
) -> Result<WebResponse<Vec<Metadata>>, WebError> {
    let _ = crate::util::loader::prepare_challenge(challenge_id)
        .await?
        .has_attachment
        .then_some(())
        .ok_or_else(|| WebError::NotFound(json!("challenge_has_not_attachment")))?;

    let path = crate::util::media::build_challenge_attachment_path(challenge_id);
    let metadata = cds_media::scan_dir(path.clone())
        .await?
        .into_iter()
        .map(|(filename, size)| Metadata {
            filename: filename.to_string(),
            size,
        })
        .collect::<Vec<Metadata>>();

    Ok(WebResponse {
        data: Some(metadata),
        ..Default::default()
    })
}

pub async fn save_challenge_attachment(
    Path(challenge_id): Path<i64>,
    mut multipart: Multipart,
) -> Result<WebResponse<()>, WebError> {
    let _ = crate::util::loader::prepare_challenge(challenge_id).await?;

    let path = crate::util::media::build_challenge_attachment_path(challenge_id);
    let mut filename = String::new();
    let mut data = Vec::<u8>::new();
    while let Some(field) = multipart.next_field().await.unwrap() {
        if let Some(name) = field.file_name() {
            filename = name.to_string();
            data = match field.bytes().await {
                Ok(bytes) => bytes.to_vec(),
                _ => return Err(WebError::BadRequest(json!("size_too_large"))),
            };
            break;
        }
    }

    cds_media::save(path, filename, data)
        .await
        .map_err(|_| WebError::InternalServerError(json!("")))?;

    Ok(WebResponse {
        code: StatusCode::OK,
        ..Default::default()
    })
}
