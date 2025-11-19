use axum::{
    Router,
    body::Body,
    http::{Response, header},
    response::IntoResponse,
};
use serde_json::json;

use crate::{
    extract::Path,
    traits::{WebError, WebResponse},
};

pub fn router() -> Router {
    Router::new()
        .route("/", axum::routing::get(get_attachment))
        .route("/", axum::routing::delete(delete_attachment))
}

pub async fn get_attachment(
    Path((challenge_id, filename)): Path<(i64, String)>,
) -> Result<impl IntoResponse, WebError> {
    let _ = crate::util::loader::prepare_challenge(challenge_id)
        .await?
        .has_attachment
        .then_some(())
        .ok_or_else(|| WebError::NotFound(json!("challenge_has_not_attachment")))?;

    let path = crate::util::media::build_challenge_attachment_path(challenge_id);
    let buffer = cds_media::get(path.clone(), filename.clone())
        .await
        .map_err(|_| WebError::NotFound(json!("")))?;

    Ok(Response::builder()
        .header(header::CONTENT_TYPE, "application/octet-stream")
        .header(
            header::CONTENT_DISPOSITION,
            format!("attachment; filename=\"{}\"", filename),
        )
        .body(Body::from(buffer))?)
}

pub async fn delete_attachment(
    Path((challenge_id, filename)): Path<(i64, String)>,
) -> Result<WebResponse<()>, WebError> {
    let _ = crate::util::loader::prepare_challenge(challenge_id)
        .await?
        .has_attachment
        .then_some(())
        .ok_or_else(|| WebError::NotFound(json!("challenge_has_not_attachment")))?;

    let path = crate::util::media::build_challenge_attachment_path(challenge_id);
    cds_media::delete(path, filename).await?;

    Ok(WebResponse {
        ..Default::default()
    })
}
