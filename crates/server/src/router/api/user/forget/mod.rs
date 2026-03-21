use std::sync::Arc;

use axum::{Json, Router, extract::State};
use cds_db::{Email, User};
use cds_media::config::email::EmailType;
use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use serde_json::json;
use utoipa_axum::{
    router::{OpenApiRouter, UtoipaMethodRouterExt},
    routes,
};
use validator::Validate;

use crate::{
    extract::Json as ReqJson,
    traits::{AppState, EmptySuccess, WebError},
    util,
};

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", axum::routing::post(user_forget))
        .route("/send", axum::routing::post(send_forget_email))
}

pub fn openapi_router(state: Arc<AppState>) -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::from(Router::new().with_state(state.clone()))
        .routes(routes!(user_forget).with_state(state.clone()))
        .routes(routes!(send_forget_email).with_state(state.clone()))
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate, utoipa::ToSchema)]
pub struct UserForgetRequest {
    #[validate(email)]
    pub email: String,
    pub code: String,
    pub password: String,
}

#[utoipa::path(
    post,
    path = "/",
    tag = "user",
    request_body = UserForgetRequest,
    responses(
        (status = 200, description = "Password reset", body = EmptySuccess),
        (status = 400, description = "Bad request", body = crate::traits::ApiJsonError),
        (status = 500, description = "Server error", body = crate::traits::ApiJsonError),
    )
)]
pub async fn user_forget(
    State(s): State<Arc<AppState>>,
    ReqJson(body): ReqJson<UserForgetRequest>,
) -> Result<Json<EmptySuccess>, WebError> {
    let user: User = cds_db::user::find_by_email(&s.db.conn, body.email.to_lowercase())
        .await?
        .ok_or(WebError::BadRequest("user_not_found".into()))?;

    let code = s
        .cache
        .get::<String>(format!("mailbox:{}:code", body.email.to_lowercase()))
        .await?
        .ok_or(WebError::BadRequest("email_code_expired".into()))?;

    if code != body.code {
        return Err(WebError::BadRequest(json!("email_code_incorrect")));
    }

    let hashed_password = util::crypto::hash_password(body.password);

    cds_db::user::update_password(&s.db.conn, user.id, hashed_password).await?;

    let _ = s
        .cache
        .get_del::<String>(format!("mailbox:{}:code", body.email.to_lowercase()))
        .await?;

    Ok(Json(EmptySuccess::default()))
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate, utoipa::ToSchema)]
pub struct UserSendForgetEmailRequest {
    #[validate(email)]
    pub email: String,
}

#[utoipa::path(
    post,
    path = "/send",
    tag = "user",
    request_body = UserSendForgetEmailRequest,
    responses(
        (status = 200, description = "Email queued", body = EmptySuccess),
        (status = 400, description = "Bad request", body = crate::traits::ApiJsonError),
        (status = 500, description = "Server error", body = crate::traits::ApiJsonError),
    )
)]
pub async fn send_forget_email(
    State(s): State<Arc<AppState>>,
    ReqJson(body): ReqJson<UserSendForgetEmailRequest>,
) -> Result<Json<EmptySuccess>, WebError> {
    if !cds_db::get_config(&s.db.conn).await.email.enabled {
        return Err(WebError::BadRequest(json!("email_disabled")));
    }

    let email: Email = cds_db::email::find_by_email(&s.db.conn, body.email.to_owned())
        .await?
        .ok_or(WebError::BadRequest("email_not_found".into()))?;

    let user: User = cds_db::user::find_by_email(&s.db.conn, email.email.to_owned())
        .await?
        .ok_or(WebError::BadRequest("user_not_found".into()))?;

    if s.cache
        .get::<i64>(format!("mailbox:{}:buffer", email.email.to_owned()))
        .await?
        .is_some()
    {
        return Err(WebError::BadRequest(json!("email_send_too_frequently")));
    }

    let code = nanoid!();
    s.cache
        .set_ex(
            format!("mailbox:{}:code", email.email.to_owned()),
            code.to_owned(),
            60 * 60,
        )
        .await?;

    let body = s
        .media
        .config()
        .email()
        .get_email(EmailType::Forget)
        .await?;

    s.queue
        .publish(
            "mailbox",
            cds_mailbox::Payload {
                name: user.name.to_owned(),
                email: email.email.to_owned(),
                subject: util::email::extract_title(&body)
                    .unwrap_or("Reset Your Password".to_owned()),
                body: body.replace("%CODE%", &code).replace("%USER%", &user.name),
            },
        )
        .await?;

    s.cache
        .set_ex(format!("mailbox:{}:buffer", email.email.to_owned()), 1, 60)
        .await?;

    Ok(Json(EmptySuccess::default()))
}
