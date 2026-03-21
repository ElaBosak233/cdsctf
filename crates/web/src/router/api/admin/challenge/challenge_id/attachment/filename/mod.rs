use std::sync::Arc;

use axum::{
    Json, Router,
    body::Body,
    extract::State,
    http::{Response, StatusCode, header},
    response::{IntoResponse, Redirect},
};
use serde_json::json;
use utoipa_axum::{
    router::{OpenApiRouter, UtoipaMethodRouterExt},
    routes,
};

use crate::{
    extract::Path,
    traits::{AppState, EmptyJson, WebError},
};

pub fn router(state: Arc<AppState>) -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::from(Router::new().with_state(state.clone()))
        .routes(routes!(get_attachment).with_state(state.clone()))
        .routes(routes!(delete_attachment).with_state(state.clone()))
}

#[utoipa::path(
    get,
    path = "/",
    tag = "admin-challenge",
    params(
        ("challenge_id" = i64, Path, description = "Challenge id"),
        ("filename" = String, Path, description = "File name"),
    ),
    responses(
        (status = 200, description = "File bytes"),
        (status = 302, description = "Presigned redirect"),
        (status = 404, description = "Not found", body = crate::traits::ErrorResponse),
    )
)]
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

#[utoipa::path(
    delete,
    path = "/",
    tag = "admin-challenge",
    params(
        ("challenge_id" = i64, Path, description = "Challenge id"),
        ("filename" = String, Path, description = "File name"),
    ),
    responses(
        (status = 200, description = "Deleted", body = EmptyJson),
        (status = 404, description = "Not found", body = crate::traits::ErrorResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
pub async fn delete_attachment(
    State(s): State<Arc<AppState>>,
    Path((challenge_id, filename)): Path<(i64, String)>,
) -> Result<Json<EmptyJson>, WebError> {
    let _ = crate::util::loader::prepare_challenge(&s.db.conn, challenge_id)
        .await?
        .has_attachment
        .then_some(())
        .ok_or_else(|| WebError::NotFound(json!("challenge_has_not_attachment")))?;

    let path = crate::util::media::build_challenge_attachment_path(challenge_id);
    s.media.delete(path, filename).await?;

    Ok(Json(EmptyJson::default()))
}
