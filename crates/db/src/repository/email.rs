//! Email ownership and verification changes, with account-level serialization.

use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DatabaseTransaction, EntityTrait,
    FromQueryResult, PaginatorTrait, QueryFilter, Set, SqlErr, TransactionTrait, Unchanged,
};

use super::user;
pub(crate) use crate::entity::email::{Column, Entity};
use crate::{Permission, entity::email::ActiveModel, traits::DbError};
pub use crate::{dto::email::EmailView, entity::email::Model};

/// Lists addresses linked to an account.
pub async fn find_by_user_id<T>(
    conn: &impl ConnectionTrait,
    user_id: i64,
) -> Result<Vec<T>, DbError>
where
    T: FromQueryResult, {
    Ok(Entity::find()
        .filter(Column::UserId.eq(user_id))
        .into_model::<T>()
        .all(conn)
        .await?)
}

/// Looks up an address.
pub async fn find_by_email<T>(
    conn: &impl ConnectionTrait,
    email: String,
) -> Result<Option<T>, DbError>
where
    T: FromQueryResult, {
    Ok(Entity::find_by_id(email)
        .into_model::<T>()
        .one(conn)
        .await?)
}

async fn linked_email(
    conn: &DatabaseTransaction,
    user_id: i64,
    email: &str,
) -> Result<Model, DbError> {
    Entity::find_by_id(email.to_owned())
        .filter(Column::UserId.eq(user_id))
        .one(conn)
        .await?
        .ok_or_else(|| DbError::NotFound("email_not_found".to_owned()))
}

async fn has_other_verified(
    conn: &DatabaseTransaction,
    user_id: i64,
    email: &str,
) -> Result<bool, DbError> {
    Ok(Entity::find()
        .filter(Column::UserId.eq(user_id))
        .filter(Column::Verified.eq(true))
        .filter(Column::Email.ne(email))
        .count(conn)
        .await?
        > 0)
}

async fn grant_basic_after_first_verification(
    conn: &DatabaseTransaction,
    account: user::Model,
    email: &str,
) -> Result<(), DbError> {
    let mut permissions = account.permissions;
    if permissions.contains(&Permission::Basic)
        || has_other_verified(conn, account.id, email).await?
    {
        return Ok(());
    }

    permissions.push(Permission::Basic);
    user::ActiveModel {
        id: Unchanged(account.id),
        permissions: Set(permissions),
        ..Default::default()
    }
    .update(conn)
    .await?;
    tracing::info!(
        user_id = account.id,
        "granted Basic after first email verification"
    );
    Ok(())
}

async fn preserve_verified_email(conn: &DatabaseTransaction, email: &Model) -> Result<(), DbError> {
    if email.verified && !has_other_verified(conn, email.user_id, &email.email).await? {
        return Err(DbError::Conflict("last_verified_email".to_owned()));
    }
    Ok(())
}

/// Links an address. A verified address requires proof or explicit
/// administrative confirmation.
pub async fn create(
    conn: &impl TransactionTrait<Transaction = DatabaseTransaction>,
    user_id: i64,
    email: String,
    verified: bool,
) -> Result<EmailView, DbError> {
    let transaction = conn.begin().await?;
    let account = user::lock_active(&transaction, user_id).await?;
    let email = ActiveModel {
        user_id: Set(user_id),
        email: Set(email),
        verified: Set(verified),
    }
    .insert(&transaction)
    .await
    .map_err(|error| match error.sql_err() {
        Some(SqlErr::UniqueConstraintViolation(_)) => {
            DbError::Conflict("email_already_exists".to_owned())
        }
        _ => error.into(),
    })?;

    if verified {
        grant_basic_after_first_verification(&transaction, account, &email.email).await?;
    }
    transaction.commit().await?;
    tracing::info!(user_id, email = %email.email, verified, "linked email");
    Ok(EmailView {
        email: email.email,
        verified,
    })
}

/// Changes verification status while preserving at least one verified address
/// once verified.
pub async fn set_verified(
    conn: &impl TransactionTrait<Transaction = DatabaseTransaction>,
    user_id: i64,
    email: &str,
    verified: bool,
) -> Result<EmailView, DbError> {
    let transaction = conn.begin().await?;
    let account = user::lock_active(&transaction, user_id).await?;
    let email = linked_email(&transaction, user_id, email).await?;

    if email.verified != verified {
        if verified {
            grant_basic_after_first_verification(&transaction, account, &email.email).await?;
        } else {
            preserve_verified_email(&transaction, &email).await?;
        }
        ActiveModel {
            email: Unchanged(email.email.clone()),
            user_id: Unchanged(user_id),
            verified: Set(verified),
        }
        .update(&transaction)
        .await?;
    }

    transaction.commit().await?;
    tracing::info!(user_id, email = %email.email, verified, "set email verification status");
    Ok(EmailView {
        email: email.email,
        verified,
    })
}

/// Removes an address while preserving the last verified address.
pub async fn delete(
    conn: &impl TransactionTrait<Transaction = DatabaseTransaction>,
    user_id: i64,
    email: String,
) -> Result<(), DbError> {
    let transaction = conn.begin().await?;
    user::lock_active(&transaction, user_id).await?;
    let email = linked_email(&transaction, user_id, &email).await?;
    preserve_verified_email(&transaction, &email).await?;

    Entity::delete_by_id(email.email.clone())
        .filter(Column::UserId.eq(user_id))
        .exec(&transaction)
        .await?;
    transaction.commit().await?;
    tracing::info!(user_id, email = %email.email, "deleted email");
    Ok(())
}

/// Cleans up addresses during account deletion, with the account already
/// locked.
pub(super) async fn delete_by_user_id(
    conn: &DatabaseTransaction,
    user_id: i64,
) -> Result<(), DbError> {
    Entity::delete_many()
        .filter(Column::UserId.eq(user_id))
        .exec(conn)
        .await?;
    Ok(())
}
