use axum::Router;
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
    traits::{AuthPrincipal, WebError, WebResponse},
    util,
};

pub fn router() -> Router {
    Router::new()
        .route("/", axum::routing::get(get_email))
        .route("/", axum::routing::post(add_email))
        .route("/{email}", axum::routing::delete(delete_email))
        .route("/{email}/verify", axum::routing::post(verify_email))
        .route(
            "/{email}/verify/send",
            axum::routing::post(send_verify_email),
        )
}

pub async fn get_email(
    Extension(ext): Extension<AuthPrincipal>,
) -> Result<WebResponse<Vec<Email>>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized("".into()))?;
    let emails = cds_db::email::find_by_user_id::<Email>(operator.id).await?;

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
    Extension(ext): Extension<AuthPrincipal>,
    Json(body): Json<UserAddEmailRequest>,
) -> Result<WebResponse<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized("".into()))?;

    let _ = cds_db::email::create::<Email>(cds_db::email::ActiveModel {
        user_id: Set(operator.id),
        email: Set(body.email.to_lowercase()),
        is_verified: Set(!cds_db::get_config().await.email.is_enabled),
    })
    .await?;

    Ok(WebResponse::default())
}

pub async fn delete_email(
    Extension(ext): Extension<AuthPrincipal>,
    Path(email): Path<String>,
) -> Result<WebResponse<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized("".into()))?;
    let email = email.to_lowercase();

    let _ = cds_db::email::delete(operator.id, email).await?;

    Ok(WebResponse::default())
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EmailVerifyRequest {
    pub code: String,
}

pub async fn verify_email(
    Extension(ext): Extension<AuthPrincipal>,
    Path(email): Path<String>,
    Json(body): Json<EmailVerifyRequest>,
) -> Result<WebResponse<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized("".into()))?;

    let email = cds_db::email::find_by_email::<Email>(email.to_lowercase())
        .await?
        .ok_or(WebError::BadRequest(json!("email_not_found")))?;

    if email.user_id != operator.id {
        return Err(WebError::Forbidden(json!("email_not_found")));
    }

    if email.is_verified {
        return Err(WebError::BadRequest(json!("email_already_verified")));
    }

    if cds_db::get_config().await.email.is_enabled {
        let code = cds_cache::get::<String>(format!("email:{}:code", email.email.to_owned()))
            .await?
            .ok_or(WebError::BadRequest("email_code_expired".into()))?;

        if code != body.code {
            return Err(WebError::BadRequest(json!("email_code_incorrect")));
        }

        let _ =
            cds_cache::get_del::<String>(format!("email:{}:code", email.email.to_owned())).await?;
    }

    let _ = cds_db::email::update::<Email>(cds_db::email::ActiveModel {
        email: Unchanged(email.email.to_owned()),
        user_id: Unchanged(email.user_id),
        is_verified: Set(true),
        ..Default::default()
    })
    .await?;

    Ok(WebResponse::default())
}

pub async fn send_verify_email(
    Extension(ext): Extension<AuthPrincipal>,
    Path(email): Path<String>,
) -> Result<WebResponse<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized("".into()))?;
    if !cds_db::get_config().await.email.is_enabled {
        return Err(WebError::BadRequest(json!("email_disabled")));
    }

    let email = cds_db::email::find_by_email::<Email>(email.to_lowercase())
        .await?
        .ok_or(WebError::BadRequest(json!("email_not_found")))?;

    if email.user_id != operator.id {
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
            name: operator.name.to_owned(),
            email: email.email.to_owned(),
            subject: util::email::extract_title(&body).unwrap_or("Verify Your Email".to_owned()),
            body: body
                .replace("%CODE%", &code)
                .replace("%USER%", &operator.name),
        },
    )
    .await?;

    cds_cache::set_ex(format!("email:{}:buffer", email.email.to_owned()), 1, 60).await?;

    Ok(WebResponse {
        ..Default::default()
    })
}
