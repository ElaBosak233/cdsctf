use axum::{
    Router,
    extract::{DefaultBodyLimit, Multipart},
    http::StatusCode,
    response::IntoResponse,
};
use cds_db::{entity::user::Group, get_db};
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set},
    ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter, QuerySelect,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use validator::Validate;

use crate::{
    extract::{Extension, Json, Path, Query, VJson},
    model::Metadata,
    traits::{Ext, WebError, WebResponse},
    util,
};

pub fn router() -> Router {
    Router::new()
        .route("/", axum::routing::get(get))
        .route("/", axum::routing::post(create))
        .route("/register", axum::routing::post(register))
        .route("/{id}", axum::routing::put(update))
        .route("/{id}", axum::routing::delete(delete))
        .route("/{id}/users", axum::routing::post(create_user))
        .route("/{id}/users/{user_id}", axum::routing::delete(delete_user))
        .route("/{id}/invite", axum::routing::get(get_invite_token))
        .route("/{id}/invite", axum::routing::put(update_invite_token))
        .route("/{id}/join", axum::routing::post(join))
        .route("/{id}/leave", axum::routing::delete(leave))
        .route("/{id}/avatar", axum::routing::get(get_avatar))
        .route(
            "/{id}/avatar/metadata",
            axum::routing::get(get_avatar_metadata),
        )
        .route(
            "/{id}/avatar",
            axum::routing::post(save_avatar)
                .layer(DefaultBodyLimit::max(3 * 1024 * 1024 /* MB */)),
        )
        .route("/{id}/avatar", axum::routing::delete(delete_avatar))
}

