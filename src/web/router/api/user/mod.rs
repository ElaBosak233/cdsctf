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
use reqwest::StatusCode;
use sea_orm::{
    prelude::Expr,
    sea_query::Func,
    ActiveModelTrait,
    ActiveValue::{NotSet, Set},
    Condition, EntityTrait, PaginatorTrait, QueryFilter,
};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    config,
    db::{entity::user::Group, get_db},
    media::util::hash,
    web::{
        extract::validate,
        model::Metadata,
        traits::{Ext, WebError, WebResult},
        util::{handle_image_multipart, jwt},
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
) -> Result<WebResult<Vec<crate::db::transfer::User>>, WebError> {
    let (mut users, total) = crate::db::transfer::user::find(
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
) -> Result<WebResult<crate::db::transfer::User>, WebError> {
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

    let user = crate::db::entity::user::ActiveModel {
        username: Set(body.username),
        nickname: Set(body.nickname),
        email: Set(body.email),
        hashed_password: Set(hashed_password),
        group: Set(body.group),
        ..Default::default()
    }
    .insert(get_db())
    .await?;
    let mut user = crate::db::transfer::User::from(user);

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
) -> Result<WebResult<crate::db::transfer::User>, WebError> {
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

    let user = crate::db::entity::user::ActiveModel {
        id: Set(body.id.unwrap_or(0)),
        username: body.username.map_or(NotSet, |v| Set(v)),
        nickname: body.nickname.map_or(NotSet, |v| Set(v)),
        email: body.email.map_or(NotSet, |v| Set(v)),
        hashed_password: body.password.map_or(NotSet, |v| Set(v)),
        group: body.group.map_or(NotSet, |v| Set(v)),
        ..Default::default()
    }
    .update(get_db())
    .await?;
    let user = crate::db::transfer::User::from(user);

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

    let _ = crate::db::entity::user::ActiveModel {
        id: Set(id),
        is_deleted: Set(true),
        ..Default::default()
    }
    .update(get_db())
    .await?;

    Ok(WebResult {
        code: StatusCode::OK.as_u16(),
        ..WebResult::default()
    })
}

pub async fn get_teams(
    Extension(ext): Extension<Ext>, Path(id): Path<i64>,
) -> Result<WebResult<Vec<crate::db::transfer::Team>>, WebError> {
    let _ = ext.operator.ok_or(WebError::Unauthorized(String::new()))?;

    let teams = crate::db::transfer::team::find_by_user_id(id).await?;

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

    let user = crate::db::entity::user::Entity::find()
        .filter(
            Condition::any()
                .add(
                    Expr::expr(Func::lower(Expr::col(
                        crate::db::entity::user::Column::Username,
                    )))
                    .eq(body.account.clone()),
                )
                .add(
                    Expr::expr(Func::lower(Expr::col(
                        crate::db::entity::user::Column::Email,
                    )))
                    .eq(body.account.clone()),
                ),
        )
        .one(get_db())
        .await?
        .ok_or_else(|| WebError::BadRequest(String::from("invalid")))?;

    let mut user = crate::db::transfer::User::from(user);

    let hashed_password = user.hashed_password.clone();

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
        format!(
            "token={}; Max-Age={}; Path=/; HttpOnly; SameSite=Strict",
            token,
            chrono::Duration::minutes(config::get_config().await.auth.jwt.expiration).num_seconds()
        )
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
) -> Result<WebResult<crate::db::transfer::User>, WebError> {
    body.email = body.email.to_lowercase();
    body.username = body.username.to_lowercase();

    let is_conflict = crate::db::entity::user::Entity::find()
        .filter(
            Condition::any()
                .add(
                    Expr::expr(Func::lower(Expr::col(
                        crate::db::entity::user::Column::Username,
                    )))
                    .eq(body.username.clone()),
                )
                .add(
                    Expr::expr(Func::lower(Expr::col(
                        crate::db::entity::user::Column::Email,
                    )))
                    .eq(body.email.clone()),
                ),
        )
        .count(get_db())
        .await?
        > 0;

    if is_conflict {
        return Err(WebError::Conflict(String::new()));
    }

    let hashed_password = Argon2::default()
        .hash_password(body.password.as_bytes(), &SaltString::generate(&mut OsRng))
        .unwrap()
        .to_string();

    let user = crate::db::entity::user::ActiveModel {
        username: Set(body.username),
        nickname: Set(body.nickname),
        email: Set(body.email),
        hashed_password: Set(hashed_password),
        group: Set(Group::User),
        ..Default::default()
    }
    .insert(get_db())
    .await?;
    let user = crate::db::transfer::User::from(user);

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
    Extension(ext): Extension<Ext>, Path(id): Path<i64>, multipart: Multipart,
) -> Result<WebResult<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(String::new()))?;
    if operator.group != Group::Admin && operator.id != id {
        return Err(WebError::Forbidden(String::new()));
    }

    let path = format!("users/{}/avatar", id);

    let data = handle_image_multipart(multipart).await?;

    crate::media::delete_dir(path.clone()).await.unwrap();

    let data = crate::media::util::img_convert_to_webp(data).await?;
    let filename = format!("{}.webp", hash(data.clone()));

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

    let _ = crate::media::delete_dir(path)
        .await
        .map_err(|_| WebError::InternalServerError(String::new()));

    Ok(WebResult {
        code: StatusCode::OK.as_u16(),
        ..WebResult::default()
    })
}
