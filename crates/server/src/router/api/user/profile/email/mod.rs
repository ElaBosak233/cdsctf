use std::sync::Arc;

use axum::{Router, extract::State};
use cds_db::{
    Email,
    sea_orm::{Set, Unchanged},
};
use cds_media::config::email::EmailType;
use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use serde_json::json;
use validator::Validate;

use crate::{
    extract::{Extension, Json, Path},
    traits::{AppState, AuthPrincipal, WebError, WebResponse},
    util,
};

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", axum::routing::get(get_email))
        .route("/", axum::routing::post(add_email))
        .route("/{mailbox}", axum::routing::delete(delete_email))
        .route("/{mailbox}/verify", axum::routing::post(verify_email))
        .route(
            "/{mailbox}/verify/send",
            axum::routing::post(send_verify_email),
        )
}

pub async fn get_email(
    State(s): State<Arc<AppState>>,

    Extension(ext): Extension<AuthPrincipal>,
) -> Result<WebResponse<Vec<Email>>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized("".into()))?;
    let emails = cds_db::email::find_by_user_id(&s.db.conn, operator.id).await?;

    Ok(WebResponse {
        data: Some(emails),
        ..Default::default()
    })
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
pub struct UserAddEmailRequest {
    #[validate(email)]
    pub email: String,
}

pub async fn add_email(
    State(s): State<Arc<AppState>>,

    Extension(ext): Extension<AuthPrincipal>,
    Json(body): Json<UserAddEmailRequest>,
) -> Result<WebResponse<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized("".into()))?;

    let _ = cds_db::email::create::<Email>(
        &s.db.conn,
        cds_db::email::ActiveModel {
            user_id: Set(operator.id),
            email: Set(body.email.to_lowercase()),
            is_verified: Set(!cds_db::get_config(&s.db.conn).await.email.is_enabled),
        },
    )
    .await?;

    Ok(WebResponse::default())
}

pub async fn delete_email(
    State(s): State<Arc<AppState>>,

    Extension(ext): Extension<AuthPrincipal>,
    Path(email): Path<String>,
) -> Result<WebResponse<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized("".into()))?;
    let email = email.to_lowercase();

    let _ = cds_db::email::delete(&s.db.conn, operator.id, email).await?;

    Ok(WebResponse::default())
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EmailVerifyRequest {
    pub code: String,
}

pub async fn verify_email(
    State(s): State<Arc<AppState>>,

    Extension(ext): Extension<AuthPrincipal>,
    Path(email): Path<String>,
    Json(body): Json<EmailVerifyRequest>,
) -> Result<WebResponse<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized("".into()))?;

    let email = cds_db::email::find_by_email::<Email>(&s.db.conn, email.to_lowercase())
        .await?
        .ok_or(WebError::BadRequest(json!("email_not_found")))?;

    if email.user_id != operator.id {
        return Err(WebError::Forbidden(json!("email_not_found")));
    }

    if email.is_verified {
        return Err(WebError::BadRequest(json!("email_already_verified")));
    }

    if cds_db::get_config(&s.db.conn).await.email.is_enabled {
        let code = s
            .cache
            .get::<String>(format!("mailbox:{}:code", email.email.to_owned()))
            .await?
            .ok_or(WebError::BadRequest("email_code_expired".into()))?;

        if code != body.code {
            return Err(WebError::BadRequest(json!("email_code_incorrect")));
        }

        let _ = s
            .cache
            .get_del::<String>(format!("mailbox:{}:code", email.email.to_owned()))
            .await?;
    }

    let _ = cds_db::email::update::<Email>(
        &s.db.conn,
        cds_db::email::ActiveModel {
            email: Unchanged(email.email.to_owned()),
            user_id: Unchanged(email.user_id),
            is_verified: Set(true),
            ..Default::default()
        },
    )
    .await?;

    Ok(WebResponse::default())
}

pub async fn send_verify_email(
    State(s): State<Arc<AppState>>,

    Extension(ext): Extension<AuthPrincipal>,
    Path(email): Path<String>,
) -> Result<WebResponse<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized("".into()))?;
    if !cds_db::get_config(&s.db.conn).await.email.is_enabled {
        return Err(WebError::BadRequest(json!("email_disabled")));
    }

    let email: Email = cds_db::email::find_by_email(&s.db.conn, email.to_lowercase())
        .await?
        .ok_or(WebError::BadRequest(json!("email_not_found")))?;

    if email.user_id != operator.id {
        return Err(WebError::Forbidden(json!("email_not_found")));
    }

    if email.is_verified {
        return Err(WebError::BadRequest(json!("email_already_verified")));
    }

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
        .get_email(EmailType::Verify)
        .await?;

    s.queue
        .publish(
            "mailbox",
            cds_mailbox::Payload {
                name: operator.name.to_owned(),
                email: email.email.to_owned(),
                subject: util::email::extract_title(&body)
                    .unwrap_or("Verify Your Email".to_owned()),
                body: body
                    .replace("%CODE%", &code)
                    .replace("%USER%", &operator.name),
            },
        )
        .await?;

    s.cache
        .set_ex(format!("mailbox:{}:buffer", email.email.to_owned()), 1, 60)
        .await?;

    Ok(WebResponse {
        ..Default::default()
    })
}
