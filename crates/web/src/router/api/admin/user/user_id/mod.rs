use argon2::{
    Argon2, PasswordHasher,
    password_hash::{SaltString, rand_core::OsRng},
};
use axum::{Router, http::StatusCode};
use cds_db::{
    entity::user::Group,
    get_db,
    sea_orm::{
        ActiveModelTrait,
        ActiveValue::{Set, Unchanged},
        EntityTrait, NotSet,
    },
    transfer::User,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use validator::Validate;

use crate::{
    extract::{Path, VJson},
    traits::{WebError, WebResponse},
};

mod avatar;

pub fn router() -> Router {
    Router::new()
        .route("/", axum::routing::put(update_user))
        .route("/", axum::routing::delete(delete_user))
        .nest("/avatar", avatar::router())
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
pub struct UpdateUserRequest {
    pub nickname: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub group: Option<Group>,
    pub description: Option<String>,
}

/// Update a user with given data.
///
/// # Prerequisite
/// - Operator is admin.
pub async fn update_user(
    Path(user_id): Path<i64>, VJson(mut body): VJson<UpdateUserRequest>,
) -> Result<WebResponse<User>, WebError> {
    let user = crate::util::loader::prepare_user(user_id).await?;

    if let Some(email) = body.email {
        body.email = Some(email.to_lowercase());
        if !cds_db::util::is_user_email_unique(user.id, &email.to_lowercase()).await? {
            return Err(WebError::Conflict(json!("email_already_exists")));
        }
    }

    if let Some(password) = body.password {
        let hashed_password = Argon2::default()
            .hash_password(password.as_bytes(), &SaltString::generate(&mut OsRng))
            .unwrap()
            .to_string();
        body.password = Some(hashed_password);
    }

    let user = cds_db::entity::user::ActiveModel {
        id: Unchanged(user.id),
        nickname: body.nickname.map_or(NotSet, Set),
        email: body.email.map_or(NotSet, Set),
        hashed_password: body.password.map_or(NotSet, Set),
        group: body.group.map_or(NotSet, Set),
        description: body.description.map_or(NotSet, |v| Set(Some(v))),
        ..Default::default()
    }
    .update(get_db())
    .await?;
    let user = cds_db::transfer::User::from(user);

    Ok(WebResponse {
        code: StatusCode::OK,
        data: Some(user),
        ..Default::default()
    })
}

/// Delete a user with given data.
///
/// # Prerequisite
/// - Operator is admin.
pub async fn delete_user(Path(user_id): Path<i64>) -> Result<WebResponse<()>, WebError> {
    let user = crate::util::loader::prepare_user(user_id).await?;

    let _ = cds_db::entity::user::ActiveModel {
        id: Unchanged(user.id),
        username: Set(format!("[DELETED]_{}", user.username)),
        email: Set(format!("deleted_{}@del.cdsctf", user.email)),
        deleted_at: Set(Some(chrono::Utc::now().timestamp())),
        ..Default::default()
    }
    .update(get_db())
    .await?;

    Ok(WebResponse {
        code: StatusCode::OK,
        ..Default::default()
    })
}
