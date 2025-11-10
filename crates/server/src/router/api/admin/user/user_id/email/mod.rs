use axum::{Router, http::StatusCode};
use cds_db::{
    Email,
    sea_orm::ActiveValue::{Set, Unchanged},
};
use cds_media::config::email::EmailType;
use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use serde_json::json;
use validator::Validate;

use crate::{
    extract::{Path, VJson},
    traits::{WebError, WebResponse},
    util,
};

pub fn router() -> Router {
    Router::new()
        .route("/", axum::routing::get(list_emails))
        .route("/", axum::routing::post(add_email))
        .route("/{email}", axum::routing::put(update_email))
        .route("/{email}", axum::routing::delete(delete_email))
        .route(
            "/{email}/verify/send",
            axum::routing::post(send_verify_email),
        )
}

pub async fn list_emails(Path(user_id): Path<i64>) -> Result<WebResponse<Vec<Email>>, WebError> {
    crate::util::loader::prepare_user(user_id).await?;
    let emails = cds_db::email::find_by_user_id::<Email>(user_id).await?;

    Ok(WebResponse {
        code: StatusCode::OK,
        data: Some(emails),
        ..Default::default()
    })
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
pub struct AdminAddEmailRequest {
    #[validate(email)]
    pub email: String,
    pub is_verified: Option<bool>,
}

pub async fn add_email(
    Path(user_id): Path<i64>,
    VJson(body): VJson<AdminAddEmailRequest>,
) -> Result<WebResponse<Email>, WebError> {
    let user = crate::util::loader::prepare_user(user_id).await?;
    let email = body.email.to_lowercase();

    if cds_db::email::find_by_email::<Email>(email.to_owned())
        .await?
        .is_some()
    {
        return Err(WebError::Conflict(json!("email_already_exists")));
    }

    let config = cds_db::get_config().await;
    let is_verified = body.is_verified.unwrap_or(!config.email.is_enabled);

    let email = cds_db::email::create::<Email>(cds_db::email::ActiveModel {
        user_id: Set(user.id),
        email: Set(email),
        is_verified: Set(is_verified),
    })
    .await?;

    Ok(WebResponse {
        code: StatusCode::OK,
        data: Some(email),
        ..Default::default()
    })
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
pub struct AdminUpdateEmailRequest {
    pub is_verified: Option<bool>,
}

pub async fn update_email(
    Path((user_id, email)): Path<(i64, String)>,
    VJson(body): VJson<AdminUpdateEmailRequest>,
) -> Result<WebResponse<Email>, WebError> {
    let email = cds_db::email::find_by_email::<Email>(email.to_lowercase())
        .await?
        .ok_or(WebError::NotFound(json!("email_not_found")))?;

    if email.user_id != user_id {
        return Err(WebError::Forbidden(json!("email_not_found")));
    }

    let is_verified = body
        .is_verified
        .ok_or(WebError::BadRequest(json!("missing_fields")))?;

    let email = cds_db::email::update::<Email>(cds_db::email::ActiveModel {
        email: Unchanged(email.email.to_owned()),
        user_id: Unchanged(email.user_id),
        is_verified: Set(is_verified),
        ..Default::default()
    })
    .await?;

    Ok(WebResponse {
        code: StatusCode::OK,
        data: Some(email),
        ..Default::default()
    })
}

pub async fn delete_email(
    Path((user_id, email)): Path<(i64, String)>,
) -> Result<WebResponse<()>, WebError> {
    crate::util::loader::prepare_user(user_id).await?;
    cds_db::email::delete(user_id, email.to_lowercase()).await?;

    Ok(WebResponse {
        code: StatusCode::OK,
        ..Default::default()
    })
}

pub async fn send_verify_email(
    Path((user_id, email)): Path<(i64, String)>,
) -> Result<WebResponse<()>, WebError> {
    let user = crate::util::loader::prepare_user(user_id).await?;
    let config = cds_db::get_config().await;

    if !config.email.is_enabled {
        return Err(WebError::BadRequest(json!("email_disabled")));
    }

    let email = cds_db::email::find_by_email::<Email>(email.to_lowercase())
        .await?
        .ok_or(WebError::BadRequest(json!("email_not_found")))?;

    if email.user_id != user_id {
        return Err(WebError::Forbidden(json!("email_not_found")));
    }

    if email.is_verified {
        return Err(WebError::BadRequest(json!("email_already_verified")));
    }

    if cds_cache::get::<i64>(format!("email:{}:buffer", email.email.to_owned()))
        .await?
        .is_some()
    {
        return Err(WebError::BadRequest(json!("email_send_too_frequently")));
    }

    let code = nanoid!();
    cds_cache::set_ex(
        format!("email:{}:code", email.email.to_owned()),
        code.to_owned(),
        60 * 60,
    )
    .await?;

    let body = cds_media::config::email::get_email(EmailType::Verify).await?;

    cds_queue::publish(
        "email",
        cds_email::Payload {
            name: user.name.to_owned(),
            email: email.email.to_owned(),
            subject: util::email::extract_title(&body).unwrap_or("Verify Your Email".to_owned()),
            body: body.replace("%CODE%", &code).replace("%USER%", &user.name),
        },
    )
    .await?;

    cds_cache::set_ex(format!("email:{}:buffer", email.email.to_owned()), 1, 60).await?;

    Ok(WebResponse {
        code: StatusCode::OK,
        ..Default::default()
    })
}
