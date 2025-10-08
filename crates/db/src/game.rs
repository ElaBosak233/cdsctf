use std::str::FromStr;

use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, FromQueryResult, Order, PaginatorTrait,
    QueryFilter, QueryOrder, QuerySelect,
};
use serde::{Deserialize, Serialize};

pub use crate::entity::game::{ActiveModel, Model, Relation, Timeslot};
use crate::{
    entity::game::{Column, Entity},
    get_db,
};

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromQueryResult)]
pub struct Game {
    pub id: i64,
    pub title: String,
    pub sketch: Option<String>,
    pub description: Option<String>,
    pub is_enabled: bool,
    pub is_public: bool,
    pub is_need_write_up: bool,
    pub member_limit_min: i64,
    pub member_limit_max: i64,
    pub timeslots: Vec<Timeslot>,
    pub started_at: i64,
    pub frozen_at: i64,
    pub ended_at: i64,
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
}

#[derive(Clone, Debug, Default)]
pub struct FindGameOptions {
    pub id: Option<i64>,
    pub title: Option<String>,
    pub is_enabled: Option<bool>,
    pub page: Option<u64>,
    pub size: Option<u64>,
    pub sorts: Option<String>,
}

pub async fn find<T>(
    FindGameOptions {
        id,
        title,
        is_enabled,
        page,
        size,
        sorts,
    }: FindGameOptions,
) -> Result<(Vec<T>, u64), DbErr>
where
    T: FromQueryResult, {
    let mut sql = Entity::find();

    if let Some(id) = id {
        sql = sql.filter(Column::Id.eq(id));
    }

    if let Some(title) = title {
        sql = sql.filter(Column::Title.contains(title));
    }

    if let Some(is_enabled) = is_enabled {
        sql = sql.filter(Column::IsEnabled.eq(is_enabled));
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

    let total = sql.clone().count(get_db()).await?;

    if let (Some(page), Some(size)) = (page, size) {
        let offset = (page - 1) * size;
        sql = sql.offset(offset).limit(size);
    }

    let games = sql.into_model::<T>().all(get_db()).await?;

    Ok((games, total))
}

pub async fn find_by_id<T>(game_id: i64) -> Result<Option<T>, DbErr>
where
    T: FromQueryResult, {
    Ok(Entity::find_by_id(game_id)
        .into_model::<T>()
        .one(get_db())
        .await?)
}

pub async fn count() -> Result<u64, DbErr> {
    Ok(Entity::find().count(get_db()).await?)
}

pub async fn create<T>(model: ActiveModel) -> Result<T, DbErr>
where
    T: FromQueryResult, {
    let game = model.insert(get_db()).await?;

    Ok(find_by_id::<T>(game.id).await?.unwrap())
}

pub async fn update<T>(model: ActiveModel) -> Result<T, DbErr>
where
    T: FromQueryResult, {
    let game = model.update(get_db()).await?;

    Ok(find_by_id::<T>(game.id).await?.unwrap())
}

pub async fn delete(game_id: i64) -> Result<(), DbErr> {
    Entity::delete_by_id(game_id).exec(get_db()).await?;

    Ok(())
}
