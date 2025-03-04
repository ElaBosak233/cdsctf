use axum::Router;
use cds_db::{entity::user::Group, get_db};
use nanoid::nanoid;
use sea_orm::{ActiveModelTrait, EntityTrait};
use serde_json::json;

use crate::{
    extract::{Extension, Path},
    traits::{Ext, WebError, WebResponse},
};

pub fn router() -> Router {
    Router::new()
        .route("/", axum::routing::post(create_token))
        .route("/", axum::routing::get(get_token))
        .route("/", axum::routing::delete(delete_token))
}

/// Create an invitation token.
///
/// # Prerequisite
/// - Operator is admin or one of the members of current team.
pub async fn create_token(
    Extension(ext): Extension<Ext>, Path((_game_id, team_id)): Path<(i64, i64)>,
) -> Result<WebResponse<String>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    let team = cds_db::transfer::Team::from(
        cds_db::entity::team::Entity::find_by_id(team_id)
            .one(get_db())
            .await?
            .ok_or(WebError::BadRequest(json!("team_not_found")))?,
    );

    if operator.group != Group::Admin && !cds_db::util::is_user_in_team(operator.id, team.id).await?
    {
        return Err(WebError::Forbidden(json!("")));
    }

    let token = nanoid!(16);

    cds_cache::set_ex(format!("team:{}:invite", team.id), token.clone(), 60 * 60).await?;

    Ok(WebResponse {
        data: Some(token),
        ..Default::default()
    })
}

/// Get invitation token.
///
/// # Prerequisite
/// - Operator is admin or one of the members of current team.
pub async fn get_token(
    Extension(ext): Extension<Ext>, Path((_game_id, team_id)): Path<(i64, i64)>,
) -> Result<WebResponse<String>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    let team = cds_db::transfer::Team::from(
        cds_db::entity::team::Entity::find_by_id(team_id)
            .one(get_db())
            .await?
            .ok_or(WebError::BadRequest(json!("team_not_found")))?,
    );

    if operator.group != Group::Admin && !cds_db::util::is_user_in_team(operator.id, team.id).await?
    {
        return Err(WebError::Forbidden(json!("")));
    }

    let token = cds_cache::get::<String>(format!("team:{}:invite", team.id)).await?;

    Ok(WebResponse {
        data: token,
        ..Default::default()
    })
}

/// Delete invitation token.
///
/// # Prerequisite
/// - Operator is admin or one of the members of current team.
pub async fn delete_token(
    Extension(ext): Extension<Ext>, Path((_game_id, team_id)): Path<(i64, i64)>,
) -> Result<WebResponse<String>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    let team = cds_db::transfer::Team::from(
        cds_db::entity::team::Entity::find_by_id(team_id)
            .one(get_db())
            .await?
            .ok_or(WebError::BadRequest(json!("team_not_found")))?,
    );

    if operator.group != Group::Admin && !cds_db::util::is_user_in_team(operator.id, team.id).await?
    {
        return Err(WebError::Forbidden(json!("")));
    }

    let token = cds_cache::get_del::<String>(format!("team:{}:invite", team.id)).await?;

    Ok(WebResponse {
        data: token,
        ..Default::default()
    })
}
