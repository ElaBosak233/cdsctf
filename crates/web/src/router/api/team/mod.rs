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
    user.group == Group::Admin
        || user
            .teams
            .iter()
            .any(|team| team.id == team_id && team.captain_id == user.id)
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct GetRequest {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub email: Option<String>,
    pub user_id: Option<i64>,
    pub page: Option<u64>,
    pub size: Option<u64>,
}

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
    pub captain_id: i64,
    pub slogan: Option<String>,
}

pub async fn create(
    Extension(ext): Extension<Ext>, VJson(body): VJson<CreateRequest>,
) -> Result<WebResponse<cds_db::transfer::Team>, WebError> {
    let operator = ext
        .operator
        .ok_or(WebError::Unauthorized(serde_json::json!("")))?;
    if !(operator.group == Group::Admin || operator.id == body.captain_id) {
        return Err(WebError::Forbidden(serde_json::json!("")));
    }

    let team = cds_db::entity::team::ActiveModel {
        name: Set(body.name),
        email: Set(Some(body.email)),
        captain_id: Set(body.captain_id),
        slogan: body.slogan.map_or(NotSet, |v| Set(Some(v))),
        ..Default::default()
    }
    .insert(get_db())
    .await?;

    let team = cds_db::transfer::Team::from(team);

    let _ = cds_db::entity::user_team::ActiveModel {
        user_id: Set(body.captain_id),
        team_id: Set(team.id),
        ..Default::default()
    }
    .insert(get_db())
    .await?;

    Ok(WebResponse {
        code: StatusCode::OK.as_u16(),
        data: Some(team),
        ..WebResponse::default()
    })
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
pub struct UpdateRequest {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub email: Option<String>,
    pub captain_id: Option<i64>,
    pub slogan: Option<String>,
}

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
        captain_id: body.captain_id.map_or(NotSet, Set),
        slogan: body.slogan.map_or(NotSet, |v| Set(Some(v))),
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
        ..WebResponse::default()
    })
}

pub async fn delete_user(
    Extension(ext): Extension<Ext>, Path((id, user_id)): Path<(i64, i64)>,
) -> Result<WebResponse<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    if !can_modify_team(operator.clone(), id) && operator.id != user_id {
        return Err(WebError::Forbidden(json!("")));
    }

    let _ = cds_db::entity::user_team::Entity::delete_many()
        .filter(cds_db::entity::user_team::Column::UserId.eq(user_id))
        .filter(cds_db::entity::user_team::Column::TeamId.eq(id))
        .exec(get_db())
        .await?;

    Ok(WebResponse {
        code: StatusCode::OK.as_u16(),
        ..WebResponse::default()
    })
}

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
        ..WebResponse::default()
    })
}

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
        ..WebResponse::default()
    })
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JoinRequest {
    pub user_id: i64,
    pub team_id: i64,
    pub invite_token: String,
}

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
        ..WebResponse::default()
    })
}

pub async fn leave() -> impl IntoResponse {
    ""
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
