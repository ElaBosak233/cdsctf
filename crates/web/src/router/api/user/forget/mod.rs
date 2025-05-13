use argon2::{
    Argon2, PasswordHasher,
    password_hash::{SaltString, rand_core::OsRng},
};
use axum::Router;
use cds_db::{
    get_db,
    sea_orm::{
        ActiveModelTrait,
        ActiveValue::{Set, Unchanged},
        ColumnTrait, EntityTrait, QueryFilter,
    },
};
use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    extract::Json,
    traits::{WebError, WebResponse},
};

pub fn router() -> Router {
    Router::new()
        .route("/", axum::routing::post(user_forget))
        .route("/send", axum::routing::post(send_forget_email))
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserForgetRequest {
    pub email: String,
    pub code: String,
    pub password: String,
}

pub async fn user_forget(Json(body): Json<UserForgetRequest>) -> Result<WebResponse<()>, WebError> {
    let user = cds_db::entity::user::Entity::find()
        .filter(cds_db::entity::user::Column::Email.eq(body.email.to_owned().to_lowercase()))
        .one(get_db())
        .await?
        .ok_or(WebError::BadRequest("user_not_found".into()))?;

    let code = cds_cache::get::<String>(format!("email:{}:code", user.email))
        .await?
        .ok_or(WebError::BadRequest("email_code_expired".into()))?;

    if code != body.code {
        return Err(WebError::BadRequest(json!("email_code_incorrect")));
    }

    let hashed_password = Argon2::default()
        .hash_password(body.password.as_bytes(), &SaltString::generate(&mut OsRng))
        .unwrap()
        .to_string();

    let _ = cds_db::entity::user::ActiveModel {
        id: Unchanged(user.id),
        hashed_password: Set(hashed_password),
        ..Default::default()
    }
    .update(get_db())
    .await?;

    let _ = cds_cache::get_del::<String>(format!("email:{}:code", user.email)).await?;

    Ok(WebResponse {
        ..Default::default()
    })
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserSendForgetEmailRequest {
    pub email: String,
}

pub async fn send_forget_email(
    Json(body): Json<UserSendForgetEmailRequest>,
) -> Result<WebResponse<()>, WebError> {
    if !cds_db::get_config().await.email.is_enabled {
        return Err(WebError::BadRequest(json!("email_disabled")));
    }

    let user = cds_db::entity::user::Entity::find()
        .filter(cds_db::entity::user::Column::Email.eq(body.email.to_owned().to_lowercase()))
        .one(get_db())
        .await?
        .ok_or(WebError::BadRequest("user_not_found".into()))?;

    if cds_cache::get::<i64>(format!("email:{}:buffer", user.email.to_owned()))
        .await?
        .is_some()
    {
        return Err(WebError::BadRequest(json!("email_send_too_frequently")));
    }

    let code = nanoid!();
    cds_cache::set_ex(
        format!("email:{}:code", user.email.to_owned()),
        code.to_owned(),
        60 * 60,
    )
    .await?;

    cds_queue::publish(
        "email",
        crate::worker::email_sender::Payload {
            name: user.name.to_owned(),
            email: user.email.to_owned(),
            subject: "Reset Password".to_owned(),
            body: code,
        },
    )
    .await?;

    cds_cache::set_ex(format!("email:{}:buffer", user.email.to_owned()), 1, 60).await?;

    Ok(WebResponse {
        ..Default::default()
    })
}
