//! Database access for `team_user` — SeaORM queries, updates, and DTOs.

use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, FromQueryResult, PaginatorTrait,
    QueryFilter,
};
use tracing::info;

use super::{
    team::{Column as TeamColumn, Entity as TeamEntity},
    user::{Column as UserColumn, Entity as UserEntity},
};
pub(crate) use crate::entity::team_user::{Column, Entity, Relation};
use crate::traits::DbError;
pub use crate::{dto::team_user::TeamUserView, entity::team_user::ActiveModel};

#[derive(Clone, Debug, Default)]
pub struct FindTeamUserOptions {
    pub team_id: Option<i64>,
    pub user_id: Option<i64>,
}

/// Returns whether a user belongs to a team.
pub async fn contains_user(
    conn: &impl ConnectionTrait,
    team_id: i64,
    user_id: i64,
) -> Result<bool, DbError> {
    Ok(membership_query(team_id, user_id).count(conn).await? > 0)
}

fn membership_query(team_id: i64, user_id: i64) -> sea_orm::Select<Entity> {
    Entity::find()
        .filter(Column::TeamId.eq(team_id))
        .filter(Column::UserId.eq(user_id))
}

/// Queries rows using filter options and returns `(rows, total_count)`.
pub async fn find<T>(
    conn: &impl ConnectionTrait,
    FindTeamUserOptions { team_id, user_id }: FindTeamUserOptions,
) -> Result<(Vec<T>, u64), DbError>
where
    T: FromQueryResult, {
    let mut sql = Entity::find();

    if let Some(team_id) = team_id {
        sql = sql.filter(Column::TeamId.eq(team_id));
    }

    if let Some(user_id) = user_id {
        sql = sql.filter(Column::UserId.eq(user_id));
    }

    let total = sql.clone().count(conn).await?;
    let team_users = sql.into_model::<T>().all(conn).await?;

    Ok((team_users, total))
}

/// Looks up by id.

pub async fn find_by_id<T>(
    conn: &impl ConnectionTrait,
    team_id: i64,
    user_id: i64,
) -> Result<Option<T>, DbError>
where
    T: FromQueryResult, {
    Ok(Entity::find()
        .filter(Column::TeamId.eq(team_id))
        .filter(Column::UserId.eq(user_id))
        .into_model::<T>()
        .one(conn)
        .await?)
}

/// Looks up users.

pub async fn find_users<T>(conn: &impl ConnectionTrait, team_id: i64) -> Result<Vec<T>, DbError>
where
    T: FromQueryResult, {
    Ok(UserEntity::find()
        .inner_join(TeamEntity)
        .filter(TeamColumn::Id.eq(team_id))
        .into_model::<T>()
        .all(conn)
        .await?)
}

/// Looks up teams.

pub async fn find_teams<T>(conn: &impl ConnectionTrait, user_id: i64) -> Result<Vec<T>, DbError>
where
    T: FromQueryResult, {
    Ok(TeamEntity::find()
        .inner_join(UserEntity)
        .filter(UserColumn::Id.eq(user_id))
        .into_model::<T>()
        .all(conn)
        .await?)
}

/// Inserts a new row and returns the persisted model.
pub async fn create<T>(conn: &impl ConnectionTrait, model: ActiveModel) -> Result<T, DbError>
where
    T: FromQueryResult, {
    let team_user = model.insert(conn).await?;
    info!(
        team_id = team_user.team_id,
        user_id = team_user.user_id,
        "team member created"
    );

    Ok(find_by_id::<T>(conn, team_user.team_id, team_user.user_id)
        .await?
        .ok_or_else(|| {
            DbError::NotFound(format!(
                "team_user_{}_{}",
                team_user.team_id, team_user.user_id
            ))
        })?)
}

/// Deletes rows matching the provided identifier or filter.
pub async fn delete(
    conn: &impl ConnectionTrait,
    team_id: i64,
    user_id: i64,
) -> Result<(), DbError> {
    Entity::delete_many()
        .filter(Column::TeamId.eq(team_id))
        .filter(Column::UserId.eq(user_id))
        .exec(conn)
        .await?;
    info!(team_id, user_id, "team member deleted");

    Ok(())
}

/// Deletes by team id.

pub async fn delete_by_team_id(conn: &impl ConnectionTrait, team_id: i64) -> Result<(), DbError> {
    Entity::delete_many()
        .filter(Column::TeamId.eq(team_id))
        .exec(conn)
        .await?;
    info!(team_id, "team members deleted");

    Ok(())
}

#[cfg(test)]
mod tests {
    use sea_orm::{DbBackend, QueryTrait};

    use super::*;

    #[test]
    fn membership_query_is_scoped_to_team_and_user() {
        let statement = membership_query(11, 22).build(DbBackend::Postgres);

        assert!(statement.sql.contains("FROM \"team_users\""));
        assert!(statement.sql.contains("\"team_id\" = $1"));
        assert!(statement.sql.contains("\"user_id\" = $2"));
        assert_eq!(statement.values.unwrap().0.len(), 2);
    }
}