fn can_modify_team(user: cds_db::transfer::User, team_id: i64) -> bool {
    user.group == Group::Admin || user.teams.iter().any(|team| team.id == team_id)
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct GetRequest {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub email: Option<String>,
    pub page: Option<u64>,
    pub size: Option<u64>,
}

/// Get teams by given params.
pub async fn get(
    Query(params): Query<GetRequest>,
) -> Result<WebResponse<Vec<cds_db::transfer::Team>>, WebError> {
    let (teams, total) = cds_db::transfer::team::find(
        params.id,
        params.name,
        params.email,
        params.page,
        params.size,
    )
    .await?;

    Ok(WebResponse {
        code: StatusCode::OK.as_u16(),
        data: Some(teams),
        total: Some(total),
        ..WebResponse::default()
    })
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
pub struct CreateRequest {
    pub name: String,
    pub email: String,
    pub slogan: Option<String>,
    pub description: Option<String>,
}

/// Create a team by given data.
///
/// Only admins can use this function.
///
/// No users will be added to the newly created team.
pub async fn create(
    Extension(ext): Extension<Ext>, VJson(body): VJson<CreateRequest>,
) -> Result<WebResponse<cds_db::transfer::Team>, WebError> {
    let _ = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;

    let team = cds_db::entity::team::ActiveModel {
        name: Set(body.name),
        email: Set(Some(body.email)),
        slogan: body.slogan.map_or(NotSet, |v| Set(Some(v))),
        description: body.description.map_or(NotSet, |v| Set(Some(v))),
        ..Default::default()
    }
    .insert(get_db())
    .await?;

    let team = cds_db::transfer::Team::from(team);

    Ok(WebResponse {
        code: StatusCode::OK.as_u16(),
        data: Some(team),
        ..Default::default()
    })
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
pub struct RegisterRequest {
    pub name: String,
    pub email: String,
    pub slogan: Option<String>,
    pub description: Option<String>,
}

/// Register a team by given data.
///
/// The operator of this function will be added into the newly created team.
/// So you should call this function in general teams page, not in admin panel.
pub async fn register(
    Extension(ext): Extension<Ext>, VJson(body): VJson<RegisterRequest>,
) -> Result<WebResponse<cds_db::transfer::Team>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;

    let team = cds_db::entity::team::ActiveModel {
        name: Set(body.name),
        email: Set(Some(body.email)),
        slogan: body.slogan.map_or(NotSet, |v| Set(Some(v))),
        description: body.description.map_or(NotSet, |v| Set(Some(v))),
        ..Default::default()
    }
    .insert(get_db())
    .await?;

    let _ = cds_db::entity::user_team::ActiveModel {
        user_id: Set(operator.id),
        team_id: Set(team.id.clone()),
        ..Default::default()
    }
    .insert(get_db())
    .await?;

    let team = cds_db::transfer::team::find_by_ids(vec![team.id])
        .await?
        .first()
        .unwrap()
        .to_owned();

    Ok(WebResponse {
        code: StatusCode::OK.as_u16(),
        data: Some(team),
        ..Default::default()
    })
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
pub struct UpdateRequest {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub email: Option<String>,
    pub slogan: Option<String>,
    pub description: Option<String>,
}

/// Update a team by given data.
///
/// Admins or the members of team can update any field of current team.
pub async fn update(
    Extension(ext): Extension<Ext>, Path(id): Path<i64>, VJson(mut body): VJson<UpdateRequest>,
) -> Result<WebResponse<cds_db::transfer::Team>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;

    if !can_modify_team(operator, id) {
        return Err(WebError::Forbidden(json!("")));
    }
    body.id = Some(id);

    let team = cds_db::entity::team::ActiveModel {
        id: body.id.map_or(NotSet, Set),
        name: body.name.map_or(NotSet, Set),
        email: body.email.map_or(NotSet, |v| Set(Some(v))),
        slogan: body.slogan.map_or(NotSet, |v| Set(Some(v))),
        description: body.description.map_or(NotSet, |v| Set(Some(v))),
        ..Default::default()
    }
    .update(get_db())
    .await?;
    let team = cds_db::transfer::Team::from(team);

    Ok(WebResponse {
        code: StatusCode::OK.as_u16(),
        data: Some(team),
        ..WebResponse::default()
    })
}

/// Delete a team by `id`.
///
/// Admins or the members of team can delete current team.
///
/// The team won't be permanently deleted;
/// for safety, it's marked as deleted using the `is_deleted` field.
pub async fn delete(
    Extension(ext): Extension<Ext>, Path(id): Path<i64>,
) -> Result<WebResponse<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;

    if !can_modify_team(operator, id) {
        return Err(WebError::Forbidden(json!("")));
    }

    let _ = cds_db::entity::team::ActiveModel {
        id: Set(id),
        is_deleted: Set(true),
        ..Default::default()
    }
    .update(get_db())
    .await?;

    Ok(WebResponse {
        code: StatusCode::OK.as_u16(),
        ..WebResponse::default()
    })
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub user_id: i64,
    pub team_id: i64,
}

/// Add a user into a team by given data.
///
/// Only admins can use this function.
pub async fn create_user(
    Extension(ext): Extension<Ext>, Json(body): Json<CreateUserRequest>,
) -> Result<WebResponse<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    if operator.group != Group::Admin {
        return Err(WebError::Forbidden(json!("")));
    }

    let _ = cds_db::entity::user_team::ActiveModel {
        user_id: Set(body.user_id),
        team_id: Set(body.team_id),
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
/// Only admins can use this function.
pub async fn delete_user(
    Extension(ext): Extension<Ext>, Path((id, user_id)): Path<(i64, i64)>,
) -> Result<WebResponse<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    if operator.group != Group::Admin {
        return Err(WebError::Forbidden(json!("")));
    }

    let _ = cds_db::entity::user_team::Entity::delete_many()
        .filter(cds_db::entity::user_team::Column::UserId.eq(user_id))
        .filter(cds_db::entity::user_team::Column::TeamId.eq(id))
        .exec(get_db())
        .await?;

    Ok(WebResponse {
        code: StatusCode::OK.as_u16(),
        ..Default::default()
    })
}

/// Get the invite_token of a team by `id`.
pub async fn get_invite_token(
    Extension(ext): Extension<Ext>, Path(id): Path<i64>,
) -> Result<WebResponse<String>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;

    if !can_modify_team(operator, id) {
        return Err(WebError::Forbidden(json!("")));
    }

    let team = cds_db::entity::team::Entity::find_by_id(id)
        .select_only()
        .column(cds_db::entity::team::Column::InviteToken)
        .one(get_db())
        .await?
        .ok_or_else(|| WebError::NotFound(json!("")))?;

    Ok(WebResponse {
        code: StatusCode::OK.as_u16(),
        data: team.invite_token,
        ..Default::default()
    })
}

