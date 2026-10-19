//! Database access for configured identity providers.

use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, FromQueryResult, QueryFilter,
};

pub(crate) use crate::entity::idp::{Column as IdpColumn, Entity as IdpEntity};
use crate::traits::DbError;
pub use crate::{
    dto::idp::{IdpSummary, IdpView},
    entity::idp::{ActiveModel as IdpActiveModel, Model as IdpModel},
};

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
    tracing::info!(
        resource = "idp",
        action = "create",
        idp_id = idp.id,
        "resource step completed"
    );
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
    let changed_fields = [
        ("name", model.name.is_set()),
        ("enabled", model.enabled.is_set()),
        ("registration_enabled", model.registration_enabled.is_set()),
        ("avatar_hash", model.avatar_hash.is_set()),
        ("portal", model.portal.is_set()),
        ("script", model.script.is_set()),
    ]
    .into_iter()
    .filter_map(|(name, changed)| changed.then_some(name))
    .collect::<Vec<_>>();
    let idp = model.update(conn).await?;
    tracing::info!(
        resource = "idp",
        action = "update",
        changed_fields = ?changed_fields,
        idp_id = idp.id,
        "database step completed",
    );
    find_idp_by_id(conn, idp.id)
        .await?
        .ok_or_else(|| DbError::NotFound(format!("idp_{}", idp.id)))
}

pub async fn delete_idp(conn: &impl ConnectionTrait, id: i64) -> Result<(), DbError> {
    let _ = IdpEntity::delete_by_id(id).exec(conn).await?;
    tracing::info!(
        resource = "idp",
        action = "delete",
        idp_id = id,
        "resource step completed"
    );
    Ok(())
}
