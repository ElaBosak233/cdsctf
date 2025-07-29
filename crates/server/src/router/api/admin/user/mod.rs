mod user_id;

use argon2::{
    Argon2, PasswordHasher,
    password_hash::{SaltString, rand_core::OsRng},
};
use axum::{Router, http::StatusCode};
use cds_db::{
    User,
    sea_orm::ActiveValue::Set,
    user::{FindUserOptions, Group},
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use validator::Validate;

use crate::{
    extract::{Query, VJson},
    traits::{WebError, WebResponse},
};

pub fn router() -> Router {
    Router::new()
        .route("/", axum::routing::get(get_users))
        .route("/", axum::routing::post(create_user))
        .nest("/{user_id}", user_id::router())
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetUsersRequest {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub email: Option<String>,
    pub group: Option<Group>,
    pub page: Option<u64>,
    pub size: Option<u64>,
    pub sorts: Option<String>,
}

pub async fn get_users(
    Query(params): Query<GetUsersRequest>,
) -> Result<WebResponse<Vec<User>>, WebError> {
    let page = params.page.unwrap_or(1);
    let size = params.size.unwrap_or(10).min(100);

    let (users, total) = cds_db::user::find::<User>(FindUserOptions {
        id: params.id,
        name: params.name,
        email: params.email,
        group: params.group,
        sorts: params.sorts,
        page: Some(page),
        size: Some(size),
    })
    .await?;

    Ok(WebResponse {
        code: StatusCode::OK,
        data: Some(users),
        total: Some(total),
        ..Default::default()
    })
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
pub struct CreateUserRequest {
    pub name: String,
    #[validate(length(min = 3, max = 20))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    pub password: String,
    pub group: Group,
}

pub async fn create_user(
    VJson(mut body): VJson<CreateUserRequest>,
) -> Result<WebResponse<User>, WebError> {
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

    let user = cds_db::user::create::<User>(cds_db::user::ActiveModel {
        name: Set(body.name),
        username: Set(body.username),
        email: Set(body.email),
        is_verified: Set(true),
        hashed_password: Set(hashed_password),
        group: Set(body.group),
        ..Default::default()
    })
    .await?;

    Ok(WebResponse {
        code: StatusCode::OK,
        data: Some(user),
        ..Default::default()
    })
}
