use axum::Router;
use cds_db::{
    get_db,
    sea_orm::{
        ActiveModelTrait,
        ActiveValue::{Set, Unchanged},
    },
};
use cds_media::config::email::EmailType;
use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    extract::{Extension, Json},
    traits::{AuthPrincipal, WebError, WebResponse},
    util,
};

pub fn router() -> Router {
    Router::new()
        .route("/", axum::routing::post(user_verify))
        .route("/send", axum::routing::post(send_verify_email))
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserVerifyRequest {
    pub code: String,
}

pub async fn user_verify(
    Extension(ext): Extension<AuthPrincipal>,
    Json(body): Json<UserVerifyRequest>,
) -> Result<WebResponse<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized("".into()))?;

    if operator.is_verified {
        return Err(WebError::BadRequest(json!("email_already_verified")));
    }

    let code = cds_cache::get::<String>(format!("email:{}:code", operator.email))
        .await?
        .ok_or(WebError::BadRequest("email_code_expired".into()))?;

    if code != body.code {
        return Err(WebError::BadRequest(json!("email_code_incorrect")));
    }

    let _ = cds_db::entity::user::ActiveModel {
        id: Unchanged(operator.id),
        is_verified: Set(true),
        ..Default::default()
    }
    .update(get_db())
    .await?;

    let _ = cds_cache::get_del::<String>(format!("email:{}:code", operator.email)).await?;

    Ok(WebResponse {
        ..Default::default()
    })
}

pub async fn send_verify_email(
    Extension(ext): Extension<AuthPrincipal>,
) -> Result<WebResponse<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized("".into()))?;
    if !cds_db::get_config().await.email.is_enabled {
        return Err(WebError::BadRequest(json!("email_disabled")));
    }

    if operator.is_verified {
        return Err(WebError::BadRequest(json!("email_already_verified")));
    }

    if cds_cache::get::<i64>(format!("email:{}:buffer", operator.email.to_owned()))
        .await?
        .is_some()
    {
        return Err(WebError::BadRequest(json!("email_send_too_frequently")));
    }

    let code = nanoid!();
    cds_cache::set_ex(
        format!("email:{}:code", operator.email.to_owned()),
        code.to_owned(),
        60 * 60,
    )
    .await?;

    let body = cds_media::config::email::get_email(EmailType::Verify).await?;

    cds_queue::publish(
        "email",
        crate::worker::email_sender::Payload {
            name: operator.name.to_owned(),
            email: operator.email.to_owned(),
            subject: util::email::extract_title(&body).unwrap_or("Verify Your Email".to_owned()),
            body: body
                .replace("%CODE%", &code)
                .replace("%USER%", &operator.name),
        },
    )
    .await?;

    cds_cache::set_ex(format!("email:{}:buffer", operator.email.to_owned()), 1, 60).await?;

    Ok(WebResponse {
        ..Default::default()
    })
}