/// Update the invite_token of a team by `id`.
///
/// Admins or the members of team can update the invite_token.
pub async fn update_invite_token(
    Extension(ext): Extension<Ext>, Path(id): Path<i64>,
) -> Result<WebResponse<String>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    if !can_modify_team(operator, id) {
        return Err(WebError::Forbidden(json!("")));
    }

    let mut team = cds_db::entity::team::Entity::find_by_id(id)
        .one(get_db())
        .await?
        .ok_or_else(|| WebError::NotFound(json!("")))?
        .into_active_model();

    let token = uuid::Uuid::new_v4().simple().to_string();
    team.invite_token = Set(Some(token.clone()));

    let _ = team.update(get_db()).await?;

    Ok(WebResponse {
        code: StatusCode::OK.as_u16(),
        data: Some(token),
        ..Default::default()
    })
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JoinRequest {
    pub user_id: i64,
    pub team_id: i64,
    pub invite_token: String,
}

/// Join a team by given data.
///
/// The field `user_id` will be overwritten by operator's id.
pub async fn join(
    Extension(ext): Extension<Ext>, Json(mut body): Json<JoinRequest>,
) -> Result<WebResponse<cds_db::transfer::UserTeam>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;

    body.user_id = operator.id;

    let _ = cds_db::entity::user::Entity::find_by_id(body.user_id)
        .one(get_db())
        .await?
        .ok_or_else(|| WebError::NotFound(json!("invalid_user_or_team")))?;

    let team = cds_db::entity::team::Entity::find_by_id(body.team_id)
        .one(get_db())
        .await?
        .ok_or_else(|| WebError::NotFound(json!("invalid_user_or_team")))?;

    if Some(body.invite_token.clone()) != team.invite_token {
        return Err(WebError::BadRequest(json!("invalid_invite_token")));
    }

    let user_team = cds_db::entity::user_team::ActiveModel {
        user_id: Set(body.user_id),
        team_id: Set(body.team_id),
        ..Default::default()
    }
    .insert(get_db())
    .await?;
    let user_team = cds_db::transfer::UserTeam::from(user_team);

    Ok(WebResponse {
        code: StatusCode::OK.as_u16(),
        data: Some(user_team),
        ..Default::default()
    })
}

/// Leave a team by `id`.
///
/// Remove the operator from the current team.
pub async fn leave(Extension(ext): Extension<Ext>, Path(id): Path<i64>) -> Result<WebResponse<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;

    let _ = cds_db::entity::user_team::Entity::delete_many()
        .filter(cds_db::entity::user_team::Column::UserId.eq(operator.id))
        .filter(cds_db::entity::user_team::Column::TeamId.eq(id))
        .exec(get_db())
        .await?;

    Ok(WebResponse {
        code: StatusCode::OK.as_u16(),
        ..Default::default()
    })
}

pub async fn get_avatar(Path(id): Path<i64>) -> Result<impl IntoResponse, WebError> {
    let path = format!("teams/{}/avatar", id);

    util::media::get_img(path).await
}

pub async fn get_avatar_metadata(Path(id): Path<i64>) -> Result<WebResponse<Metadata>, WebError> {
    let path = format!("teams/{}/avatar", id);

    util::media::get_img_metadata(path).await
}

pub async fn save_avatar(
    Extension(ext): Extension<Ext>, Path(id): Path<i64>, multipart: Multipart,
) -> Result<WebResponse<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    if !can_modify_team(operator, id) {
        return Err(WebError::Forbidden(json!("")));
    }

    let path = format!("teams/{}/avatar", id);

    util::media::save_img(path, multipart).await
}

pub async fn delete_avatar(
    Extension(ext): Extension<Ext>, Path(id): Path<i64>,
) -> Result<WebResponse<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    if !can_modify_team(operator, id) {
        return Err(WebError::Forbidden(json!("")));
    }

    let path = format!("teams/{}/avatar", id);

    util::media::delete_img(path).await
}
