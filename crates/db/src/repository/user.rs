//! Database access for `user` — SeaORM queries, updates, and DTOs.

use std::{fmt::Debug, str::FromStr};

use sea_orm::{
    ActiveEnum, ActiveModelTrait, ColumnTrait, Condition, ConnectionTrait, DatabaseTransaction,
    DbBackend, EntityName, EntityTrait, ExprTrait, FromQueryResult, Order, PaginatorTrait,
    QueryFilter, QueryOrder, QuerySelect, Set, SqlErr, Statement, TransactionTrait, Unchanged,
    prelude::Expr,
    sea_query::{
        Func, Query,
        extension::postgres::{PgBinOper, PgExpr},
    },
};

pub use super::team_user::find_users as find_by_team_id;
pub(crate) use crate::entity::user::{Column, Entity};
use crate::{EmailView, Permission, traits::DbError};
pub use crate::{
    dto::user::{UserAccountView, UserProfile, UserSummary},
    entity::user::{ActiveModel, Model},
};

#[derive(Clone, Debug)]
pub enum PermissionFilter {
    AnyOf(Vec<Permission>),
    NoneOf(Vec<Permission>),
}

#[derive(Clone, Debug, Default)]
pub struct FindUserOptions {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub banned: Option<bool>,
    pub keyword: Option<String>,
    pub permissions: Option<PermissionFilter>,
    pub page: Option<u64>,
    pub size: Option<u64>,
    pub sorts: Option<String>,
}

/// Queries rows using filter options and returns `(rows, total_count)`.
pub async fn find<T>(
    conn: &impl ConnectionTrait,
    options: FindUserOptions,
) -> Result<(Vec<T>, u64), DbError>
where
    T: FromQueryResult, {
    find_scoped(conn, options, None).await
}

/// Queries users who belong to at least one team in a game.
pub async fn find_by_game_id<T>(
    conn: &impl ConnectionTrait,
    game_id: i64,
    options: FindUserOptions,
) -> Result<(Vec<T>, u64), DbError>
where
    T: FromQueryResult, {
    find_scoped(conn, options, Some(game_id)).await
}

async fn find_scoped<T>(
    conn: &impl ConnectionTrait,
    FindUserOptions {
        id,
        name,
        banned,
        keyword,
        permissions,
        page,
        size,
        sorts,
    }: FindUserOptions,
    game_id: Option<i64>,
) -> Result<(Vec<T>, u64), DbError>
where
    T: FromQueryResult, {
    let mut sql = Entity::find();

    if let Some(id) = id {
        sql = sql.filter(Column::Id.eq(id));
    }

    if let Some(name) = name {
        let pattern = format!("%{}%", name);
        let condition = Condition::any()
            .add(Column::Username.like(&pattern))
            .add(Column::Name.like(&pattern));
        sql = sql.filter(condition);
    }

    if let Some(keyword) = keyword {
        let pattern = super::literal_search_pattern(&keyword);
        sql = sql.filter(
            Condition::any()
                .add(Expr::col(Column::Username).ilike(&pattern))
                .add(Expr::col(Column::Name).ilike(&pattern)),
        );
    }

    if let Some(banned) = banned {
        sql = sql.filter(Column::Banned.eq(banned));
    }

    if let Some(permissions) = permissions {
        let (include, permissions) = match permissions {
            PermissionFilter::AnyOf(permissions) => (true, permissions),
            PermissionFilter::NoneOf(permissions) => (false, permissions),
        };
        let values: Vec<_> = permissions.iter().map(ActiveEnum::to_value).collect();
        let overlaps = Expr::col(Column::Permissions).binary(PgBinOper::Overlap, values);
        sql = sql.filter(if include { overlaps } else { overlaps.not() });
    }

    if let Some(game_id) = game_id {
        sql = sql
            .inner_join(crate::entity::team::Entity)
            .filter(crate::entity::team::Column::GameId.eq(game_id))
            .distinct();
    }

    sql = sql.filter(Column::DeletedAt.is_null());

    let total = sql.clone().count(conn).await?;

    if let Some(sorts) = sorts {
        let sorts = sorts.split(",").collect::<Vec<&str>>();
        for sort in sorts {
            let col = match Column::from_str(sort.replace("-", "").as_str()) {
                Ok(col) => col,
                Err(_) => continue,
            };
            if sort.starts_with("-") {
                sql = sql.order_by(col, Order::Desc);
            } else {
                sql = sql.order_by(col, Order::Asc);
            }
        }
    }

    if let (Some(page), Some(size)) = (page, size) {
        let offset = std::cmp::min(page.saturating_sub(1).saturating_mul(size), i64::MAX as u64);
        sql = sql.order_by_asc(Column::Id).offset(offset).limit(size);
    }

    let users = sql.into_model::<T>().all(conn).await?;

    Ok((users, total))
}

