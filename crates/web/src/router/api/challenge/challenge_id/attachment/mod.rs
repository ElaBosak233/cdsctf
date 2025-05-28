mod filename;

use axum::Router;
use serde_json::json;

use crate::{
    extract::{Extension, Path},
    model::Metadata,
    traits::{Ext, WebError, WebResponse},
};

pub fn router() -> Router {
    Router::new()
        .route("/", axum::routing::get(get_challenge_attachment))
        .nest("/{filename}", filename::router())
}

pub async fn get_challenge_attachment(
    Extension(ext): Extension<Ext>,
    Path(challenge_id): Path<uuid::Uuid>,
) -> Result<WebResponse<Vec<Metadata>>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;

    let _ = crate::util::loader::prepare_challenge(challenge_id)
        .await?
        .has_attachment
        .then_some(())
        .ok_or_else(|| WebError::NotFound(json!("challenge_has_not_attachment")))?;

    if !cds_db::util::can_user_access_challenge(operator.id, challenge_id).await? {
        return Err(WebError::Forbidden(json!("")));
    }

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
