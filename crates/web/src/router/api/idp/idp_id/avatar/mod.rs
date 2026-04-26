//! HTTP routing for public IdP avatar access.

use std::sync::Arc;

use axum::{body::Body, extract::State, http::Response, response::IntoResponse};

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
        (status = 200, description = "IdP avatar image bytes"),
        (status = 404, description = "Not found", body = crate::traits::ErrorResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
#[tracing::instrument(skip_all, fields(handler = "get_idp_avatar"))]
pub async fn get_idp_avatar(
    State(s): State<Arc<AppState>>,
    Path(idp_id): Path<i64>,
) -> Result<impl IntoResponse, WebError> {
    let path = format!("idps/{idp_id}");
    let buffer = s.media.get(path, "avatar".to_owned()).await?;
    Ok(Response::builder().body(Body::from(buffer))?)
}
