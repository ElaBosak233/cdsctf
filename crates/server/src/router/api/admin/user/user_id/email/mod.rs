use axum::{Router, http::StatusCode};
use cds_db::{
    Email,
    sea_orm::ActiveValue::{Set, Unchanged},
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use validator::Validate;

use crate::{
    extract::{Path, VJson},
    traits::{WebError, WebResponse},
};

pub fn router() -> Router {
    Router::new()
        .route("/", axum::routing::get(get_email))
        .route("/", axum::routing::post(add_email))
        .route("/{email}", axum::routing::put(update_email))
        .route("/{email}", axum::routing::delete(delete_email))
}

pub async fn get_email(Path(user_id): Path<i64>) -> Result<WebResponse<Vec<Email>>, WebError> {
    crate::util::loader::prepare_user(user_id).await?;
    let emails = cds_db::email::find_by_user_id::<Email>(user_id).await?;

    Ok(WebResponse {
        code: StatusCode::OK,
        data: Some(emails),
        ..Default::default()
    })
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
pub struct AddEmailRequest {
    #[validate(email)]
    pub email: String,
    pub is_verified: Option<bool>,
}

pub async fn add_email(
    Path(user_id): Path<i64>,
    VJson(body): VJson<AddEmailRequest>,
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
pub struct UpdateEmailRequest {
    pub is_verified: Option<bool>,
}

pub async fn update_email(
    Path((user_id, email)): Path<(i64, String)>,
    VJson(body): VJson<UpdateEmailRequest>,
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
