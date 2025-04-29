use axum::{
    Router,
    body::Body,
    http::{Response, StatusCode, header},
    response::IntoResponse,
};
use cds_db::{
    get_db,
    sea_orm::{ColumnTrait, EntityTrait, QueryFilter},
};
use serde_json::json;

use crate::{
    extract::{Extension, Path},
    model::Metadata,
    traits::{Ext, WebError, WebResponse},
};

pub fn router() -> Router {
    Router::new()
        .route("/", axum::routing::get(get_challenge_attachment))
        .route(
            "/metadata",
            axum::routing::get(get_challenge_attachment_metadata),
        )
}

pub async fn get_challenge_attachment(
    Extension(ext): Extension<Ext>,
    Path(challenge_id): Path<uuid::Uuid>,
) -> Result<impl IntoResponse, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;

    let challenge = crate::util::loader::prepare_challenge(challenge_id).await?;
    if !cds_db::util::can_user_access_challenge(operator.id, challenge.id).await? {
        return Err(WebError::Forbidden(json!("")));
    }

    if !challenge.has_attachment {
        return Err(WebError::NotFound(json!("challenge_has_not_attachment")));
    }

    let path = format!("challenges/{}/attachment", challenge_id);
    match cds_media::scan_dir(path.clone()).await?.first() {
        Some((filename, _size)) => {
            let buffer = cds_media::get(path, filename.to_string()).await?;
            Ok(Response::builder()
                .header(header::CONTENT_TYPE, "application/octet-stream")
                .header(
                    header::CONTENT_DISPOSITION,
                    format!("attachment; filename=\"{}\"", filename),
                )
                .body(Body::from(buffer))
                .unwrap())
        }
        None => Err(WebError::NotFound(json!(""))),
    }
}

pub async fn get_challenge_attachment_metadata(
    Extension(ext): Extension<Ext>,
    Path(challenge_id): Path<uuid::Uuid>,
) -> Result<WebResponse<Metadata>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;

    let challenge = crate::util::loader::prepare_challenge(challenge_id).await?;
    if !cds_db::util::can_user_access_challenge(operator.id, challenge.id).await? {
        return Err(WebError::Forbidden(json!("")));
    }

    if !challenge.has_attachment {
        return Err(WebError::NotFound(json!("challenge_has_not_attachment")));
    }

    let path = format!("challenges/{}/attachment", challenge_id);
    match cds_media::scan_dir(path.clone()).await?.first() {
        Some((filename, size)) => Ok(WebResponse {
            code: StatusCode::OK,
            data: Some(Metadata {
                filename: filename.to_string(),
                size: *size,
            }),
            ..Default::default()
        }),
        None => Err(WebError::NotFound(json!(""))),
    }
}
