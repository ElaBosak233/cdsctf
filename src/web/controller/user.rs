use axum::{
    body::Body,
    extract::{Multipart, Path, Query},
    http::{Response, StatusCode},
    response::IntoResponse,
    Extension, Json,
};
use bcrypt::{hash, DEFAULT_COST};
use mime::Mime;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, Set};
use serde_json::json;

use crate::database::get_db;
use crate::{util::jwt, web::traits::Ext};
use crate::{util::validate, web::traits::Error};

pub async fn get(
    Query(params): Query<crate::model::user::request::FindRequest>,
) -> Result<impl IntoResponse, Error> {
    let (mut users, total) = crate::model::user::find(
        params.id,
        params.name,
        None,
        params.group,
        params.email,
        params.page,
        params.size,
    )
    .await
    .map_err(|err| Error::DatabaseError(err))?;

    for user in users.iter_mut() {
        user.simplify();
    }

    return Ok((
        StatusCode::OK,
        Json(json!({
            "code": StatusCode::OK.as_u16(),
            "data": json!(users),
            "total": total,
        })),
    ));
}

pub async fn create(
    validate::Json(mut body): validate::Json<crate::model::user::request::CreateRequest>,
) -> Result<impl IntoResponse, Error> {
    let hashed_password = hash(body.password, DEFAULT_COST);
    body.password = hashed_password.unwrap();

    let mut user = crate::model::user::ActiveModel::from(body)
        .insert(&get_db())
        .await
        .map_err(|err| Error::DatabaseError(err))?;

    user.simplify();

    return Ok((
        StatusCode::OK,
        Json(json!({
            "code": StatusCode::OK.as_u16(),
            "data": json!(user),
        })),
    ));
}

pub async fn update(
    Extension(ext): Extension<Ext>, Path(id): Path<i64>,
    validate::Json(mut body): validate::Json<crate::model::user::request::UpdateRequest>,
) -> Result<impl IntoResponse, Error> {
    let operator = ext.clone().operator.unwrap();
    body.id = Some(id);
    if !(operator.group == "admin"
        || (operator.id == body.id.unwrap_or(0)
            && (body.group.clone().is_none()
                || operator.group == body.group.clone().unwrap_or("".to_string()))))
    {
        return Err(Error::Forbidden(String::new()));
    }

    if let Some(password) = body.password {
        let hashed_password = hash(password, DEFAULT_COST);
        body.password = Some(hashed_password.unwrap());
    }

    let user = crate::model::user::ActiveModel::from(body)
        .update(&get_db())
        .await
        .map_err(|err| Error::DatabaseError(err))?;

    return Ok((
        StatusCode::OK,
        Json(json!({
            "code": StatusCode::OK.as_u16(),
            "data": json!(user),
        })),
    ));
}

pub async fn delete(Path(id): Path<i64>) -> Result<impl IntoResponse, Error> {
    let _ = crate::model::user::Entity::delete_by_id(id)
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

pub async fn get_teams(Path(id): Path<i64>) -> Result<impl IntoResponse, Error> {
    let teams = crate::model::team::find_by_user_id(id)
        .await
        .map_err(|err| Error::DatabaseError(err))?;

    return Ok((
        StatusCode::OK,
        Json(json!({
            "code": StatusCode::OK.as_u16(),
            "data": json!(teams),
        })),
    ));
}

pub async fn login(
    Json(body): Json<crate::model::user::request::LoginRequest>,
) -> Result<impl IntoResponse, Error> {
    let mut user = crate::model::user::Entity::find()
        .filter(crate::model::user::Column::Username.eq(body.username))
        .one(&get_db())
        .await
        .map_err(|err| Error::DatabaseError(err))?
        .ok_or_else(|| Error::BadRequest(String::from("invalid")))?;

    let hashed_password = user.password.clone().unwrap();
    let is_match = bcrypt::verify(&body.password, &hashed_password).unwrap();

    if !is_match {
        return Err(Error::BadRequest(String::from("invalid")));
    }

    let token = jwt::generate_jwt_token(user.id.clone()).await;
    user.simplify();

    return Ok((
        StatusCode::OK,
        Json(json!({
            "code": StatusCode::OK.as_u16(),
            "data": json!(user),
            "token": token,
        })),
    ));
}

pub async fn register(
    validate::Json(body): validate::Json<crate::model::user::request::RegisterRequest>,
) -> Result<impl IntoResponse, Error> {
    let is_conflict = crate::model::user::Entity::find()
        .filter(crate::model::user::Column::Username.eq(body.username.clone()))
        .count(&get_db())
        .await
        .map_err(|err| Error::DatabaseError(err))?
        > 0;

    if is_conflict {
        return Err(Error::Conflict(String::new()));
    }

    let hashed_password = hash(body.password.clone(), DEFAULT_COST).unwrap();
    let mut user = crate::model::user::ActiveModel::from(body);
    user.password = Set(Some(hashed_password));
    user.group = Set(String::from("user"));

    let user = user
        .insert(&get_db())
        .await
        .map_err(|err| Error::DatabaseError(err))?;

    return Ok((
        StatusCode::OK,
        Json(json!({
            "code": StatusCode::OK.as_u16(),
            "data": json!(user),
        })),
    ));
}

pub async fn get_avatar(Path(id): Path<i64>) -> Result<impl IntoResponse, Error> {
    let path = format!("users/{}/avatar", id);
    match crate::media::scan_dir(path.clone()).await.unwrap().first() {
        Some((filename, _size)) => {
            let buffer = crate::media::get(path, filename.to_string()).await.unwrap();
            return Ok(Response::builder().body(Body::from(buffer)).unwrap());
        }
        None => return Err(Error::NotFound(String::new())),
    }
}

pub async fn get_avatar_metadata(Path(id): Path<i64>) -> Result<impl IntoResponse, Error> {
    let path = format!("users/{}/avatar", id);
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
        None => {
            return Err(Error::NotFound(String::new()));
        }
    }
}

pub async fn save_avatar(
    Extension(ext): Extension<Ext>, Path(id): Path<i64>, mut multipart: Multipart,
) -> Result<impl IntoResponse, Error> {
    let operator = ext.operator.unwrap();
    if operator.group != "admin" && operator.id != id {
        return Err(Error::Forbidden(String::new()));
    }

    let path = format!("users/{}/avatar", id);
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
        .map_err(|_| Error::InternalServerError(String::new()));

    return Ok((
        StatusCode::OK,
        Json(json!({
            "code": StatusCode::OK.as_u16(),
        })),
    ));
}

pub async fn delete_avatar(
    Extension(ext): Extension<Ext>, Path(id): Path<i64>,
) -> Result<impl IntoResponse, Error> {
    let operator = ext.operator.unwrap();
    if operator.group != "admin" && operator.id != id {
        return Err(Error::Forbidden(String::new()));
    }

    let path = format!("users/{}/avatar", id);

    let _ = crate::media::delete(path)
        .await
        .map_err(|_| Error::InternalServerError(String::new()));

    return Ok((
        StatusCode::OK,
        Json(json!({
            "code": StatusCode::OK.as_u16(),
        })),
    ));
}
