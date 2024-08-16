use std::net::SocketAddr;

use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};
use axum::{
    body::Body,
    extract::{ConnectInfo, Multipart, Path, Query},
    http::{HeaderMap, Response, StatusCode},
    response::IntoResponse,
    Extension, Json,
};
use mime::Mime;
use sea_orm::{
    prelude::Expr, sea_query::Func, ActiveModelTrait, ActiveValue::NotSet, Condition, EntityTrait,
    PaginatorTrait, QueryFilter, Set,
};

use crate::web::model::{user::*, Metadata};
use crate::{database::get_db, model::user::group::Group};
use crate::{util::jwt, web::traits::Ext};
use crate::{util::validate, web::traits::WebError};

pub async fn get(Query(params): Query<GetRequest>) -> Result<impl IntoResponse, WebError> {
    let (mut users, total) = crate::model::user::find(
        params.id,
        params.name,
        None,
        params.group,
        params.email,
        params.page,
        params.size,
    )
    .await?;

    for user in users.iter_mut() {
        user.simplify();
    }

    return Ok((
        StatusCode::OK,
        Json(GetResponse {
            code: StatusCode::OK.as_u16(),
            data: users,
            total: total,
        }),
    ));
}

pub async fn create(
    Extension(ext): Extension<Ext>, validate::Json(mut body): validate::Json<CreateRequest>,
) -> Result<impl IntoResponse, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(String::new()))?;
    if operator.group != Group::Admin {
        return Err(WebError::Unauthorized(String::new()));
    }

    body.email = body.email.to_lowercase();
    body.username = body.username.to_lowercase();

    let hashed_password = Argon2::default()
        .hash_password(body.password.as_bytes(), &SaltString::generate(&mut OsRng))
        .unwrap()
        .to_string();

    body.password = hashed_password;

    let mut user = crate::model::user::ActiveModel {
        username: Set(body.username),
        nickname: Set(body.nickname),
        email: Set(body.email),
        password: Set(body.password),
        group: Set(body.group),
        ..Default::default()
    }
    .insert(&get_db())
    .await?;

    user.simplify();

    return Ok((
        StatusCode::OK,
        Json(CreateResponse {
            code: StatusCode::OK.as_u16(),
            data: user,
        }),
    ));
}

pub async fn update(
    Extension(ext): Extension<Ext>, Path(id): Path<i64>,
    validate::Json(mut body): validate::Json<UpdateRequest>,
) -> Result<impl IntoResponse, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(String::new()))?;
    body.id = Some(id);
    if !(operator.group == Group::Admin
        || (operator.id == body.id.unwrap_or(0)
            && (body.group.clone().is_none() || operator.group == body.group.clone().unwrap())))
    {
        return Err(WebError::Forbidden(String::new()));
    }

    if let Some(password) = body.password {
        let hashed_password = Argon2::default()
            .hash_password(password.as_bytes(), &SaltString::generate(&mut OsRng))
            .unwrap()
            .to_string();
        body.password = Some(hashed_password);
    }

    let user = crate::model::user::ActiveModel {
        id: Set(body.id.unwrap_or(0)),
        username: body.username.map_or(NotSet, |v| Set(v)),
        nickname: body.nickname.map_or(NotSet, |v| Set(v)),
        email: body.email.map_or(NotSet, |v| Set(v)),
        password: body.password.map_or(NotSet, |v| Set(v)),
        group: body.group.map_or(NotSet, |v| Set(v)),
        ..Default::default()
    }
    .update(&get_db())
    .await?;

    return Ok((
        StatusCode::OK,
        Json(UpdateResponse {
            code: StatusCode::OK.as_u16(),
            data: user,
        }),
    ));
}

pub async fn delete(
    Extension(ext): Extension<Ext>, Path(id): Path<i64>,
) -> Result<impl IntoResponse, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(String::new()))?;
    if !(operator.group == Group::Admin || operator.id == id) {
        return Err(WebError::Forbidden(String::new()));
    }

    let _ = crate::model::user::Entity::delete_by_id(id)
        .exec(&get_db())
        .await?;

    return Ok((
        StatusCode::OK,
        Json(DeleteResponse {
            code: StatusCode::OK.as_u16(),
        }),
    ));
}

pub async fn get_teams(
    Extension(ext): Extension<Ext>, Path(id): Path<i64>,
) -> Result<impl IntoResponse, WebError> {
    let _ = ext.operator.ok_or(WebError::Unauthorized(String::new()))?;

    let teams = crate::model::team::find_by_user_id(id).await?;

    return Ok((
        StatusCode::OK,
        Json(GetTeamResponse {
            code: StatusCode::OK.as_u16(),
            data: teams,
        }),
    ));
}

