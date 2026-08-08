//! Database access for `game` — SeaORM queries, updates, and DTOs.

use std::str::FromStr;

use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, FromQueryResult, Order,
    PaginatorTrait, QueryFilter, QueryOrder, QuerySelect,
    sea_query::{Alias, Expr, ExprTrait, Func, Query},
};
use tracing::info;

pub use crate::{
    dto::game::{GameDetail, GameSummary, GameView},
    entity::game::{ActiveModel, Model, Relation, Timeslot},
};
use crate::{
    entity::game::{Column, Entity},
    traits::DbError,
};

const SCORE_LOCK_NAMESPACE: i64 = 0x4344_5300_0000_0000;

#[derive(Clone, Debug, Default)]
pub struct FindGameOptions {
    pub id: Option<i64>,
    pub title: Option<String>,
    pub enabled: Option<bool>,
    pub page: Option<u64>,
    pub size: Option<u64>,
    pub sorts: Option<String>,
}

/// Returns all game identifiers in deterministic order for a full rebuild.
pub async fn find_ids(conn: &impl ConnectionTrait) -> Result<Vec<i64>, DbError> {
    Ok(Entity::find()
        .select_only()
        .column(Column::Id)
        .order_by_asc(Column::Id)
        .into_tuple::<i64>()
        .all(conn)
        .await?)
}

/// Returns games whose requested score revision has not been applied.
pub async fn find_dirty_score_ids(conn: &impl ConnectionTrait) -> Result<Vec<i64>, DbError> {
    Ok(Entity::find()
        .select_only()
        .column(Column::Id)
        .filter(Column::ScoreRevision.ne(0_i64))
        .order_by_asc(Column::Id)
        .into_tuple::<i64>()
        .all(conn)
        .await?)
}

/// Increments the requested score revision for a game.
pub async fn request_score_recalculation(
    conn: &impl ConnectionTrait,
    game_id: i64,
) -> Result<(), DbError> {
    let result = score_recalculation_request(game_id).exec(conn).await?;
    if result.rows_affected == 0 {
        return Err(DbError::NotFound(format!("game_{game_id}")));
    }
    Ok(())
}

fn score_recalculation_request(game_id: i64) -> sea_orm::UpdateMany<Entity> {
    Entity::update_many()
        .col_expr(
            Column::ScoreRevision,
            Expr::col(Column::ScoreRevision).add(1_i64),
        )
        .filter(Column::Id.eq(game_id))
}

/// Loads the current dirty generation for one game (`0` means clean).
pub async fn find_score_revision(
    conn: &impl ConnectionTrait,
    game_id: i64,
) -> Result<Option<i64>, DbError> {
    Ok(Entity::find_by_id(game_id)
        .select_only()
        .column(Column::ScoreRevision)
        .into_tuple::<i64>()
        .one(conn)
        .await?)
}

/// Clears dirty state only if no newer recalculation request has arrived.
pub async fn mark_score_recalculation_applied(
    conn: &impl ConnectionTrait,
    game_id: i64,
    revision: i64,
) -> Result<bool, DbError> {
    Ok(score_recalculation_applied(game_id, revision)
        .exec(conn)
        .await?
        .rows_affected
        == 1)
}

fn score_recalculation_applied(game_id: i64, revision: i64) -> sea_orm::UpdateMany<Entity> {
    Entity::update_many()
        .col_expr(Column::ScoreRevision, Expr::value(0_i64))
        .filter(Column::Id.eq(game_id))
        .filter(Column::ScoreRevision.eq(revision))
}

/// Serializes score recomputation for one game across database clients.
pub async fn lock_score_recalculation(
    conn: &impl ConnectionTrait,
    game_id: i64,
) -> Result<(), DbError> {
    let query = score_lock_query(game_id);
    conn.query_one(&query).await?;
    Ok(())
}

fn score_lock_query(game_id: i64) -> sea_orm::sea_query::SelectStatement {
    Query::select()
        .expr(Func::cust(Alias::new("pg_advisory_xact_lock")).arg(score_lock_key(game_id)))
        .to_owned()
}

