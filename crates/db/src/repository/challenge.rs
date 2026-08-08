//! Database access for `challenge` — SeaORM queries, updates, and DTOs.

use std::str::FromStr;

use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, ConnectionTrait, EntityName, EntityTrait,
    FromQueryResult, Iden as _, JoinType, Order, PaginatorTrait, QueryFilter, QueryOrder,
    QuerySelect, QueryTrait, RelationTrait, Set, prelude::Expr,
};
use tracing::info;

pub(crate) use crate::entity::challenge::{Column, Entity};
use crate::traits::DbError;
pub use crate::{
    dto::challenge::{ChallengeDetail, ChallengeSummary, ChallengeView},
    entity::challenge::{ActiveModel, Container, EnvVar, Instance, Model, Port},
};

#[derive(Clone, Debug, Default)]
pub struct FindChallengeOptions {
    pub id: Option<i64>,
    pub title: Option<String>,
    pub category: Option<i32>,
    pub tag: Option<String>,
    pub public: Option<bool>,
    pub has_instance: Option<bool>,
    pub page: Option<u64>,
    pub size: Option<u64>,
    pub sorts: Option<String>,
}

/// Checks whether a challenge is public or available through an active game
/// in which the user is a passed team member.
pub async fn can_user_access(
    conn: &impl ConnectionTrait,
    user_id: i64,
    challenge_id: i64,
) -> Result<bool, DbError> {
    Ok(user_access_query(
        user_id,
        challenge_id,
        time::OffsetDateTime::now_utc().unix_timestamp(),
    )
    .count(conn)
    .await?
        > 0)
}

fn user_access_query(user_id: i64, challenge_id: i64, now: i64) -> sea_orm::Select<Entity> {
    let active_game_access = active_game_access_query(user_id, challenge_id, now)
        .select_only()
        .column(crate::entity::game::Column::Id)
        .into_query();

    Entity::find_by_id(challenge_id).filter(
        Condition::any()
            .add(Column::Public.eq(true))
            .add(Expr::exists(active_game_access)),
    )
}

fn active_game_access_query(
    user_id: i64,
    challenge_id: i64,
    now: i64,
) -> sea_orm::Select<crate::entity::game::Entity> {
    crate::entity::game::Entity::find()
        .join(
            JoinType::InnerJoin,
            crate::entity::game_challenge::Relation::Game.def().rev(),
        )
        .join(
            JoinType::InnerJoin,
            crate::entity::team::Relation::Game.def().rev(),
        )
        .join(
            JoinType::InnerJoin,
            crate::entity::team_user::Relation::Team.def().rev(),
        )
        .filter(crate::entity::game_challenge::Column::ChallengeId.eq(challenge_id))
        .filter(crate::entity::team_user::Column::UserId.eq(user_id))
        .filter(crate::entity::team::Column::State.eq(crate::entity::team::State::Passed))
        .filter(crate::entity::game::Column::Enabled.eq(true))
        .filter(crate::entity::game::Column::Paused.eq(false))
        .filter(crate::entity::game::Column::StartedAt.lte(now))
        .filter(crate::entity::game::Column::EndedAt.gte(now))
}

/// Queries rows using filter options and returns `(rows, total_count)`.
pub async fn find<T>(
    conn: &impl ConnectionTrait,
    FindChallengeOptions {
        id,
        title,
        category,
        tag,
        public,
        has_instance,
        page,
        size,
        sorts,
    }: FindChallengeOptions,
) -> Result<(Vec<T>, u64), DbError>
where
    T: FromQueryResult, {
    let mut sql = Entity::find();

    if let Some(id) = id {
        sql = sql.filter(Column::Id.eq(id));
    }

    if let Some(title) = title {
        sql = sql.filter(Column::Title.contains(title));
    }

    if let Some(category) = category {
        sql = sql.filter(Column::Category.eq(category));
    }

    if let Some(tag) = tag {
        sql = sql.filter(Expr::cust_with_expr(
            format!(
                "\"{}\".\"{}\" @> $1::text[]",
                Entity.table_name(),
                Column::Tags.to_string()
            ),
            vec![tag],
        ))
    }

    if let Some(public) = public {
        sql = sql.filter(Column::Public.eq(public));
    }

    if let Some(has_instance) = has_instance {
        sql = sql.filter(Column::HasInstance.eq(has_instance));
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
        let offset = (page - 1) * size;
        sql = sql.offset(offset).limit(size);
    }

    let challenges = sql.into_model::<T>().all(conn).await?;

    Ok((challenges, total))
}

