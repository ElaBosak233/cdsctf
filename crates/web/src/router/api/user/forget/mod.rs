//! HTTP routing for `forget` — Axum router wiring and OpenAPI route
//! registration.

use std::sync::Arc;

use axum::{Json, Router, extract::State};
use cds_mailbox::{CODE_TTL, EmailPurpose, Payload};
use cds_worker::mailbox::SUBJECT;
use garde::Validate;
use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use serde_json::json;
use utoipa_axum::{
    router::{OpenApiRouter, UtoipaMethodRouterExt},
    routes,
};

use crate::{
    extract::{Json as ReqJson, Query},
    traits::{AppState, EmptyJson, WebError},
    util,
};

/// Builds the Axum router fragment for this module.

pub fn router(state: Arc<AppState>) -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::from(Router::new().with_state(state.clone()))
        .routes(routes!(user_forget).with_state(state.clone()))
        .routes(routes!(send_forget_email).with_state(state.clone()))
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate, utoipa::ToSchema)]
#[garde(allow_unvalidated)]
pub struct UserForgetRequest {
    #[garde(email)]
    pub email: String,
    pub code: String,
    pub password: String,
}

/// Starts the password-reset flow.
#[utoipa::path(
    post,
    path = "/",
    tag = "user",
    request_body = UserForgetRequest,
    responses(
        (status = 200, description = "Password reset", body = EmptyJson),
        (status = 409, description = "Address no longer verified for the account", body = crate::traits::ErrorResponse),
        (status = 400, description = "Bad request", body = crate::traits::ErrorResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
#[tracing::instrument(skip_all, fields(handler = "user_forget"))]
pub async fn user_forget(
    State(s): State<Arc<AppState>>,
    ReqJson(body): ReqJson<UserForgetRequest>,
) -> Result<Json<EmptyJson>, WebError> {
    let email = body.email.to_lowercase();
    let user = cds_db::user::find_by_verified_email(&s.db.conn, &email)
        .await?
        .ok_or_else(|| WebError::BadRequest(json!("email_not_verified")))?;

    util::email::consume_verification_code(&s, &reset_code_key(user.id, &email), &body.code)
        .await?;
    let hashed_password = util::crypto::hash_password(body.password);
    cds_db::user::reset_password_by_verified_email(&s.db.conn, user.id, &email, hashed_password)
        .await?;

    Ok(Json(EmptyJson::default()))
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate, utoipa::ToSchema)]
#[garde(allow_unvalidated)]
pub struct UserSendForgetEmailRequest {
    #[garde(email)]
    pub email: String,
}

/// Enqueues email with a password-reset token.
#[utoipa::path(
    post,
    path = "/send",
    tag = "user",
    request_body = UserSendForgetEmailRequest,
    params(util::email::EmailLocaleQuery),
    responses(
        (status = 200, description = "Email queued", body = EmptyJson),
        (status = 400, description = "Bad request", body = crate::traits::ErrorResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
#[tracing::instrument(skip_all, fields(handler = "send_forget_email"))]
pub async fn send_forget_email(
    State(s): State<Arc<AppState>>,
    Query(locale): Query<util::email::EmailLocaleQuery>,
    ReqJson(body): ReqJson<UserSendForgetEmailRequest>,
) -> Result<Json<EmptyJson>, WebError> {
    let config = cds_db::get_config(&s.db.conn).await;
    if !config.email.enabled {
        return Err(WebError::BadRequest(json!("email_disabled")));
    }

    let email = body.email.to_lowercase();
    let user = cds_db::user::find_by_verified_email(&s.db.conn, &email)
        .await?
        .ok_or_else(|| WebError::BadRequest(json!("email_not_verified")))?;
    let cooldown_key = format!("mailbox:reset:{}:{email}:buffer", user.id);

    if s.cache.get::<i64>(&cooldown_key).await?.is_some() {
        return Err(WebError::BadRequest(json!("email_send_too_frequently")));
    }

    let code = nanoid!();
    let payload = Payload::transactional(
        user.name,
        email.clone(),
        EmailPurpose::Reset,
        locale.locale.as_deref(),
        &code,
        &config.meta.title,
    )?;
    s.cache
        .set_with_ttl(reset_code_key(user.id, &email), code.to_owned(), CODE_TTL)
        .await?;

    s.queue.publish(SUBJECT, payload, None).await?;

    s.cache
        .set_with_ttl(cooldown_key, 1, std::time::Duration::from_secs(60))
        .await?;

    Ok(Json(EmptyJson::default()))
}

fn reset_code_key(user_id: i64, email: &str) -> String {
    format!("mailbox:reset:{user_id}:{email}:code")
}
