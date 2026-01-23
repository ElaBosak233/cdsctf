mod attachment;

use std::sync::Arc;

use axum::{Router, extract::State, http::StatusCode};
use cds_db::{Challenge, DB};
use serde_json::json;

use crate::{
    extract::{Extension, Path},
    traits::{AppState, AuthPrincipal, WebError, WebResponse},
};

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", axum::routing::get(get_challenge))
        .nest("/attachments", attachment::router())
}

pub async fn get_challenge(
    State(ref s): State<Arc<AppState>>,

    Extension(ext): Extension<AuthPrincipal>,
    Path(challenge_id): Path<i64>,
) -> Result<WebResponse<Challenge>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;

    let challenge = crate::util::loader::prepare_challenge(&s.db.conn, challenge_id)
        .await?
        .desensitize();

    if !cds_db::util::can_user_access_challenge(&s.db.conn, operator.id, challenge.id).await? {
        return Err(WebError::Forbidden(json!("")));
    }

    Ok(WebResponse {
        code: StatusCode::OK,
        data: Some(challenge.desensitize()),
        ..Default::default()
    })
}
