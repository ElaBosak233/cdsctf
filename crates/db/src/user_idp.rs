//! Database access for external identity bindings owned by local users.

use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, FromQueryResult, QueryFilter, Set,
};
use serde::{Deserialize, Serialize};
use tracing::info;

pub use crate::entity::user_idp::{ActiveModel as UserIdpActiveModel, Model as UserIdpModel};
pub(crate) use crate::entity::user_idp::{Column as UserIdpColumn, Entity as UserIdpEntity};
use crate::traits::DbError;

#[derive(
    Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromQueryResult, utoipa::ToSchema,
)]
pub struct UserIdp {
    pub id: i64,
    pub user_id: i64,
    pub idp_id: i64,
    pub auth_key: String,
    pub data: Option<serde_json::Value>,
    pub created_at: i64,
    pub updated_at: i64,
}

pub async fn find_user_idp_by_auth_key<T>(
    conn: &impl ConnectionTrait,
    idp_id: i64,
    auth_key: &str,
) -> Result<Option<T>, DbError>
where
    T: FromQueryResult, {
    Ok(UserIdpEntity::find()
        .filter(UserIdpColumn::IdpId.eq(idp_id))
        .filter(UserIdpColumn::AuthKey.eq(auth_key))
        .into_model::<T>()
        .one(conn)
        .await?)
}

pub async fn find_user_idp_by_user_and_idp<T>(
    conn: &impl ConnectionTrait,
    user_id: i64,
    idp_id: i64,
) -> Result<Option<T>, DbError>
where
    T: FromQueryResult, {
    Ok(UserIdpEntity::find()
        .filter(UserIdpColumn::UserId.eq(user_id))
        .filter(UserIdpColumn::IdpId.eq(idp_id))
        .into_model::<T>()
        .one(conn)
        .await?)
}

pub async fn find_user_idps_by_user<T>(
    conn: &impl ConnectionTrait,
    user_id: i64,
) -> Result<Vec<T>, DbError>
where
    T: FromQueryResult, {
    Ok(UserIdpEntity::find()
        .filter(UserIdpColumn::UserId.eq(user_id))
        .into_model::<T>()
        .all(conn)
        .await?)
}

pub async fn create_user_idp<T>(
    conn: &impl ConnectionTrait,
    model: UserIdpActiveModel,
) -> Result<T, DbError>
where
    T: FromQueryResult, {
    let user_idp = model.insert(conn).await?;
    info!(
        idp_id = user_idp.idp_id,
        user_id = user_idp.user_id,
        "user idp bound"
    );
    find_user_idp_by_auth_key(conn, user_idp.idp_id, &user_idp.auth_key)
        .await?
        .ok_or_else(|| DbError::NotFound(format!("user_idp_{}", user_idp.id)))
}

pub async fn update_user_idp_data(
    conn: &impl ConnectionTrait,
    user_idp: &UserIdpModel,
    data: Option<serde_json::Value>,
) -> Result<(), DbError> {
    let _ = UserIdpActiveModel {
        id: Set(user_idp.id),
        data: Set(data),
        ..Default::default()
    }
    .update(conn)
    .await?;
    Ok(())
}

pub async fn delete_user_idp(
    conn: &impl ConnectionTrait,
    user_id: i64,
    id: i64,
) -> Result<(), DbError> {
    let _ = UserIdpEntity::delete_many()
        .filter(UserIdpColumn::Id.eq(id))
        .filter(UserIdpColumn::UserId.eq(user_id))
        .exec(conn)
        .await?;
    info!(user_idp_id = id, user_id, "user idp unbound");
    Ok(())
}
