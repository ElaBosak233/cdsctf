//! Database access for configured identity providers.

use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, FromQueryResult, QueryFilter,
};
use serde::{Deserialize, Serialize};
use tracing::info;

pub use crate::entity::idp::{ActiveModel as IdpActiveModel, Model as IdpModel};
pub(crate) use crate::entity::idp::{Column as IdpColumn, Entity as IdpEntity};
use crate::traits::DbError;

#[derive(
    Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromQueryResult, utoipa::ToSchema,
)]
pub struct Idp {
    pub id: i64,
    pub name: String,
    pub enabled: bool,
    pub has_avatar: bool,
    pub portal: Option<String>,
    pub script: String,
    pub created_at: i64,
    pub updated_at: i64,
}

impl Idp {
    pub fn desensitize(mut self) -> Self {
        self.script.clear();
        self
    }
}

pub async fn find_idps<T>(conn: &impl ConnectionTrait) -> Result<Vec<T>, DbError>
where
    T: FromQueryResult, {
    Ok(IdpEntity::find().into_model::<T>().all(conn).await?)
}

pub async fn find_public_idps<T>(conn: &impl ConnectionTrait) -> Result<Vec<T>, DbError>
where
    T: FromQueryResult, {
    Ok(IdpEntity::find()
        .filter(IdpColumn::Enabled.eq(true))
        .into_model::<T>()
        .all(conn)
        .await?)
}

pub async fn find_idp_by_id<T>(conn: &impl ConnectionTrait, id: i64) -> Result<Option<T>, DbError>
where
    T: FromQueryResult, {
    Ok(IdpEntity::find_by_id(id)
        .into_model::<T>()
        .one(conn)
        .await?)
}

pub async fn create_idp<T>(
    conn: &impl ConnectionTrait,
    model: IdpActiveModel,
) -> Result<T, DbError>
where
    T: FromQueryResult, {
    let idp = model.insert(conn).await?;
    info!(idp_id = idp.id, "idp created");
    find_idp_by_id(conn, idp.id)
        .await?
        .ok_or_else(|| DbError::NotFound(format!("idp_{}", idp.id)))
}

pub async fn update_idp<T>(
    conn: &impl ConnectionTrait,
    model: IdpActiveModel,
) -> Result<T, DbError>
where
    T: FromQueryResult, {
    let idp = model.update(conn).await?;
    info!(idp_id = idp.id, "idp updated");
    find_idp_by_id(conn, idp.id)
        .await?
        .ok_or_else(|| DbError::NotFound(format!("idp_{}", idp.id)))
}

pub async fn delete_idp(conn: &impl ConnectionTrait, id: i64) -> Result<(), DbError> {
    let _ = IdpEntity::delete_by_id(id).exec(conn).await?;
    info!(idp_id = id, "idp deleted");
    Ok(())
}