pub async fn login(Json(mut body): Json<LoginRequest>) -> Result<impl IntoResponse, WebError> {
    body.account = body.account.to_lowercase();

    let mut user = crate::model::user::Entity::find()
        .filter(
            Condition::any()
                .add(
                    Expr::expr(Func::lower(Expr::col(crate::model::user::Column::Username)))
                        .eq(body.account.clone()),
                )
                .add(
                    Expr::expr(Func::lower(Expr::col(crate::model::user::Column::Email)))
                        .eq(body.account.clone()),
                ),
        )
        .one(&get_db())
        .await?
        .ok_or_else(|| WebError::BadRequest(String::from("invalid")))?;

    let hashed_password = user.password.clone();

    if Argon2::default()
        .verify_password(
            body.password.as_bytes(),
            &PasswordHash::new(&hashed_password).unwrap(),
        )
        .is_err()
    {
        return Err(WebError::BadRequest(String::from("invalid")));
    }

    let token = jwt::generate_jwt_token(user.id.clone()).await;
    user.simplify();

    return Ok((
        StatusCode::OK,
        Json(LoginResponse {
            code: StatusCode::OK.as_u16(),
            data: user,
            token: token,
        }),
    ));
}

pub async fn register(
    Extension(ext): Extension<Ext>, validate::Json(mut body): validate::Json<RegisterRequest>,
) -> Result<impl IntoResponse, WebError> {
    body.email = body.email.to_lowercase();
    body.username = body.username.to_lowercase();

    let is_conflict = crate::model::user::Entity::find()
        .filter(
            Condition::any()
                .add(
                    Expr::expr(Func::lower(Expr::col(crate::model::user::Column::Username)))
                        .eq(body.username.clone()),
                )
                .add(
                    Expr::expr(Func::lower(Expr::col(crate::model::user::Column::Email)))
                        .eq(body.email.clone()),
                ),
        )
        .count(&get_db())
        .await?
        > 0;

    if is_conflict {
        return Err(WebError::Conflict(String::new()));
    }

    if crate::config::get_config().auth.registration.captcha {
        let captcha = crate::captcha::new().unwrap();
        let token = body
            .token
            .ok_or(WebError::BadRequest(String::from("invalid_captcha_token")))?;
        if !captcha.verify(token, ext.client_ip).await {
            return Err(WebError::BadRequest(String::from("captcha_failed")));
        }
    }

    let hashed_password = Argon2::default()
        .hash_password(body.password.as_bytes(), &SaltString::generate(&mut OsRng))
        .unwrap()
        .to_string();

    let user = crate::model::user::ActiveModel {
        username: Set(body.username),
        nickname: Set(body.nickname),
        email: Set(body.email),
        password: Set(hashed_password),
        group: Set(Group::User),
        ..Default::default()
    }
    .insert(&get_db())
    .await?;

    return Ok((
        StatusCode::OK,
        Json(RegisterResponse {
            code: StatusCode::OK.as_u16(),
            data: user,
        }),
    ));
}

pub async fn get_avatar(Path(id): Path<i64>) -> Result<impl IntoResponse, WebError> {
    let path = format!("users/{}/avatar", id);
    match crate::media::scan_dir(path.clone()).await.unwrap().first() {
        Some((filename, _size)) => {
            let buffer = crate::media::get(path, filename.to_string()).await.unwrap();
            return Ok(Response::builder().body(Body::from(buffer)).unwrap());
        }
        None => return Err(WebError::NotFound(String::new())),
    }
}

pub async fn get_avatar_metadata(Path(id): Path<i64>) -> Result<impl IntoResponse, WebError> {
    let path = format!("users/{}/avatar", id);
    match crate::media::scan_dir(path.clone()).await.unwrap().first() {
        Some((filename, size)) => {
            return Ok((
                StatusCode::OK,
                Json(GetAvatarMetadataResponse {
                    code: StatusCode::OK.as_u16(),
                    data: Metadata {
                        filename: filename.to_string(),
                        size: *size,
                    },
                }),
            ));
        }
        None => {
            return Err(WebError::NotFound(String::new()));
        }
    }
}

pub async fn save_avatar(
    Extension(ext): Extension<Ext>, Path(id): Path<i64>, mut multipart: Multipart,
) -> Result<impl IntoResponse, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(String::new()))?;
    if operator.group != Group::Admin && operator.id != id {
        return Err(WebError::Forbidden(String::new()));
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
        .map_err(|_| WebError::InternalServerError(String::new()));

    return Ok((
        StatusCode::OK,
        Json(SaveAvatarResponse {
            code: StatusCode::OK.as_u16(),
        }),
    ));
}

pub async fn delete_avatar(
    Extension(ext): Extension<Ext>, Path(id): Path<i64>,
) -> Result<impl IntoResponse, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(String::new()))?;
    if operator.group != Group::Admin && operator.id != id {
        return Err(WebError::Forbidden(String::new()));
    }

    let path = format!("users/{}/avatar", id);

    let _ = crate::media::delete(path)
        .await
        .map_err(|_| WebError::InternalServerError(String::new()));

    return Ok((
        StatusCode::OK,
        Json(DeleteAvatarResponse {
            code: StatusCode::OK.as_u16(),
        }),
    ));
}
