use axum::{
    body::Body,
    extract::{DefaultBodyLimit, Multipart, Path, Query},
    http::{Response, StatusCode},
    response::IntoResponse,
    Extension, Json, Router,
};
use mime::Mime;
use sea_orm::{
    ActiveModelTrait, ActiveValue::NotSet, ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter,
    QuerySelect, Set,
};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    database::get_db,
    model::user::group::Group,
    web::{
        model::Metadata,
        traits::{Ext, WebError, WebResult},
    },
};

pub fn router() -> Router {
    Router::new()
        .route("/", axum::routing::get(get))
        .route("/", axum::routing::post(create))
        .route("/:id", axum::routing::put(update))
        .route("/:id", axum::routing::delete(delete))
        .route("/:id/users", axum::routing::post(create_user))
        .route("/:id/users/:user_id", axum::routing::delete(delete_user))
        .route("/:id/invite", axum::routing::get(get_invite_token))
        .route("/:id/invite", axum::routing::put(update_invite_token))
        .route("/:id/join", axum::routing::post(join))
        .route("/:id/leave", axum::routing::delete(leave))
        .route("/:id/avatar", axum::routing::get(get_avatar))
        .route(
            "/:id/avatar/metadata",
            axum::routing::get(get_avatar_metadata),
        )
        .route(
            "/:id/avatar",
            axum::routing::post(save_avatar)
                .layer(DefaultBodyLimit::max(3 * 1024 * 1024 /* MB */)),
        )
        .route("/:id/avatar", axum::routing::delete(delete_avatar))
}

fn can_modify_team(user: crate::model::user::Model, team_id: i64) -> bool {
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
) -> Result<WebResult<Vec<crate::model::team::Model>>, WebError> {
    let (teams, total) = crate::model::team::find(
        params.id,
        params.name,
        params.email,
        params.page,
        params.size,
    )
    .await?;

    Ok(WebResult {
        code: StatusCode::OK.as_u16(),
        data: Some(teams),
        total: Some(total),
        ..WebResult::default()
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
    Extension(ext): Extension<Ext>, Json(body): Json<CreateRequest>,
) -> Result<WebResult<crate::model::team::Model>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(String::new()))?;
    if !(operator.group == Group::Admin || operator.id == body.captain_id) {
        return Err(WebError::Forbidden(String::new()));
    }

    let team = crate::model::team::ActiveModel {
        name: Set(body.name),
        email: Set(Some(body.email)),
        captain_id: Set(body.captain_id),
        slogan: body.slogan.map_or(NotSet, |v| Set(Some(v))),
        ..Default::default()
    }
    .insert(&get_db())
    .await?;

    let _ = crate::model::user_team::ActiveModel {
        user_id: Set(body.captain_id),
        team_id: Set(team.id),
        ..Default::default()
    }
    .insert(&get_db())
    .await?;

    Ok(WebResult {
        code: StatusCode::OK.as_u16(),
        data: Some(team),
        ..WebResult::default()
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
    Extension(ext): Extension<Ext>, Path(id): Path<i64>, Json(mut body): Json<UpdateRequest>,
) -> Result<WebResult<crate::model::team::Model>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(String::new()))?;

    if !can_modify_team(operator, id) {
        return Err(WebError::Forbidden(String::new()));
    }
    body.id = Some(id);

    let team = crate::model::team::ActiveModel {
        id: body.id.map_or(NotSet, |v| Set(v)),
        name: body.name.map_or(NotSet, |v| Set(v)),
        email: body.email.map_or(NotSet, |v| Set(Some(v))),
        captain_id: body.captain_id.map_or(NotSet, |v| Set(v)),
        slogan: body.slogan.map_or(NotSet, |v| Set(Some(v))),
        ..Default::default()
    }
    .insert(&get_db())
    .await?;

    Ok(WebResult {
        code: StatusCode::OK.as_u16(),
        data: Some(team),
        ..WebResult::default()
    })
}

pub async fn delete(
    Extension(ext): Extension<Ext>, Path(id): Path<i64>,
) -> Result<WebResult<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(String::new()))?;

    if !can_modify_team(operator, id) {
        return Err(WebError::Forbidden(String::new()));
    }

    let _ = crate::model::team::Entity::delete_by_id(id)
        .exec(&get_db())
        .await?;

    Ok(WebResult {
        code: StatusCode::OK.as_u16(),
        ..WebResult::default()
    })
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub user_id: i64,
    pub team_id: i64,
}

pub async fn create_user(
    Extension(ext): Extension<Ext>, Json(body): Json<CreateUserRequest>,
) -> Result<WebResult<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(String::new()))?;
    if operator.group != Group::Admin {
        return Err(WebError::Forbidden(String::new()));
    }

    let _ = crate::model::user_team::ActiveModel {
        user_id: Set(body.user_id),
        team_id: Set(body.team_id),
    }
    .insert(&get_db())
    .await?;

    Ok(WebResult {
        code: StatusCode::OK.as_u16(),
        ..WebResult::default()
    })
}

pub async fn delete_user(
    Extension(ext): Extension<Ext>, Path((id, user_id)): Path<(i64, i64)>,
) -> Result<WebResult<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(String::new()))?;
    if !can_modify_team(operator.clone(), id) && operator.id != user_id {
        return Err(WebError::Forbidden(String::new()));
    }

    let _ = crate::model::user_team::Entity::delete_many()
        .filter(crate::model::user_team::Column::UserId.eq(user_id))
        .filter(crate::model::user_team::Column::TeamId.eq(id))
        .exec(&get_db())
        .await?;

    Ok(WebResult {
        code: StatusCode::OK.as_u16(),
        ..WebResult::default()
    })
}

