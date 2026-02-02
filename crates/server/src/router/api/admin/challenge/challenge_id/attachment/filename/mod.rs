use std::sync::Arc;

use axum::{
    Router,
    body::Body,
    extract::State,
    http::{Response, header, StatusCode},
    response::{IntoResponse, Redirect},
};
use serde_json::json;

use crate::{
    extract::Path,
    traits::{AppState, WebError, WebResponse},
};

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", axum::routing::get(get_attachment))
        .route("/", axum::routing::delete(delete_attachment))
}

pub async fn get_attachment(
    State(s): State<Arc<AppState>>,

    Path((challenge_id, filename)): Path<(i64, String)>,
) -> Result<impl IntoResponse, WebError> {
    let _ = crate::util::loader::prepare_challenge(&s.db.conn, challenge_id)
        .await?
        .has_attachment
        .then_some(())
        .ok_or_else(|| WebError::NotFound(json!("challenge_has_not_attachment")))?;

    let path = crate::util::media::build_challenge_attachment_path(challenge_id);

    if s.media.presigned_enabled() {
        let url = s
            .media
            .presign_get(&path, &filename, 3600)
            .await
            .map_err(|_| WebError::NotFound(json!("")))?;
        return Ok(Redirect::temporary(&url).into_response());
    }

    let buffer = s
        .media
        .get(path, filename.clone())
        .await
        .map_err(|_| WebError::NotFound(json!("")))?;

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/octet-stream")
        .header(
            header::CONTENT_DISPOSITION,
            format!("attachment; filename=\"{}\"", filename),
        )
        .body(Body::from(buffer))?)
}

pub async fn delete_attachment(
    State(s): State<Arc<AppState>>,

    Path((challenge_id, filename)): Path<(i64, String)>,
) -> Result<WebResponse<()>, WebError> {
    let _ = crate::util::loader::prepare_challenge(&s.db.conn, challenge_id)
        .await?
        .has_attachment
        .then_some(())
        .ok_or_else(|| WebError::NotFound(json!("challenge_has_not_attachment")))?;

    let path = crate::util::media::build_challenge_attachment_path(challenge_id);
    s.media.delete(path, filename).await?;

    Ok(WebResponse {
        ..Default::default()
    })
}