/// Looks up by id.

pub async fn find_by_id<T>(
    conn: &impl ConnectionTrait,
    user_id: i64,
) -> Result<Option<T>, DbError>
where
    T: FromQueryResult, {
    Ok(Entity::find()
        .filter(Column::Id.eq(user_id))
        .filter(Column::DeletedAt.is_null())
        .into_model::<T>()
        .one(conn)
        .await?)
}

fn matching_verified_email(address: &str) -> Expr {
    use crate::entity::email::{Column as EmailColumn, Entity as EmailEntity};

    let emails = EmailEntity.table_name();
    let mut query = Query::select();
    query
        .expr(Expr::val(1))
        .from(emails)
        .and_where(
            Expr::col((emails, EmailColumn::UserId))
                .eq(Expr::col((Entity.table_name(), Column::Id))),
        )
        .and_where(Expr::expr(Func::lower(Expr::col((emails, EmailColumn::Email)))).eq(address))
        .and_where(Expr::col((emails, EmailColumn::Verified)).eq(true));
    Expr::exists(query)
}

/// Looks up a login name or verified email address.
pub async fn find_by_account<T>(
    conn: &impl ConnectionTrait,
    account: String,
) -> Result<Option<T>, DbError>
where
    T: FromQueryResult + Debug, {
    let account = account.to_lowercase();
    Ok(Entity::find()
        .filter(
            Condition::any()
                .add(Expr::expr(Func::lower(Expr::col(Column::Username))).eq(&account))
                .add(matching_verified_email(&account)),
        )
        .filter(Column::DeletedAt.is_null())
        .into_model::<T>()
        .one(conn)
        .await?)
}

/// Resolves a verified address for account recovery.
pub async fn find_by_verified_email(
    conn: &impl ConnectionTrait,
    email: &str,
) -> Result<Option<UserAccountView>, DbError> {
    Ok(Entity::find()
        .filter(matching_verified_email(&email.to_lowercase()))
        .filter(Column::DeletedAt.is_null())
        .into_model::<UserAccountView>()
        .one(conn)
        .await?)
}

/// Counts rows that match optional filters.
pub async fn count(conn: &impl ConnectionTrait) -> Result<u64, DbError> {
    Ok(Entity::find()
        .filter(Column::DeletedAt.is_null())
        .count(conn)
        .await?)
}

/// Serializes the first local registration so only one account is bootstrapped.
pub async fn is_first_registration(conn: &DatabaseTransaction) -> Result<bool, DbError> {
    if count(conn).await? != 0 {
        return Ok(false);
    }
    // The two-integer namespace is separate from the single-key owner and score
    // locks.
    conn.execute_raw(Statement::from_string(
        DbBackend::Postgres,
        "SELECT pg_advisory_xact_lock(1128551251, 1)".to_owned(),
    ))
    .await?;
    Ok(count(conn).await? == 0)
}

/// Returns whether is username unique.

pub async fn is_username_unique(
    conn: &impl ConnectionTrait,
    user_id: i64,
    username: &str,
) -> Result<bool, DbError> {
    let user = Entity::find()
        .filter(Expr::expr(Func::lower(Expr::col(Column::Username))).eq(username.to_lowercase()))
        .one(conn)
        .await?;

    Ok(user.map(|u| u.id == user_id).unwrap_or(true))
}

/// Returns whether is email unique.

pub async fn is_email_unique(conn: &impl ConnectionTrait, email: &str) -> Result<bool, DbError> {
    Ok(
        crate::email::find_by_email::<EmailView>(conn, email.to_owned())
            .await?
            .is_none(),
    )
}