pub async fn get_invite_token(
    Extension(ext): Extension<Ext>, Path(id): Path<i64>,
) -> Result<WebResult<String>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(String::new()))?;

    if !can_modify_team(operator, id) {
        return Err(WebError::Forbidden(String::new()));
    }

    let team = crate::model::team::Entity::find_by_id(id)
        .select_only()
        .column(crate::model::team::Column::InviteToken)
        .one(&get_db())
        .await?
        .ok_or_else(|| WebError::NotFound(String::new()))?;

    Ok(WebResult {
        code: StatusCode::OK.as_u16(),
        data: team.invite_token,
        ..WebResult::default()
    })
}

pub async fn update_invite_token(
    Extension(ext): Extension<Ext>, Path(id): Path<i64>,
) -> Result<WebResult<String>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(String::new()))?;
    if !can_modify_team(operator, id) {
        return Err(WebError::Forbidden(String::new()));
    }

    let mut team = crate::model::team::Entity::find_by_id(id)
        .one(&get_db())
        .await?
        .ok_or_else(|| WebError::NotFound(String::new()))?
        .into_active_model();

    let token = uuid::Uuid::new_v4().simple().to_string();
    team.invite_token = Set(Some(token.clone()));

    let _ = team.update(&get_db()).await?;

    Ok(WebResult {
        code: StatusCode::OK.as_u16(),
        data: Some(token),
        ..WebResult::default()
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
) -> Result<WebResult<crate::model::user_team::Model>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(String::new()))?;

    body.user_id = operator.id;

    let _ = crate::model::user::Entity::find_by_id(body.user_id)
        .one(&get_db())
        .await?
        .ok_or_else(|| WebError::NotFound(String::from("invalid_user_or_team")))?;

    let team = crate::model::team::Entity::find_by_id(body.team_id)
        .one(&get_db())
        .await?
        .ok_or_else(|| WebError::NotFound(String::from("invalid_user_or_team")))?;

    if Some(body.invite_token.clone()) != team.invite_token {
        return Err(WebError::BadRequest(String::from("invalid_invite_token")));
    }

    let user_team = crate::model::user_team::ActiveModel {
        user_id: Set(body.user_id),
        team_id: Set(body.team_id),
        ..Default::default()
    }
    .insert(&get_db())
    .await?;

    Ok(WebResult {
        code: StatusCode::OK.as_u16(),
        data: Some(user_team),
        ..WebResult::default()
    })
}

pub async fn leave() -> impl IntoResponse {
    todo!()
}

pub async fn get_avatar_metadata(Path(id): Path<i64>) -> Result<WebResult<Metadata>, WebError> {
    let path = format!("teams/{}/avatar", id);
    match crate::media::scan_dir(path.clone()).await.unwrap().first() {
        Some((filename, size)) => Ok(WebResult {
            code: StatusCode::OK.as_u16(),
            data: Some(Metadata {
                filename: filename.to_string(),
                size: *size,
            }),
            ..WebResult::default()
        }),
        None => Err(WebError::NotFound(String::new())),
    }
}

pub async fn get_avatar(Path(id): Path<i64>) -> Result<impl IntoResponse, WebError> {
    let path = format!("teams/{}/avatar", id);
    match crate::media::scan_dir(path.clone()).await.unwrap().first() {
        Some((filename, _size)) => {
            let buffer = crate::media::get(path, filename.to_string()).await.unwrap();
            Ok(Response::builder().body(Body::from(buffer)).unwrap())
        }
        None => Err(WebError::NotFound(String::new())),
    }
}

pub async fn save_avatar(
    Extension(ext): Extension<Ext>, Path(id): Path<i64>, mut multipart: Multipart,
) -> Result<WebResult<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(String::new()))?;
    if !can_modify_team(operator, id) {
        return Err(WebError::Forbidden(String::new()));
    }

    let path = format!("teams/{}/avatar", id);
    let mut filename = String::new();
    let mut data = Vec::<u8>::new();
    while let Some(field) = multipart.next_field().await.unwrap() {
        if field.name() == Some("file") {
            filename = field.file_name().unwrap().to_string();
            let content_type = field.content_type().unwrap().to_string();
            let mime: Mime = content_type.parse().unwrap();
            if mime.type_() != mime::IMAGE {
                return Err(WebError::BadRequest(String::from("forbidden_file_type")));
            }
            data = match field.bytes().await {
                Ok(bytes) => bytes.to_vec(),
                Err(_err) => {
                    return Err(WebError::BadRequest(String::from("size_too_large")));
                }
            };
        }
    }

    crate::media::delete(path.clone()).await.unwrap();

    let _ = crate::media::save(path, filename, data)
        .await
        .map_err(|_| WebError::InternalServerError(String::new()))?;

    Ok(WebResult {
        code: StatusCode::OK.as_u16(),
        ..WebResult::default()
    })
}

pub async fn delete_avatar(
    Extension(ext): Extension<Ext>, Path(id): Path<i64>,
) -> Result<WebResult<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(String::new()))?;
    if !can_modify_team(operator, id) {
        return Err(WebError::Forbidden(String::new()));
    }

    let path = format!("teams/{}/avatar", id);

    let _ = crate::media::delete(path)
        .await
        .map_err(|_| WebError::InternalServerError(String::new()))?;

    Ok(WebResult {
        code: StatusCode::OK.as_u16(),
        ..WebResult::default()
    })
}
