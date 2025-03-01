mod avatar;
mod user;

use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};
use axum::{Router, http::StatusCode, response::IntoResponse};
use cds_db::get_db;
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{Set, Unchanged},
    ColumnTrait, EntityTrait, NotSet, PaginatorTrait, QueryFilter,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use validator::Validate;

use crate::{
    extract::{Extension, Json, Path, VJson},
    traits::{Ext, WebError, WebResponse},
};

pub fn router() -> Router {
    Router::new()
        .route("/", axum::routing::put(update_team))
        .route("/", axum::routing::delete(delete_team))
        .route("/join", axum::routing::post(join_team))
        .route("/quit", axum::routing::delete(quit_team))
        .nest("/avatar", avatar::router())
        .nest("/users", user::router())
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
pub struct UpdateTeamRequest {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub slogan: Option<String>,
    pub description: Option<String>,
}

/// Update a team by given data.
///
/// # Prerequisite
/// - Operator is admin or the members of current team.
pub async fn update_team(
    Extension(ext): Extension<Ext>, Path(team_id): Path<i64>,
    VJson(mut body): VJson<UpdateTeamRequest>,
) -> Result<WebResponse<cds_db::transfer::Team>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    let team = cds_db::entity::team::Entity::find_by_id(team_id)
        .filter(cds_db::entity::team::Column::DeletedAt.is_null())
        .one(get_db())
        .await?
        .map(|team| cds_db::transfer::Team::from(team))
        .ok_or(WebError::BadRequest(json!("team_not_found")))?;

    if !cds_db::util::can_user_modify_team(&operator, &team) {
        return Err(WebError::Forbidden(json!("")));
    }
    body.id = Some(team_id);

    if let Some(password) = body.password {
        let hashed_password = Argon2::default()
            .hash_password(password.as_bytes(), &SaltString::generate(&mut OsRng))
            .unwrap()
            .to_string();
        body.password = Some(hashed_password);
    }

    let team = cds_db::entity::team::ActiveModel {
        id: Unchanged(team.id),
        name: body.name.map_or(NotSet, Set),
        email: body.email.map_or(NotSet, Set),
        hashed_password: body.password.map_or(NotSet, Set),
        slogan: body.slogan.map_or(NotSet, |v| Set(Some(v))),
        description: body.description.map_or(NotSet, |v| Set(Some(v))),
        ..Default::default()
    }
    .update(get_db())
    .await?;
    let team = cds_db::transfer::Team::from(team);

    Ok(WebResponse {
        code: StatusCode::OK,
        data: Some(team),
        ..Default::default()
    })
}

/// Delete a team by `id`.
///
/// The team won't be permanently deleted.
/// For safety, it's marked as deleted using the `is_deleted` field.
///
/// # Prerequisite
/// - Operator is admin or the members of current team.
pub async fn delete_team(
    Extension(ext): Extension<Ext>, Path(team_id): Path<i64>,
) -> Result<WebResponse<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    let team = cds_db::entity::team::Entity::find_by_id(team_id)
        .filter(cds_db::entity::team::Column::DeletedAt.is_null())
        .one(get_db())
        .await?
        .map(|team| cds_db::transfer::Team::from(team))
        .ok_or(WebError::BadRequest(json!("team_not_found")))?;

    if !cds_db::util::can_user_modify_team(&operator, &team) {
        return Err(WebError::Forbidden(json!("")));
    }

    let _ = cds_db::entity::team::ActiveModel {
        id: Unchanged(team_id),
        is_locked: Set(true),
        name: Set(format!("[DELETED]_{}", team.name)),
        deleted_at: Set(Some(chrono::Utc::now().timestamp())),
        ..Default::default()
    }
    .update(get_db())
    .await?;

    Ok(WebResponse {
        code: StatusCode::OK,
        ..Default::default()
    })
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JoinTeamRequest {
    pub team_id: i64,
    pub password: String,
}

/// Join a team by given data.
///
/// The field `user_id` will be overwritten by operator's id.
pub async fn join_team(
    Extension(ext): Extension<Ext>, Path(team_id): Path<i64>, Json(mut body): Json<JoinTeamRequest>,
) -> Result<WebResponse<cds_db::transfer::TeamUser>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    let team = cds_db::entity::team::Entity::find_by_id(team_id)
        .filter(cds_db::entity::team::Column::DeletedAt.is_null())
        .one(get_db())
        .await?
        .ok_or(WebError::BadRequest(json!("team_not_found")))?;

    if Argon2::default()
        .verify_password(
            body.password.as_bytes(),
            &PasswordHash::new(&team.hashed_password).unwrap(),
        )
        .is_err()
    {
        return Err(WebError::BadRequest(json!("invalid_password")));
    }

    let team_user = cds_db::entity::team_user::ActiveModel {
        user_id: Set(operator.id),
        team_id: Set(team_id),
        ..Default::default()
    }
    .insert(get_db())
    .await?;
    let team_user = cds_db::transfer::TeamUser::from(team_user);

    Ok(WebResponse {
        code: StatusCode::OK,
        data: Some(team_user),
        ..Default::default()
    })
}

/// Quit a team by `id`.
///
/// Remove the operator from the current team.
pub async fn quit_team(
    Extension(ext): Extension<Ext>, Path(team_id): Path<i64>,
) -> Result<WebResponse<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    let team = cds_db::entity::team::Entity::find_by_id(team_id)
        .filter(cds_db::entity::team::Column::DeletedAt.is_null())
        .one(get_db())
        .await?
        .ok_or(WebError::BadRequest(json!("team_not_found")))?;

    if cds_db::entity::team_user::Entity::find()
        .filter(cds_db::entity::team_user::Column::TeamId.eq(team.id))
        .count(get_db())
        .await?
        == 1
    {
        return Err(WebError::BadRequest(json!("delete_instead_of_leave")));
    }

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
