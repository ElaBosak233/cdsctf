use axum::{
    Router,
    body::Body,
    http::{Response, StatusCode, header},
    response::IntoResponse,
};
use cds_db::get_db;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
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
    Extension(ext): Extension<Ext>, Path(challenge_id): Path<uuid::Uuid>,
) -> Result<impl IntoResponse, WebError> {
    let _ = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;

    let challenge = cds_db::entity::challenge::Entity::find_by_id(challenge_id)
        .filter(cds_db::entity::challenge::Column::DeletedAt.is_null())
        .one(get_db())
        .await?
        .ok_or(WebError::BadRequest(json!("challenge_not_found")))?;

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
    Extension(ext): Extension<Ext>, Path(challenge_id): Path<uuid::Uuid>,
) -> Result<WebResponse<Metadata>, WebError> {
    let _ = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;

    let challenge = cds_db::entity::challenge::Entity::find_by_id(challenge_id)
        .filter(cds_db::entity::challenge::Column::DeletedAt.is_null())
        .one(get_db())
        .await?
        .ok_or(WebError::BadRequest(json!("challenge_not_found")))?;

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
