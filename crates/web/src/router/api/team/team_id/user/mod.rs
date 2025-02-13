use axum::{Router, http::StatusCode};
use cds_db::{entity::user::Group, get_db};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    extract::{Extension, Json, Path},
    traits::{Ext, WebError, WebResponse},
};

pub fn router() -> Router {
    Router::new()
        .route("/", axum::routing::post(create_team_user))
        .route("/{user_id}", axum::routing::delete(delete_team_user))
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateTeamUserRequest {
    pub user_id: i64,
    pub team_id: i64,
}

/// Add a user into a team by given data.
///
/// Only admins can use this function.
///
/// # Prerequisite
/// - Operator is admin.
pub async fn create_team_user(
    Extension(ext): Extension<Ext>, Json(body): Json<CreateTeamUserRequest>,
) -> Result<WebResponse<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    let team = cds_db::transfer::Team::from(
        cds_db::entity::team::Entity::find_by_id(body.team_id)
            .filter(cds_db::entity::team::Column::DeletedAt.is_null())
            .one(get_db())
            .await?
            .ok_or(WebError::BadRequest(json!("team_not_found")))?,
    );

    if operator.group != Group::Admin {
        return Err(WebError::Forbidden(json!("")));
    }

    let _ = cds_db::entity::team_user::ActiveModel {
        user_id: Set(body.user_id),
        team_id: Set(team.id),
    }
    .insert(get_db())
    .await?;

    Ok(WebResponse {
        code: StatusCode::OK.as_u16(),
        ..Default::default()
    })
}

/// Kick a user from a team by `id` and `user_id`.
///
/// # Prerequisite
/// - Operator is admin.
pub async fn delete_team_user(
    Extension(ext): Extension<Ext>, Path((team_id, user_id)): Path<(i64, i64)>,
) -> Result<WebResponse<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    let team = cds_db::transfer::Team::from(
        cds_db::entity::team::Entity::find_by_id(team_id)
            .filter(cds_db::entity::team::Column::DeletedAt.is_null())
            .one(get_db())
            .await?
            .ok_or(WebError::BadRequest(json!("team_not_found")))?,
    );

    if operator.group != Group::Admin || team.deleted_at.is_some() {
        return Err(WebError::Forbidden(json!("")));
    }

    let _ = cds_db::entity::team_user::Entity::delete_many()
        .filter(cds_db::entity::team_user::Column::UserId.eq(user_id))
        .filter(cds_db::entity::team_user::Column::TeamId.eq(team_id))
        .exec(get_db())
        .await?;

    Ok(WebResponse {
        code: StatusCode::OK.as_u16(),
        ..Default::default()
    })
}
