use axum::Router;
use nanoid::nanoid;
use serde_json::json;

use crate::{
    extract::{Extension, Path},
    traits::{AuthPrincipal, WebError, WebResponse},
};

pub fn router() -> Router {
    Router::new()
        .route("/", axum::routing::post(create_token))
        .route("/", axum::routing::get(get_token))
        .route("/", axum::routing::delete(delete_token))
}

/// Create an invitation token.
pub async fn create_token(
    Extension(ext): Extension<AuthPrincipal>,
    Path(game_id): Path<i64>,
) -> Result<WebResponse<String>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    let team = crate::util::loader::prepare_self_team(game_id, operator.id).await?;

    let token = nanoid!(16);
    cds_cache::set_ex(format!("team:{}:invite", team.id), token.clone(), 60 * 60).await?;

    Ok(WebResponse {
        data: Some(token),
        ..Default::default()
    })
}

/// Get invitation token.
pub async fn get_token(
    Extension(ext): Extension<AuthPrincipal>,
    Path(game_id): Path<i64>,
) -> Result<WebResponse<String>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    let team = crate::util::loader::prepare_self_team(game_id, operator.id).await?;
    let token = cds_cache::get::<String>(format!("team:{}:invite", team.id)).await?;

    Ok(WebResponse {
        data: token,
        ..Default::default()
    })
}

/// Delete invitation token.
pub async fn delete_token(
    Extension(ext): Extension<AuthPrincipal>,
    Path(game_id): Path<i64>,
) -> Result<WebResponse<String>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    let team = crate::util::loader::prepare_self_team(game_id, operator.id).await?;
    let token = cds_cache::get_del::<String>(format!("team:{}:invite", team.id)).await?;

    Ok(WebResponse {
        data: token,
        ..Default::default()
    })
}