fn score_lock_key(game_id: i64) -> i64 {
    SCORE_LOCK_NAMESPACE.wrapping_add(game_id)
}

/// Queries rows using filter options and returns `(rows, total_count)`.
pub async fn find<T>(
    conn: &impl ConnectionTrait,
    FindGameOptions {
        id,
        title,
        enabled,
        page,
        size,
        sorts,
    }: FindGameOptions,
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

    if let Some(enabled) = enabled {
        sql = sql.filter(Column::Enabled.eq(enabled));
    }

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

    let total = sql.clone().count(conn).await?;

    if let (Some(page), Some(size)) = (page, size) {
        let offset = (page - 1) * size;
        sql = sql.offset(offset).limit(size);
    }

    let games = sql.into_model::<T>().all(conn).await?;

    Ok((games, total))
}

/// Looks up by id.

pub async fn find_by_id<T>(
    conn: &impl ConnectionTrait,
    game_id: i64,
) -> Result<Option<T>, DbError>
where
    T: FromQueryResult, {
    Ok(Entity::find_by_id(game_id)
        .into_model::<T>()
        .one(conn)
        .await?)
}

/// Counts rows that match optional filters.
pub async fn count(conn: &impl ConnectionTrait) -> Result<u64, DbError> {
    Ok(Entity::find().count(conn).await?)
}

/// Inserts a new row and returns the persisted model.
pub async fn create<T>(conn: &impl ConnectionTrait, model: ActiveModel) -> Result<T, DbError>
where
    T: FromQueryResult, {
    let game = model.insert(conn).await?;
    info!(
        game_id = game.id,
        title = %game.title,
        enabled = game.enabled,
        public = game.public,
        started_at = game.started_at,
        ended_at = game.ended_at,
        "game created"
    );

    Ok(find_by_id::<T>(conn, game.id)
        .await?
        .ok_or_else(|| DbError::NotFound(format!("game_{}", game.id)))?)
}

/// Applies an active model update to the database.
pub async fn update<T>(conn: &impl ConnectionTrait, model: ActiveModel) -> Result<T, DbError>
where
    T: FromQueryResult, {
    let game = model.update(conn).await?;
    info!(
        game_id = game.id,
        title = %game.title,
        enabled = game.enabled,
        public = game.public,
        started_at = game.started_at,
        ended_at = game.ended_at,
        "game updated"
    );

    Ok(find_by_id::<T>(conn, game.id)
        .await?
        .ok_or_else(|| DbError::NotFound(format!("game_{}", game.id)))?)
}

/// Deletes rows matching the provided identifier or filter.
pub async fn delete(conn: &impl ConnectionTrait, game_id: i64) -> Result<(), DbError> {
    Entity::delete_by_id(game_id).exec(conn).await?;
    info!(game_id, "game deleted");

    Ok(())
}

#[cfg(test)]
mod score_tests {
    use sea_orm::{DbBackend, QueryTrait};

    use super::*;

    #[test]
    fn score_lock_is_namespaced_and_built_by_sea_query() {
        assert_ne!(score_lock_key(1), score_lock_key(2));

        let statement = DbBackend::Postgres.build(&score_lock_query(7));
        assert_eq!(statement.sql, "SELECT pg_advisory_xact_lock($1)");
        assert_eq!(statement.values.unwrap().0.len(), 1);
    }

    #[test]
    fn score_revision_updates_are_atomic_and_compare_before_apply() {
        let request = score_recalculation_request(7).build(DbBackend::Postgres);
        assert!(request.sql.contains("\"score_revision\" + $1"));
        assert!(request.sql.contains("\"games\".\"id\" = $2"));

        let apply = score_recalculation_applied(7, 4).build(DbBackend::Postgres);
        assert!(apply.sql.contains("\"score_revision\" = $1"));
        assert!(apply.sql.contains("\"games\".\"id\" = $2"));
        assert!(apply.sql.contains("\"score_revision\" = $3"));
        assert_eq!(apply.values.unwrap().0.len(), 3);
    }
}
