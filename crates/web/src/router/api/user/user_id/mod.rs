use argon2::{
    Argon2, PasswordHasher,
    password_hash::{SaltString, rand_core::OsRng},
};
use axum::{Router, http::StatusCode};
use cds_db::{entity::user::Group, get_db};
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{Set, Unchanged},
    ColumnTrait, EntityTrait, JoinType, NotSet, QueryFilter, QuerySelect, RelationTrait,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use validator::Validate;

use crate::{
    extract::{Extension, Path, VJson},
    traits::{Ext, WebError, WebResponse},
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
    #[validate(length(min = 3, max = 20))]
    pub username: Option<String>,
    pub nickname: Option<String>,
    #[validate(email)]
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
    Extension(ext): Extension<Ext>, Path(user_id): Path<i64>,
    VJson(mut body): VJson<UpdateUserRequest>,
) -> Result<WebResponse<cds_db::transfer::User>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized("".into()))?;
    if operator.group != Group::Admin {
        return Err(WebError::Forbidden("".into()));
    }

    let user = cds_db::entity::user::Entity::find_by_id(user_id)
        .filter(cds_db::entity::user::Column::DeletedAt.is_null())
        .one(get_db())
        .await?
        .ok_or(WebError::BadRequest("".into()))?;

    if let Some(email) = body.email {
        body.email = Some(email.to_lowercase());
    }

    if let Some(username) = body.username {
        body.username = Some(username.to_lowercase());
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
        username: body.username.map_or(NotSet, Set),
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
pub async fn delete_user(
    Extension(ext): Extension<Ext>, Path(user_id): Path<i64>,
) -> Result<WebResponse<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    if operator.group != Group::Admin {
        return Err(WebError::Forbidden(json!("")));
    }

    let user = cds_db::entity::user::Entity::find_by_id(user_id)
        .filter(cds_db::entity::user::Column::DeletedAt.is_null())
        .one(get_db())
        .await?
        .ok_or(WebError::BadRequest("".into()))?;

    let _ = cds_db::entity::user::ActiveModel {
        id: Unchanged(user_id),
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