/// Inserts a new row and returns the persisted model.
pub async fn create<T>(conn: &impl ConnectionTrait, model: ActiveModel) -> Result<T, DbError>
where
    T: FromQueryResult, {
    let user = model
        .insert(conn)
        .await
        .map_err(|error| match error.sql_err() {
            Some(SqlErr::UniqueConstraintViolation(_)) => {
                DbError::Conflict("username_already_exists".to_owned())
            }
            _ => error.into(),
        })?;
    tracing::info!(
        resource = "user",
        action = "create",
        user_id = user.id,
        username = ?user.username,
        banned = user.banned,
        permissions = ?user.permissions,
        "database step completed",
    );

    Ok(find_by_id::<T>(conn, user.id)
        .await?
        .ok_or_else(|| DbError::NotFound(format!("user_{}", user.id)))?)
}

/// Applies an active model update to the database.
pub async fn update<T>(conn: &impl ConnectionTrait, model: ActiveModel) -> Result<T, DbError>
where
    T: FromQueryResult, {
    let changed_fields = [
        ("name", model.name.is_set()),
        ("username", model.username.is_set()),
        ("description", model.description.is_set()),
        ("banned", model.banned.is_set()),
        ("permissions", model.permissions.is_set()),
        ("password", model.hashed_password.is_set()),
        ("avatar_hash", model.avatar_hash.is_set()),
        ("deleted_at", model.deleted_at.is_set()),
    ]
    .into_iter()
    .filter_map(|(name, changed)| changed.then_some(name))
    .collect::<Vec<_>>();
    let user = model.update(conn).await?;
    tracing::info!(
        resource = "user",
        action = "update",
        changed_fields = ?changed_fields,
        user_id = user.id,
        username = ?user.username,
        banned = user.banned,
        permissions = ?user.permissions,
        "database step completed",
    );

    Ok(find_by_id::<T>(conn, user.id)
        .await?
        .ok_or_else(|| DbError::NotFound(format!("user_{}", user.id)))?)
}

/// Updates password.

pub async fn update_password(
    conn: &impl ConnectionTrait,
    user_id: i64,
    hashed_password: String,
) -> Result<(), DbError> {
    let _ = update::<Model>(
        conn,
        ActiveModel {
            id: Unchanged(user_id),
            hashed_password: Set(hashed_password),
            ..Default::default()
        },
    )
    .await?;
    tracing::info!(
        resource = "user",
        action = "update_password",
        user_id = user_id,
        "resource step completed"
    );

    Ok(())
}

/// Locks the active account before changing its emails or recovery credentials.
pub(super) async fn lock_active(
    conn: &DatabaseTransaction,
    user_id: i64,
) -> Result<Model, DbError> {
    Entity::find_by_id(user_id)
        .filter(Column::DeletedAt.is_null())
        .lock_exclusive()
        .one(conn)
        .await?
        .ok_or_else(|| DbError::NotFound(format!("user_{user_id}")))
}

/// Resets a password while the verified address is still linked to this
/// account.
pub async fn reset_password_by_verified_email(
    conn: &impl TransactionTrait<Transaction = DatabaseTransaction>,
    user_id: i64,
    email: &str,
    hashed_password: String,
) -> Result<(), DbError> {
    let transaction = conn.begin().await?;
    lock_active(&transaction, user_id).await?;
    let user = find_by_verified_email(&transaction, email)
        .await?
        .filter(|user| user.id == user_id)
        .ok_or_else(|| DbError::Conflict("email_not_verified".to_owned()))?;
    update_password(&transaction, user.id, hashed_password).await?;
    transaction.commit().await?;
    Ok(())
}

/// Deletes the active account and its associated account data.
pub async fn delete(conn: &DatabaseTransaction, user_id: i64) -> Result<(), DbError> {
    let user = lock_active(conn, user_id).await?;

    let _ = ActiveModel {
        id: Unchanged(user.id),
        username: Set(format!("[DELETED]_{}_{}", user.id, user.username)),
        deleted_at: Set(Some(time::OffsetDateTime::now_utc())),
        ..Default::default()
    }
    .update(conn)
    .await?;

    let _ = super::email::delete_by_user_id(conn, user_id).await?;
    super::user_idp::delete_user_idps_by_user(conn, user_id).await?;
    super::personal_access_token::delete_by_user_id(conn, user_id).await?;
    tracing::info!(
        resource = "user",
        action = "delete",
        user_id,
        username = ?user.username,
        "database step completed",
    );

    Ok(())
}
