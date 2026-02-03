use std::str::FromStr;

use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, FromQueryResult, Order,
    PaginatorTrait, QueryFilter, QueryOrder, QuerySelect,
};
use serde::{Deserialize, Serialize};

pub use crate::entity::game::{ActiveModel, Model, Relation, Timeslot};
use crate::{
    entity::game::{Column, Entity},
    traits::DbError,
};

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromQueryResult)]
pub struct Game {
    pub id: i64,
    pub title: String,
    pub sketch: Option<String>,
    pub description: Option<String>,
    pub enabled: bool,
    pub public: bool,
    pub writeup_required: bool,
    pub member_limit_min: i64,
    pub member_limit_max: i64,
    pub timeslots: Vec<Timeslot>,
    pub started_at: i64,
    pub frozen_at: i64,
    pub ended_at: i64,
    pub has_icon: bool,
    pub has_poster: bool,
    pub created_at: i64,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromQueryResult)]
pub struct GameMini {
    pub id: i64,
    pub title: String,
    pub sketch: Option<String>,
    pub started_at: i64,
    pub frozen_at: i64,
    pub ended_at: i64,
    pub has_icon: bool,
    pub has_poster: bool,
}

#[derive(Clone, Debug, Default)]
pub struct FindGameOptions {
    pub id: Option<i64>,
    pub title: Option<String>,
    pub enabled: Option<bool>,
    pub page: Option<u64>,
    pub size: Option<u64>,
    pub sorts: Option<String>,
}

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

pub async fn count(conn: &impl ConnectionTrait) -> Result<u64, DbError> {
    Ok(Entity::find().count(conn).await?)
}

pub async fn create<T>(conn: &impl ConnectionTrait, model: ActiveModel) -> Result<T, DbError>
where
    T: FromQueryResult, {
    let game = model.insert(conn).await?;

    Ok(find_by_id::<T>(conn, game.id)
        .await?
        .ok_or_else(|| DbError::NotFound(format!("game_{}", game.id)))?)
}

pub async fn update<T>(conn: &impl ConnectionTrait, model: ActiveModel) -> Result<T, DbError>
where
    T: FromQueryResult, {
    let game = model.update(conn).await?;

    Ok(find_by_id::<T>(conn, game.id)
        .await?
        .ok_or_else(|| DbError::NotFound(format!("game_{}", game.id)))?)
}

pub async fn delete(conn: &impl ConnectionTrait, game_id: i64) -> Result<(), DbError> {
    Entity::delete_by_id(game_id).exec(conn).await?;

    Ok(())
}
