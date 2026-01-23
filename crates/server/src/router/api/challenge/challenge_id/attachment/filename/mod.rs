use std::sync::Arc;

use axum::{
    Router,
    body::Body,
    extract::State,
    http::{Response, header},
    response::IntoResponse,
};
use serde_json::json;

use crate::{
    extract::{Extension, Path},
    traits::{AppState, AuthPrincipal, WebError},
};

pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/", axum::routing::get(get_attachment))
}

pub async fn get_attachment(
    State(ref s): State<Arc<AppState>>,

    Extension(ext): Extension<AuthPrincipal>,
    Path((challenge_id, filename)): Path<(i64, String)>,
) -> Result<impl IntoResponse, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    let _ = crate::util::loader::prepare_challenge(&s.db.conn, challenge_id)
        .await?
        .has_attachment
        .then_some(())
        .ok_or_else(|| WebError::NotFound(json!("challenge_has_not_attachment")))?;

    if !cds_db::util::can_user_access_challenge(&s.db.conn, operator.id, challenge_id).await? {
        return Err(WebError::Forbidden(json!("")));
    }

    let path = crate::util::media::build_challenge_attachment_path(challenge_id);
    let buffer = s
        .media
        .get(path.clone(), filename.clone())
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
