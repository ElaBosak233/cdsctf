//! HTTP routing for public IdP avatar access.

use std::sync::Arc;

use axum::{
    extract::State,
    response::{IntoResponse, Redirect},
};
use serde_json::json;

use crate::{
    extract::Path,
    traits::{AppState, WebError},
};

#[utoipa::path(
    get,
    path = "/",
    tag = "idp",
    params(("idp_id" = i64, Path, description = "IdP id")),
    responses(
        (status = 302, description = "Redirect to cached avatar URL"),
        (status = 404, description = "Not found", body = crate::traits::ErrorResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
#[tracing::instrument(skip_all, fields(handler = "get_idp_avatar"))]
pub async fn get_idp_avatar(
    State(s): State<Arc<AppState>>,
    Path(idp_id): Path<i64>,
) -> Result<impl IntoResponse, WebError> {
    let idp = cds_db::idp::find_idp_by_id::<cds_db::Idp>(&s.db.conn, idp_id)
        .await?
        .ok_or(WebError::NotFound(json!("idp_not_found")))?;
    match idp.avatar_hash {
        Some(hash) => Ok(Redirect::to(&format!("/api/media?hash={}", hash))),
        None => Err(WebError::NotFound(json!("avatar_not_found"))),
    }
}