/// Looks up by id.

pub async fn find_by_id<T>(
    conn: &impl ConnectionTrait,
    challenge_id: i64,
) -> Result<Option<T>, DbError>
where
    T: FromQueryResult, {
    Ok(Entity::find_by_id(challenge_id)
        .filter(Column::DeletedAt.is_null())
        .into_model::<T>()
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

/// Inserts a new row and returns the persisted model.
pub async fn create<T>(conn: &impl ConnectionTrait, model: ActiveModel) -> Result<T, DbError>
where
    T: FromQueryResult, {
    let challenge = model.insert(conn).await?;
    info!(
        challenge_id = challenge.id,
        title = %challenge.title,
        category = challenge.category,
        public = challenge.public,
        has_instance = challenge.has_instance,
        has_attachment = challenge.has_attachment,
        "challenge created"
    );

    Ok(find_by_id::<T>(conn, challenge.id)
        .await?
        .ok_or_else(|| DbError::NotFound(format!("challenge_{}", challenge.id)))?)
}

/// Applies an active model update to the database.
pub async fn update<T>(conn: &impl ConnectionTrait, model: ActiveModel) -> Result<T, DbError>
where
    T: FromQueryResult, {
    let challenge = model.update(conn).await?;
    info!(
        challenge_id = challenge.id,
        title = %challenge.title,
        category = challenge.category,
        public = challenge.public,
        has_instance = challenge.has_instance,
        has_attachment = challenge.has_attachment,
        "challenge updated"
    );

    Ok(find_by_id::<T>(conn, challenge.id)
        .await?
        .ok_or_else(|| DbError::NotFound(format!("challenge_{}", challenge.id)))?)
}

/// Deletes rows matching the provided identifier or filter.
pub async fn delete(conn: &impl ConnectionTrait, challenge_id: i64) -> Result<(), DbError> {
    let challenge = find_by_id::<Model>(conn, challenge_id)
        .await?
        .ok_or_else(|| DbError::NotFound(format!("challenge_{challenge_id}")))?;

    let _ = ActiveModel {
        id: Set(challenge.id),
        deleted_at: Set(Some(time::OffsetDateTime::now_utc().unix_timestamp())),
        ..Default::default()
    }
    .update(conn)
    .await?;
    info!(
        challenge_id,
        title = %challenge.title,
        "challenge deleted"
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use sea_orm::{DbBackend, QueryTrait};

    use super::*;

    #[test]
    fn active_game_access_query_covers_membership_state_and_time() {
        let statement = active_game_access_query(7, 9, 1_000).build(DbBackend::Postgres);

        assert!(statement.sql.contains("INNER JOIN \"game_challenges\""));
        assert!(statement.sql.contains("INNER JOIN \"teams\""));
        assert!(statement.sql.contains("INNER JOIN \"team_users\""));
        assert!(statement.sql.contains("\"challenge_id\" = $1"));
        assert!(statement.sql.contains("\"user_id\" = $2"));
        assert!(statement.sql.contains("\"state\" = $3"));
        assert!(statement.sql.contains("\"enabled\" = $4"));
        assert!(statement.sql.contains("\"paused\" = $5"));
        assert!(statement.sql.contains("\"started_at\" <= $6"));
        assert!(statement.sql.contains("\"ended_at\" >= $7"));
        assert_eq!(statement.values.unwrap().0.len(), 7);
    }

    #[test]
    fn user_access_query_combines_public_and_game_access_in_one_statement() {
        let statement = user_access_query(7, 9, 1_000).build(DbBackend::Postgres);

        assert_eq!(statement.sql.matches("SELECT").count(), 2);
        assert!(statement.sql.contains("\"challenges\".\"id\" = $1"));
        assert!(statement.sql.contains("\"challenges\".\"public\" = $2"));
        assert!(statement.sql.contains(" OR EXISTS(SELECT"));
        assert_eq!(statement.values.unwrap().0.len(), 9);
    }
}
