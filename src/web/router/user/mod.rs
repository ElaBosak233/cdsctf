use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};
use axum::{
    body::Body,
    extract::{DefaultBodyLimit, Multipart, Path, Query},
    http::{header::SET_COOKIE, HeaderMap, Response},
    response::IntoResponse,
    Extension, Json, Router,
};
use mime::Mime;
use reqwest::StatusCode;
use sea_orm::{
    prelude::Expr, sea_query::Func, ActiveModelTrait, ActiveValue::NotSet, Condition, EntityTrait,
    PaginatorTrait, QueryFilter, Set,
};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    database::get_db,
    model::user::group::Group,
    util::{jwt, validate},
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
        .route("/:id/teams", axum::routing::get(get_teams))
        .route("/login", axum::routing::post(login))
        .route("/register", axum::routing::post(register))
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetRequest {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub email: Option<String>,
    pub group: Option<String>,
    pub page: Option<u64>,
    pub size: Option<u64>,
}

pub async fn get(
    Query(params): Query<GetRequest>,
) -> Result<WebResult<Vec<crate::model::user::Model>>, WebError> {
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
        user.desensitize();
    }

    Ok(WebResult {
        code: StatusCode::OK.as_u16(),
        data: Some(users),
        total: Some(total),
        ..WebResult::default()
    })
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
pub struct CreateRequest {
    pub username: String,
    pub nickname: String,
    pub email: String,
    pub password: String,
    pub group: Group,
}

pub async fn create(
    Extension(ext): Extension<Ext>, validate::Json(mut body): validate::Json<CreateRequest>,
) -> Result<WebResult<crate::model::user::Model>, WebError> {
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

    user.desensitize();

    Ok(WebResult {
        code: StatusCode::OK.as_u16(),
        data: Some(user),
        ..WebResult::default()
    })
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
pub struct UpdateRequest {
    pub id: Option<i64>,
    #[validate(length(min = 3, max = 20))]
    pub username: Option<String>,
    pub nickname: Option<String>,
    #[validate(email)]
    pub email: Option<String>,
    pub password: Option<String>,
    pub group: Option<Group>,
}

pub async fn update(
    Extension(ext): Extension<Ext>, Path(id): Path<i64>,
    validate::Json(mut body): validate::Json<UpdateRequest>,
) -> Result<WebResult<crate::model::user::Model>, WebError> {
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

    Ok(WebResult {
        code: StatusCode::OK.as_u16(),
        data: Some(user),
        ..WebResult::default()
    })
}

pub async fn delete(
    Extension(ext): Extension<Ext>, Path(id): Path<i64>,
) -> Result<WebResult<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(String::new()))?;
    if !(operator.group == Group::Admin || operator.id == id) {
        return Err(WebError::Forbidden(String::new()));
    }

    let _ = crate::model::user::Entity::delete_by_id(id)
        .exec(&get_db())
        .await?;

    Ok(WebResult {
        code: StatusCode::OK.as_u16(),
        ..WebResult::default()
    })
}

pub async fn get_teams(
    Extension(ext): Extension<Ext>, Path(id): Path<i64>,
) -> Result<WebResult<Vec<crate::model::team::Model>>, WebError> {
    let _ = ext.operator.ok_or(WebError::Unauthorized(String::new()))?;

    let teams = crate::model::team::find_by_user_id(id).await?;

    Ok(WebResult {
        code: StatusCode::OK.as_u16(),
        data: Some(teams),
        ..WebResult::default()
    })
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub account: String,
    pub password: String,
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
    user.desensitize();

    let mut headers = HeaderMap::new();
    headers.insert(
        SET_COOKIE,
        format!("token={}; Path=/; HttpOnly; SameSite=Strict", token)
            .parse()
            .unwrap(),
    );

    Ok((
        StatusCode::OK,
        headers,
        WebResult {
            code: StatusCode::OK.as_u16(),
            data: Some(user),
            ..WebResult::default()
        },
    ))
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(length(min = 3, max = 20))]
    pub username: String,
    pub nickname: String,
    #[validate(email)]
    pub email: String,
    pub password: String,
    pub token: Option<String>,
}

pub async fn register(
    Extension(ext): Extension<Ext>, validate::Json(mut body): validate::Json<RegisterRequest>,
) -> Result<WebResult<crate::model::user::Model>, WebError> {
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

    Ok(WebResult {
        code: StatusCode::OK.as_u16(),
        data: Some(user),
        ..WebResult::default()
    })
}

pub async fn get_avatar(Path(id): Path<i64>) -> Result<impl IntoResponse, WebError> {
    let path = format!("users/{}/avatar", id);
    match crate::media::scan_dir(path.clone()).await.unwrap().first() {
        Some((filename, _size)) => {
            let buffer = crate::media::get(path, filename.to_string()).await.unwrap();
            Ok(Response::builder().body(Body::from(buffer)).unwrap())
        }
        None => Err(WebError::NotFound(String::new())),
    }
}

pub async fn get_avatar_metadata(Path(id): Path<i64>) -> Result<WebResult<Metadata>, WebError> {
    let path = format!("users/{}/avatar", id);
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

pub async fn save_avatar(
    Extension(ext): Extension<Ext>, Path(id): Path<i64>, mut multipart: Multipart,
) -> Result<WebResult<()>, WebError> {
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

    Ok(WebResult {
        code: StatusCode::OK.as_u16(),
        ..WebResult::default()
    })
}

pub async fn delete_avatar(
    Extension(ext): Extension<Ext>, Path(id): Path<i64>,
) -> Result<WebResult<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(String::new()))?;
    if operator.group != Group::Admin && operator.id != id {
        return Err(WebError::Forbidden(String::new()));
    }

    let path = format!("users/{}/avatar", id);

    let _ = crate::media::delete(path)
        .await
        .map_err(|_| WebError::InternalServerError(String::new()));

    Ok(WebResult {
        code: StatusCode::OK.as_u16(),
        ..WebResult::default()
    })
}
