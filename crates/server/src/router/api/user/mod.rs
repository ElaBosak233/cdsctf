mod forget;
mod profile;
mod user_id;

use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};
use axum::{Router, http::StatusCode, response::IntoResponse};
use cds_db::{
    User,
    entity::user::Group,
    get_db,
    sea_orm::{
        ActiveModelTrait, ActiveValue::Set, ColumnTrait, Condition, EntityTrait, PaginatorTrait,
        QueryFilter, prelude::Expr, sea_query::Func,
    },
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tower_sessions::Session;
use validator::Validate;

use crate::{
    extract::{Extension, Json},
    traits::{AuthPrincipal, WebError, WebResponse},
};

pub fn router() -> Router {
    Router::new()
        .route("/login", axum::routing::post(user_login))
        .route("/register", axum::routing::post(user_register))
        .route("/logout", axum::routing::post(user_logout))
        .nest("/forget", forget::router())
        .nest("/{user_id}", user_id::router())
        .nest("/profile", profile::router())
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserLoginRequest {
    pub account: String,
    pub password: String,
    pub captcha: Option<cds_captcha::Answer>,
}

pub async fn user_login(
    session: Session,
    Extension(ext): Extension<AuthPrincipal>,
    Json(mut body): Json<UserLoginRequest>,
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
        .into_model::<User>()
        .one(get_db())
        .await?
        .ok_or(WebError::BadRequest(json!("invalid")))?;

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

    session.insert("user_id", user.id).await?;

    Ok(WebResponse {
        data: Some(user),
        ..Default::default()
    })
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
pub struct UserRegisterRequest {
    pub name: String,
    #[validate(length(min = 3, max = 20))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    pub password: String,
    pub captcha: Option<cds_captcha::Answer>,
}

pub async fn user_register(
    Extension(ext): Extension<AuthPrincipal>,
    Json(mut body): Json<UserRegisterRequest>,
) -> Result<WebResponse<User>, WebError> {
    if !cds_db::get_config().await.auth.is_registration_enabled {
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
    if !cds_db::user::is_email_unique(0, &body.email).await? {
        return Err(WebError::Conflict(json!("email_already_exists")));
    }

    body.username = body.username.to_lowercase();
    if !cds_db::user::is_username_unique(0, &body.username).await? {
        return Err(WebError::Conflict(json!("username_already_exists")));
    }

    let hashed_password = Argon2::default()
        .hash_password(body.password.as_bytes(), &SaltString::generate(&mut OsRng))
        .unwrap()
        .to_string();

    let user = cds_db::user::create::<User>(cds_db::entity::user::ActiveModel {
        username: Set(body.username),
        name: Set(body.name),
        email: Set(body.email),
        is_verified: Set(!cds_db::get_config().await.email.is_enabled),
        hashed_password: Set(hashed_password),
        group: Set(
            if cds_db::entity::user::Entity::find().count(get_db()).await? == 0 {
                Group::Admin
            } else {
                Group::User
            },
        ),
        ..Default::default()
    })
    .await?;

    Ok(WebResponse {
        code: StatusCode::OK,
        data: Some(user),
        ..Default::default()
    })
}

pub async fn user_logout(
    session: Session,
    Extension(ext): Extension<AuthPrincipal>,
) -> Result<impl IntoResponse, WebError> {
    let _ = ext.operator.ok_or(WebError::Unauthorized("".into()))?;

    let _ = session.remove::<Option<i64>>("user_id").await?;

    Ok(WebResponse::<()> {
        ..Default::default()
    })
}
