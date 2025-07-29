mod attachment;

use axum::{Router, http::StatusCode};
use cds_db::Challenge;
use serde_json::json;

use crate::{
    extract::{Extension, Path},
    traits::{AuthPrincipal, WebError, WebResponse},
};

pub fn router() -> Router {
    Router::new()
        .route("/", axum::routing::get(get_challenge))
        .nest("/attachments", attachment::router())
}

pub async fn get_challenge(
    Extension(ext): Extension<AuthPrincipal>,
    Path(challenge_id): Path<uuid::Uuid>,
) -> Result<WebResponse<Challenge>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;

    let challenge = crate::util::loader::prepare_challenge(challenge_id)
        .await?
        .desensitize();

    if !cds_db::util::can_user_access_challenge(operator.id, challenge.id).await? {
        return Err(WebError::Forbidden(json!("")));
    }

    Ok(WebResponse {
        code: StatusCode::OK,
        data: Some(challenge.desensitize()),
        ..Default::default()
    })
}
