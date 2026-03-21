mod forget;
mod me;
mod user_id;

use std::sync::Arc;

use axum::{Json, Router, extract::State};
use cds_db::{
    Email, User,
    sea_orm::ActiveValue::Set,
    user::{FindUserOptions, Group},
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tower_sessions::Session;
use tracing::debug;
use utoipa_axum::{
    router::{OpenApiRouter, UtoipaMethodRouterExt},
    routes,
};
use validator::Validate;

use crate::{
    extract::{Extension, Json as ReqJson},
    traits::{AppState, AuthPrincipal, EmptyJson, WebError},
    util,
};

pub fn router(state: Arc<AppState>) -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::from(Router::new().with_state(state.clone()))
        .routes(routes!(user_login).with_state(state.clone()))
        .routes(routes!(user_register).with_state(state.clone()))
        .routes(routes!(user_logout).with_state(state.clone()))
        .nest("/forget", forget::router(state.clone()))
        .nest("/{user_id}", user_id::router(state.clone()))
        .nest("/me", me::router(state.clone()))
}

#[derive(Clone, Debug, Serialize, Deserialize, utoipa::ToSchema)]
pub struct UserResponse {
    pub user: User,
}

#[derive(Clone, Debug, Serialize, Deserialize, utoipa::ToSchema)]
pub struct UserLoginRequest {
    pub account: String,
    pub password: String,
    pub captcha: Option<cds_captcha::Answer>,
}

#[utoipa::path(
    post,
    path = "/login",
    tag = "user",
    request_body = UserLoginRequest,
    responses(
        (status = 200, description = "Logged in", body = UserResponse),
        (status = 400, description = "Bad request", body = crate::traits::ErrorResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
pub async fn user_login(
    State(s): State<Arc<AppState>>,
    session: Session,
    Extension(ext): Extension<AuthPrincipal>,
    ReqJson(mut body): ReqJson<UserLoginRequest>,
) -> Result<Json<UserResponse>, WebError> {
    if !s
        .captcha
        .check(&cds_captcha::Answer {
            client_ip: Some(ext.client_ip),
            ..body.captcha.unwrap_or_default()
        })
        .await?
    {
        return Err(WebError::BadRequest(json!("captcha_invalid")));
    }

    body.account = body.account.to_lowercase();

    let user: User = cds_db::user::find_by_account(&s.db.conn, body.account)
        .await?
        .ok_or(WebError::BadRequest(json!("invalid")))?;

    let hashed_password = user.hashed_password.clone();

    if !util::crypto::verify_password(body.password, hashed_password) {
        return Err(WebError::BadRequest(json!("invalid")));
    }

    session.insert("user_id", user.id).await?;

    debug!(
        user_id = user.id,
        username = user.username,
        "User logged in"
    );

    Ok(Json(UserResponse { user }))
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate, utoipa::ToSchema)]
pub struct UserRegisterRequest {
    pub name: String,
    #[validate(length(min = 3, max = 20))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    pub password: String,
    pub captcha: Option<cds_captcha::Answer>,
}

#[utoipa::path(
    post,
    path = "/register",
    tag = "user",
    request_body = UserRegisterRequest,
    responses(
        (status = 200, description = "Registered", body = UserResponse),
        (status = 400, description = "Bad request", body = crate::traits::ErrorResponse),
        (status = 409, description = "Conflict", body = crate::traits::ErrorResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
pub async fn user_register(
    State(s): State<Arc<AppState>>,
    Extension(ext): Extension<AuthPrincipal>,
    ReqJson(mut body): ReqJson<UserRegisterRequest>,
) -> Result<Json<UserResponse>, WebError> {
    if !cds_db::get_config(&s.db.conn)
        .await
        .auth
        .registration_enabled
    {
        return Err(WebError::BadRequest(json!("registration_disabled")));
    }

    if !s
        .captcha
        .check(&cds_captcha::Answer {
            client_ip: Some(ext.client_ip),
            ..body.captcha.unwrap_or_default()
        })
        .await?
    {
        return Err(WebError::BadRequest(json!("captcha_invalid")));
    }

    body.email = body.email.to_lowercase();
    if !cds_db::user::is_email_unique(&s.db.conn, &body.email).await? {
        return Err(WebError::Conflict(json!("email_already_exists")));
    }

    body.username = body.username.to_lowercase();
    if !cds_db::user::is_username_unique(&s.db.conn, 0, &body.username).await? {
        return Err(WebError::Conflict(json!("username_already_exists")));
    }

    let hashed_password = util::crypto::hash_password(body.password);

    let user = cds_db::user::create::<User>(
        &s.db.conn,
        cds_db::user::ActiveModel {
            username: Set(body.username),
            name: Set(body.name),
            hashed_password: Set(hashed_password),
            group: Set(
                if cds_db::user::find::<User>(&s.db.conn, FindUserOptions::default())
                    .await?
                    .1
                    == 0
                {
                    Group::Admin
                } else {
                    Group::User
                },
            ),
            ..Default::default()
        },
    )
    .await?;

    let _ = cds_db::email::create::<Email>(
        &s.db.conn,
        cds_db::email::ActiveModel {
            user_id: Set(user.id),
            email: Set(body.email),
            verified: Set(!cds_db::get_config(&s.db.conn).await.email.enabled),
        },
    )
    .await?;

    debug!(
        user_id = user.id,
        username = user.username,
        "New user registered"
    );

    Ok(Json(UserResponse { user }))
}

#[utoipa::path(
    post,
    path = "/logout",
    tag = "user",
    responses(
        (status = 200, description = "Logged out", body = EmptyJson),
        (status = 401, description = "Unauthorized", body = crate::traits::ErrorResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
pub async fn user_logout(
    session: Session,
    Extension(ext): Extension<AuthPrincipal>,
) -> Result<Json<EmptyJson>, WebError> {
    let _ = ext.operator.ok_or(WebError::Unauthorized("".into()))?;
    let _ = session.remove::<Option<i64>>("user_id").await?;
    Ok(Json(EmptyJson::default()))
}
