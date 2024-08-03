use crate::database::get_db;
use crate::web::traits::{Error, Ext};
use axum::body::Body;
use axum::{
    extract::{Multipart, Path, Query},
    http::{Response, StatusCode},
    response::IntoResponse,
    Extension, Json,
};
use mime::Mime;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter, QuerySelect, Set,
};
use serde_json::json;

fn can_modify_team(user: crate::model::user::Model, team_id: i64) -> bool {
    return user.group == "admin"
        || user
            .teams
            .iter()
            .any(|team| team.id == team_id && team.captain_id == user.id);
}

pub async fn get(
    Query(params): Query<crate::model::team::request::FindRequest>,
) -> Result<impl IntoResponse, Error> {
    let (teams, total) = crate::model::team::find(
        params.id,
        params.name,
        params.email,
        params.page,
        params.size,
    )
    .await
    .map_err(|err| Error::DatabaseError(err))?;

    return Ok((
        StatusCode::OK,
        Json(json!({
            "code": StatusCode::OK.as_u16(),
            "data": json!(teams),
            "total": total,
        })),
    ));
}

pub async fn create(
    Extension(ext): Extension<Ext>, Json(body): Json<crate::model::team::request::CreateRequest>,
) -> Result<impl IntoResponse, Error> {
    let operator = ext.operator.unwrap();
    if !(operator.group == "admin" || operator.id == body.captain_id) {
        return Err(Error::Forbidden(String::new()));
    }

    let team = crate::model::team::ActiveModel::from(body)
        .insert(&get_db())
        .await
        .map_err(|err| Error::DatabaseError(err))?;

    let _ = crate::model::user_team::ActiveModel {
        user_id: Set(operator.id),
        team_id: Set(team.id),
        ..Default::default()
    }
    .insert(&get_db())
    .await
    .map_err(|err| Error::DatabaseError(err))?;

    return Ok((
        StatusCode::OK,
        Json(json!({
            "code": StatusCode::OK.as_u16(),
            "data": json!(team),
        })),
    ));
}

pub async fn update(
    Extension(ext): Extension<Ext>, Path(id): Path<i64>,
    Json(mut body): Json<crate::model::team::request::UpdateRequest>,
) -> Result<impl IntoResponse, Error> {
    if !can_modify_team(ext.operator.unwrap(), id) {
        return Err(Error::Forbidden(String::new()));
    }
    body.id = Some(id);

    let team = crate::model::team::ActiveModel::from(body)
        .insert(&get_db())
        .await
        .map_err(|err| Error::DatabaseError(err))?;

    return Ok((
        StatusCode::OK,
        Json(json!({
            "code": StatusCode::OK.as_u16(),
            "data": json!(team),
        })),
    ));
}

pub async fn delete(
    Extension(ext): Extension<Ext>, Path(id): Path<i64>,
) -> Result<impl IntoResponse, Error> {
    if !can_modify_team(ext.operator.unwrap(), id) {
        return Err(Error::Forbidden(String::new()));
    }

    let _ = crate::model::team::Entity::delete_by_id(id)
        .exec(&get_db())
        .await
        .map_err(|err| Error::DatabaseError(err))?;

    return Ok((
        StatusCode::OK,
        Json(json!({
            "code": StatusCode::OK.as_u16(),
        })),
    ));
}

pub async fn create_user(
    Json(body): Json<crate::model::user_team::request::CreateRequest>,
) -> Result<impl IntoResponse, Error> {
    let _ = crate::model::user_team::ActiveModel::from(body)
        .insert(&get_db())
        .await
        .map_err(|err| Error::DatabaseError(err))?;

    return Ok((
        StatusCode::OK,
        Json(json!({
            "code": StatusCode::OK.as_u16(),
        })),
    ));
}

pub async fn delete_user(
    Extension(ext): Extension<Ext>, Path((id, user_id)): Path<(i64, i64)>,
) -> Result<impl IntoResponse, Error> {
    let operator = ext.operator.unwrap();
    if !can_modify_team(operator.clone(), id) && operator.id != user_id {
        return Err(Error::Forbidden(String::new()));
    }

    let _ = crate::model::user_team::Entity::delete_many()
        .filter(crate::model::user_team::Column::UserId.eq(user_id))
        .filter(crate::model::user_team::Column::TeamId.eq(id))
        .exec(&get_db())
        .await
        .map_err(|err| Error::DatabaseError(err))?;

    return Ok((
        StatusCode::OK,
        Json(json!({
            "code": StatusCode::OK.as_u16(),
        })),
    ));
}

pub async fn get_invite_token(
    Extension(ext): Extension<Ext>, Path(id): Path<i64>,
) -> Result<impl IntoResponse, Error> {
    let operator = ext.operator.unwrap();

    if !can_modify_team(operator, id) {
        return Err(Error::Forbidden(String::new()));
    }

    let team = crate::model::team::Entity::find_by_id(id)
        .select_only()
        .column(crate::model::team::Column::InviteToken)
        .one(&get_db())
        .await
        .map_err(|err| Error::DatabaseError(err))?
        .ok_or_else(|| Error::NotFound(String::new()))?;

    return Ok((
        StatusCode::OK,
        Json(json!({
            "code": StatusCode::OK.as_u16(),
            "token": team.invite_token,
        })),
    ));
}

