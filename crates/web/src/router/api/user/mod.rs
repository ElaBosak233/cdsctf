mod forget;
mod profile;
mod user_id;

use std::str::FromStr;

use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};
use axum::{
    Router,
    http::{HeaderMap, StatusCode, header::SET_COOKIE},
    response::IntoResponse,
};
use cds_db::{
    entity::user::Group,
    get_db,
    sea_orm::{
        ActiveModelTrait, ActiveValue::Set, ColumnTrait, Condition, EntityTrait, Order,
        PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, prelude::Expr, sea_query::Func,
    },
    transfer::User,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use validator::Validate;

use crate::{
    extract::{Extension, Json, Query, VJson},
    traits::{Ext, WebError, WebResponse},
    util::jwt,
};

pub fn router() -> Router {
    Router::new()
        .route("/", axum::routing::get(get_user))
        .route("/", axum::routing::post(create_user))
        .route("/login", axum::routing::post(user_login))
        .route("/register", axum::routing::post(user_register))
        .route("/logout", axum::routing::post(user_logout))
        .nest("/forget", forget::router())
        .nest("/{user_id}", user_id::router())
        .nest("/profile", profile::router())
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetUserRequest {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub email: Option<String>,
    pub group: Option<Group>,
    pub page: Option<u64>,
    pub size: Option<u64>,
    pub sorts: Option<String>,
}

pub async fn get_user(
    Query(params): Query<GetUserRequest>,
) -> Result<WebResponse<Vec<User>>, WebError> {
    let mut sql = cds_db::entity::user::Entity::find();

    if let Some(id) = params.id {
        sql = sql.filter(cds_db::entity::user::Column::Id.eq(id));
    }

    if let Some(name) = params.name {
        let pattern = format!("%{}%", name);
        let condition = Condition::any()
            .add(cds_db::entity::user::Column::Username.like(&pattern))
            .add(cds_db::entity::user::Column::Nickname.like(&pattern));
        sql = sql.filter(condition);
    }

    if let Some(group) = params.group {
        sql = sql.filter(cds_db::entity::user::Column::Group.eq(group));
    }

    if let Some(email) = params.email {
        sql = sql.filter(cds_db::entity::user::Column::Email.eq(email));
    }

    sql = sql.filter(cds_db::entity::user::Column::DeletedAt.is_null());

    let total = sql.clone().count(get_db()).await?;

    if let Some(sorts) = params.sorts {
        let sorts = sorts.split(",").collect::<Vec<&str>>();
        for sort in sorts {
            let col = match cds_db::entity::user::Column::from_str(sort.replace("-", "").as_str()) {
                Ok(col) => col,
                Err(_) => continue,
            };
            if sort.starts_with("-") {
                sql = sql.order_by(col, Order::Desc);
            } else {
                sql = sql.order_by(col, Order::Asc);
            }
        }
    }

    if let (Some(page), Some(size)) = (params.page, params.size) {
        let offset = (page - 1) * size;
        sql = sql.offset(offset).limit(size);
    }

    let users = sql.all(get_db()).await?;
    let mut users = users.into_iter().map(User::from).collect::<Vec<User>>();

    for user in users.iter_mut() {
        user.desensitize();
    }

    Ok(WebResponse {
        code: StatusCode::OK,
        data: Some(users),
        total: Some(total),
        ..Default::default()
    })
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
pub struct CreateUserRequest {
    #[validate(length(min = 3, max = 20))]
    pub username: String,
    pub nickname: String,
    #[validate(email)]
    pub email: String,
    pub password: String,
    pub group: Group,
}

pub async fn create_user(
    Extension(ext): Extension<Ext>, VJson(mut body): VJson<CreateUserRequest>,
) -> Result<WebResponse<User>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    if operator.group != Group::Admin {
        return Err(WebError::Unauthorized(json!("")));
    }

    body.email = body.email.to_lowercase();
    if !cds_db::util::is_user_email_unique(0, &body.email).await? {
        return Err(WebError::Conflict(json!("email_already_exists")));
    }

    body.username = body.username.to_lowercase();
    if !cds_db::util::is_user_username_unique(0, &body.username).await? {
        return Err(WebError::Conflict(json!("username_already_exists")));
    }

    let hashed_password = Argon2::default()
        .hash_password(body.password.as_bytes(), &SaltString::generate(&mut OsRng))
        .unwrap()
        .to_string();

    let user = cds_db::entity::user::ActiveModel {
        username: Set(body.username),
        nickname: Set(body.nickname),
        email: Set(body.email),
        hashed_password: Set(hashed_password),
        group: Set(body.group),
        ..Default::default()
    }
    .insert(get_db())
    .await?;
    let mut user = cds_db::transfer::User::from(user);

    user.desensitize();

    Ok(WebResponse {
        code: StatusCode::OK,
        data: Some(user),
        ..Default::default()
    })
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserLoginRequest {
    pub account: String,
    pub password: String,
    pub captcha: Option<cds_captcha::Answer>,
}

pub async fn user_login(
    Extension(ext): Extension<Ext>, Json(mut body): Json<UserLoginRequest>,
) -> Result<impl IntoResponse, WebError> {
    if !cds_captcha::check(&cds_captcha::Answer {
        client_ip: Some(ext.client_ip),
        ..body.captcha.unwrap_or_default()
    })
    .await?
    {
        return Err(WebError::BadRequest(json!("captcha_invalid")));
    }

    body.account = body.account.to_lowercase();

    let user = cds_db::entity::user::Entity::find()
        .filter(
            Condition::any()
                .add(
                    Expr::expr(Func::lower(Expr::col(
                        cds_db::entity::user::Column::Username,
                    )))
                    .eq(body.account.clone()),
                )
                .add(
                    Expr::expr(Func::lower(Expr::col(cds_db::entity::user::Column::Email)))
                        .eq(body.account.clone()),
                ),
        )
        .filter(cds_db::entity::user::Column::DeletedAt.is_null())
        .one(get_db())
        .await?
        .ok_or(WebError::BadRequest(json!("invalid")))?;

    let mut user = cds_db::transfer::User::from(user);

    let hashed_password = user.hashed_password.clone();

    if Argon2::default()
        .verify_password(
            body.password.as_bytes(),
            &PasswordHash::new(&hashed_password).unwrap(),
        )
        .is_err()
    {
        return Err(WebError::BadRequest(json!("invalid")));
    }

    let token = jwt::generate_jwt_token(user.id).await;
    user.desensitize();

    let mut headers = HeaderMap::new();
    headers.insert(
        SET_COOKIE,
        format!(
            "token={}; Max-Age={}; Path=/; HttpOnly; SameSite=Strict",
            token,
            chrono::Duration::seconds(cds_config::get_constant().jwt.expiration).num_seconds()
        )
        .parse()
        .unwrap(),
    );

    Ok((StatusCode::OK, headers, WebResponse {
        code: StatusCode::OK,
        data: Some(user),
        ..Default::default()
    }))
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
pub struct UserRegisterRequest {
    #[validate(length(min = 3, max = 20))]
    pub username: String,
    pub nickname: String,
    #[validate(email)]
    pub email: String,
    pub password: String,
    pub captcha: Option<cds_captcha::Answer>,
}

pub async fn user_register(
    Extension(ext): Extension<Ext>, Json(mut body): Json<UserRegisterRequest>,
) -> Result<WebResponse<User>, WebError> {
    if !cds_config::get_variable().auth.is_registration_enabled {
        return Err(WebError::BadRequest(json!("registration_disabled")));
    }

    if !cds_captcha::check(&cds_captcha::Answer {
        client_ip: Some(ext.client_ip),
        ..body.captcha.unwrap_or_default()
    })
    .await?
    {
        return Err(WebError::BadRequest(json!("captcha_invalid")));
    }

    body.email = body.email.to_lowercase();
    if !cds_db::util::is_user_email_unique(0, &body.email).await? {
        return Err(WebError::Conflict(json!("email_already_exists")));
    }

    body.username = body.username.to_lowercase();
    if !cds_db::util::is_user_username_unique(0, &body.username).await? {
        return Err(WebError::Conflict(json!("username_already_exists")));
    }

    let hashed_password = Argon2::default()
        .hash_password(body.password.as_bytes(), &SaltString::generate(&mut OsRng))
        .unwrap()
        .to_string();

    let user = cds_db::entity::user::ActiveModel {
        username: Set(body.username),
        nickname: Set(body.nickname),
        email: Set(body.email),
        is_verified: Set(!cds_config::get_variable().email.is_enabled),
        hashed_password: Set(hashed_password),
        group: Set(
            if cds_db::entity::user::Entity::find().count(get_db()).await? == 0 {
                Group::Admin
            } else {
                Group::User
            },
        ),
        ..Default::default()
    }
    .insert(get_db())
    .await?;
    let user = cds_db::transfer::User::from(user);

    Ok(WebResponse {
        code: StatusCode::OK,
        data: Some(user),
        ..Default::default()
    })
}

pub async fn user_logout(Extension(ext): Extension<Ext>) -> Result<impl IntoResponse, WebError> {
    let _ = ext.operator.ok_or(WebError::Unauthorized("".into()))?;

    let mut headers = HeaderMap::new();
    headers.insert(
        SET_COOKIE,
        format!(
            "token=unknown; Max-Age={}; Path=/; HttpOnly; SameSite=Strict",
            chrono::Duration::minutes(0).num_seconds()
        )
        .parse()
        .unwrap(),
    );

    Ok((StatusCode::OK, headers, WebResponse::<()> {
        code: StatusCode::OK,
        ..Default::default()
    }))
}
