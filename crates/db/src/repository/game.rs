//! Database access for `game` — SeaORM queries, updates, and DTOs.

use std::str::FromStr;

use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, FromQueryResult, JoinType, Order,
    PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, RelationTrait,
    sea_query::{Expr, extension::postgres::PgExpr},
};

pub use crate::{
    dto::game::{GameDetail, GameSummary, GameView},
    entity::game::{ActiveModel, Model, Relation, Timeslot},
};
use crate::{
    entity::game::{Column, Entity},
    traits::DbError,
};

#[derive(Clone, Debug, Default)]
pub struct FindGameOptions {
    pub id: Option<i64>,
    pub title: Option<String>,
    pub keyword: Option<String>,
    pub enabled: Option<bool>,
    pub page: Option<u64>,
    pub size: Option<u64>,
    pub sorts: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, FromQueryResult)]
pub struct UserGameSummary {
    pub id: i64,
    pub title: String,
    pub created_at: time::OffsetDateTime,
}

/// Minimal competition time window needed by submission finalization.
#[derive(Clone, Copy, Debug, PartialEq, Eq, FromQueryResult)]
pub struct FreezeWindow {
    pub frozen_at: time::OffsetDateTime,
    pub ended_at: time::OffsetDateTime,
}

/// Loads only `frozen_at` and `ended_at` for one game.
pub async fn find_freeze_window(
    conn: &impl ConnectionTrait,
    game_id: i64,
) -> Result<Option<FreezeWindow>, DbError> {
    Ok(Entity::find_by_id(game_id)
        .select_only()
        .columns([Column::FrozenAt, Column::EndedAt])
        .into_model::<FreezeWindow>()
        .one(conn)
        .await?)
}

/// Returns the most recently created games in which a user participated.
pub async fn find_recent_by_user_id(
    conn: &impl ConnectionTrait,
    user_id: i64,
    limit: u64,
) -> Result<Vec<UserGameSummary>, DbError> {
    Ok(Entity::find()
        .select_only()
        .column(Column::Id)
        .column(Column::Title)
        .column(Column::CreatedAt)
        .join(
            JoinType::InnerJoin,
            crate::entity::team::Relation::Game.def().rev(),
        )
        .join(
            JoinType::InnerJoin,
            crate::entity::team_user::Relation::Team.def().rev(),
        )
        .filter(crate::entity::team_user::Column::UserId.eq(user_id))
        .distinct()
        .order_by_desc(Column::CreatedAt)
        .limit(limit)
        .into_model::<UserGameSummary>()
        .all(conn)
        .await?)
}

/// Returns all game identifiers in deterministic order.
pub async fn find_ids(conn: &impl ConnectionTrait) -> Result<Vec<i64>, DbError> {
    Ok(Entity::find()
        .select_only()
        .column(Column::Id)
        .order_by_asc(Column::Id)
        .into_tuple::<i64>()
        .all(conn)
        .await?)
}

/// Queries rows using filter options and returns `(rows, total_count)`.
pub async fn find<T>(
    conn: &impl ConnectionTrait,
    FindGameOptions {
        id,
        title,
        keyword,
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

    if let Some(keyword) = keyword {
        sql = sql.filter(Expr::col(Column::Title).ilike(super::literal_search_pattern(&keyword)));
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
        let offset = std::cmp::min(page.saturating_sub(1).saturating_mul(size), i64::MAX as u64);
        sql = sql.order_by_asc(Column::Id).offset(offset).limit(size);
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
    tracing::info!(
        resource = "game",
        action = "create",
        game_id = game.id,
        title = ?game.title,
        enabled = ?game.enabled,
        public = ?game.public,
        started_at = ?game.started_at,
        ended_at = ?game.ended_at,
        "database step completed",
    );

    Ok(find_by_id::<T>(conn, game.id)
        .await?
        .ok_or_else(|| DbError::NotFound(format!("game_{}", game.id)))?)
}

/// Applies an active model update to the database.
pub async fn update<T>(conn: &impl ConnectionTrait, model: ActiveModel) -> Result<T, DbError>
where
    T: FromQueryResult, {
    let changed_fields = [
        ("title", model.title.is_set()),
        ("sketch", model.sketch.is_set()),
        ("description", model.description.is_set()),
        ("enabled", model.enabled.is_set()),
        ("public", model.public.is_set()),
        ("paused", model.paused.is_set()),
        ("blacked_out", model.blacked_out.is_set()),
        ("member_limit_min", model.member_limit_min.is_set()),
        ("member_limit_max", model.member_limit_max.is_set()),
        ("writeup_required", model.writeup_required.is_set()),
        ("timeslots", model.timeslots.is_set()),
        ("started_at", model.started_at.is_set()),
        ("frozen_at", model.frozen_at.is_set()),
        ("ended_at", model.ended_at.is_set()),
        ("icon_hash", model.icon_hash.is_set()),
        ("poster_hash", model.poster_hash.is_set()),
    ]
    .into_iter()
    .filter_map(|(name, changed)| changed.then_some(name))
    .collect::<Vec<_>>();
    let game = model.update(conn).await?;
    tracing::info!(
        resource = "game",
        action = "update",
        changed_fields = ?changed_fields,
        game_id = game.id,
        title = ?game.title,
        enabled = ?game.enabled,
        public = ?game.public,
        started_at = ?game.started_at,
        ended_at = ?game.ended_at,
        "database step completed",
    );

    Ok(find_by_id::<T>(conn, game.id)
        .await?
        .ok_or_else(|| DbError::NotFound(format!("game_{}", game.id)))?)
}

/// Deletes rows matching the provided identifier or filter.
pub async fn delete(conn: &impl ConnectionTrait, game_id: i64) -> Result<(), DbError> {
    Entity::delete_by_id(game_id).exec(conn).await?;
    tracing::info!(
        resource = "game",
        action = "delete",
        game_id = game_id,
        "resource step completed"
    );

    Ok(())
}