pub async fn update_invite_token(
    Extension(ext): Extension<Ext>, Path(id): Path<i64>,
) -> Result<impl IntoResponse, Error> {
    let operator = ext.operator.unwrap();
    if !can_modify_team(operator, id) {
        return Err(Error::Forbidden(String::new()));
    }

    let mut team = crate::model::team::Entity::find_by_id(id)
        .one(&get_db())
        .await
        .map_err(|err| Error::DatabaseError(err))?
        .ok_or_else(|| Error::NotFound(String::new()))?
        .into_active_model();

    let token = uuid::Uuid::new_v4().simple().to_string();
    team.invite_token = Set(Some(token.clone()));

    let _ = team
        .update(&get_db())
        .await
        .map_err(|err| Error::DatabaseError(err))?;

    return Ok((
        StatusCode::OK,
        Json(json!({
            "code": StatusCode::OK.as_u16(),
            "token": token,
        })),
    ));
}

pub async fn join(
    Json(body): Json<crate::model::user_team::request::JoinRequest>,
) -> Result<impl IntoResponse, Error> {
    let _ = crate::model::user::Entity::find_by_id(body.user_id)
        .one(&get_db())
        .await
        .map_err(|err| Error::DatabaseError(err))?
        .ok_or_else(|| Error::NotFound(String::from("invalid_user_or_team")))?;

    let team = crate::model::team::Entity::find_by_id(body.team_id)
        .one(&get_db())
        .await
        .map_err(|err| Error::DatabaseError(err))?
        .ok_or_else(|| Error::NotFound(String::from("invalid_user_or_team")))?;

    if Some(body.invite_token.clone()) != team.invite_token {
        return Err(Error::BadRequest(String::from("invalid_invite_token")));
    }

    let user_team = crate::model::user_team::ActiveModel::from(body)
        .insert(&get_db())
        .await
        .map_err(|err| Error::DatabaseError(err))?;

    return Ok((
        StatusCode::OK,
        Json(json!({
            "code": StatusCode::OK.as_u16(),
            "data": user_team,
        })),
    ));
}

pub async fn leave() -> impl IntoResponse {
    todo!()
}

pub async fn get_avatar_metadata(Path(id): Path<i64>) -> Result<impl IntoResponse, Error> {
    let path = format!("teams/{}/avatar", id);
    match crate::media::scan_dir(path.clone()).await.unwrap().first() {
        Some((filename, size)) => {
            return Ok((
                StatusCode::OK,
                Json(json!({
                    "code": StatusCode::OK.as_u16(),
                    "data": {
                        "filename": filename,
                        "size": size,
                    },
                })),
            ));
        }
        None => return Err(Error::NotFound(String::new())),
    }
}

pub async fn get_avatar(Path(id): Path<i64>) -> Result<impl IntoResponse, Error> {
    let path = format!("teams/{}/avatar", id);
    match crate::media::scan_dir(path.clone()).await.unwrap().first() {
        Some((filename, _size)) => {
            let buffer = crate::media::get(path, filename.to_string()).await.unwrap();
            return Ok(Response::builder().body(Body::from(buffer)).unwrap());
        }
        None => return Err(Error::NotFound(String::new())),
    }
}

pub async fn save_avatar(
    Extension(ext): Extension<Ext>, Path(id): Path<i64>, mut multipart: Multipart,
) -> Result<impl IntoResponse, Error> {
    let operator = ext.operator.unwrap();
    if !can_modify_team(operator, id) {
        return Err(Error::Forbidden(String::new()));
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
                return Err(Error::BadRequest(String::from("forbidden_file_type")));
            }
            data = match field.bytes().await {
                Ok(bytes) => bytes.to_vec(),
                Err(_err) => {
                    return Err(Error::BadRequest(String::from("size_too_large")));
                }
            };
        }
    }

    crate::media::delete(path.clone()).await.unwrap();

    let _ = crate::media::save(path, filename, data)
        .await
        .map_err(|_| Error::InternalServerError(String::new()))?;

    return Ok((
        StatusCode::OK,
        Json(json!({
            "code": StatusCode::OK.as_u16(),
        })),
    ));
}

pub async fn delete_avatar(
    Extension(ext): Extension<Ext>, Path(id): Path<i64>,
) -> impl IntoResponse {
    let operator = ext.operator.unwrap();
    if !can_modify_team(operator, id) {
        return Err(Error::Forbidden(String::new()));
    }

    let path = format!("teams/{}/avatar", id);

    let _ = crate::media::delete(path)
        .await
        .map_err(|_| Error::InternalServerError(String::new()))?;

    return Ok((
        StatusCode::OK,
        Json(json!({
            "code": StatusCode::OK.as_u16(),
        })),
    ));
}
