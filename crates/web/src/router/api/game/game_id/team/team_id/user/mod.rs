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
        .route("/join", axum::routing::post(join_team))
        .route("/leave", axum::routing::delete(leave_team))
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateTeamUserRequest {
    pub user_id: i64,
}

/// Add a user into a team by given data.
///
/// Only admins can use this function.
///
/// # Prerequisite
/// - Operator is admin.
pub async fn create_team_user(
    Extension(ext): Extension<Ext>, Path((_game_id, team_id)): Path<(i64, i64)>,
    Json(body): Json<CreateTeamUserRequest>,
) -> Result<WebResponse<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    let team = cds_db::transfer::Team::from(
        cds_db::entity::team::Entity::find_by_id(team_id)
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
        code: StatusCode::OK,
        ..Default::default()
    })
}

/// Kick a user from a team by `id` and `user_id`.
///
/// # Prerequisite
/// - Operator is admin.
pub async fn delete_team_user(
    Extension(ext): Extension<Ext>, Path((_game_id, team_id, user_id)): Path<(i64, i64, i64)>,
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
        code: StatusCode::OK,
        ..Default::default()
    })
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JoinTeamRequest {
    pub token: String,
}

pub async fn join_team(
    Extension(ext): Extension<Ext>, Path((_game_id, team_id)): Path<(i64, i64)>,
    Json(body): Json<JoinTeamRequest>,
) -> Result<WebResponse<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;

    let team = cds_db::transfer::Team::from(
        cds_db::entity::team::Entity::find_by_id(team_id)
            .filter(cds_db::entity::team::Column::DeletedAt.is_null())
            .one(get_db())
            .await?
            .ok_or(WebError::BadRequest(json!("team_not_found")))?,
    );

    let criteria = cds_cache::get::<String>(format!("team:{team_id}:token"))
        .await?
        .ok_or(WebError::BadRequest(json!("no_invite_token")))?;

    if criteria != body.token {
        return Err(WebError::BadRequest(json!("invalid_invite_token")));
    }

    let _ = cds_db::entity::team_user::ActiveModel {
        team_id: Set(team.id),
        user_id: Set(operator.id),
    }
    .insert(get_db())
    .await?;

    Ok(WebResponse {
        code: StatusCode::OK,
        ..Default::default()
    })
}

pub async fn leave_team(
    Extension(ext): Extension<Ext>, Path((_game_id, team_id)): Path<(i64, i64)>,
) -> Result<WebResponse<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;

    let team = cds_db::transfer::Team::from(
        cds_db::entity::team::Entity::find_by_id(team_id)
            .filter(cds_db::entity::team::Column::DeletedAt.is_null())
            .one(get_db())
            .await?
            .ok_or(WebError::BadRequest(json!("team_not_found")))?,
    );

    let _ = cds_db::entity::team_user::Entity::delete_many()
        .filter(cds_db::entity::team_user::Column::UserId.eq(operator.id))
        .filter(cds_db::entity::team_user::Column::TeamId.eq(team.id))
        .exec(get_db())
        .await?;

    Ok(WebResponse {
        code: StatusCode::OK,
        ..Default::default()
    })
}
