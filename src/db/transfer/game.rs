use std::str::FromStr;
use sea_orm::{entity::prelude::*, Order, QueryOrder, QuerySelect};
use serde::{Deserialize, Serialize};

use crate::db::{entity, get_db};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Game {
    pub id: i64,
    pub title: String,
    pub sketch: Option<String>,
    pub description: Option<String>,
    pub is_enabled: bool,
    pub is_public: bool,
    pub member_limit_min: i64,
    pub member_limit_max: i64,
    pub parallel_container_limit: i64,
    pub is_need_write_up: bool,
    pub started_at: i64,
    pub frozen_at: i64,
    pub ended_at: i64,
    pub created_at: i64,
    pub updated_at: i64,
}

impl From<entity::game::Model> for Game {
    fn from(model: entity::game::Model) -> Self {
        Self {
            id: model.id,
            title: model.title,
            sketch: model.sketch,
            description: model.description,
            is_enabled: model.is_enabled,
            is_public: model.is_public,
            member_limit_min: model.member_limit_min,
            member_limit_max: model.member_limit_max,
            parallel_container_limit: model.parallel_container_limit,
            is_need_write_up: model.is_need_write_up,
            started_at: model.started_at,
            frozen_at: model.frozen_at,
            ended_at: model.ended_at,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}

impl From<Game> for entity::game::Model {
    fn from(game: Game) -> Self {
        Self {
            id: game.id,
            title: game.title,
            sketch: game.sketch,
            description: game.description,
            is_enabled: game.is_enabled,
            is_public: game.is_public,
            member_limit_min: game.member_limit_min,
            member_limit_max: game.member_limit_max,
            parallel_container_limit: game.parallel_container_limit,
            is_need_write_up: game.is_need_write_up,
            started_at: game.started_at,
            frozen_at: game.frozen_at,
            ended_at: game.ended_at,
            created_at: game.created_at,
            updated_at: game.updated_at,
        }
    }
}

pub async fn find(
    id: Option<i64>, title: Option<String>, is_enabled: Option<bool>, sorts: Option<String>, page: Option<u64>,
    size: Option<u64>,
) -> Result<(Vec<entity::game::Model>, u64), DbErr> {
    let mut sql = entity::game::Entity::find();

    if let Some(id) = id {
        sql = sql.filter(entity::game::Column::Id.eq(id));
    }

    if let Some(title) = title {
        sql = sql.filter(entity::game::Column::Title.contains(title));
    }

    if let Some(is_enabled) = is_enabled {
        sql = sql.filter(entity::game::Column::IsEnabled.eq(is_enabled));
    }

    if let Some(sorts) = sorts {
        let sorts = sorts.split(",").collect::<Vec<&str>>();
        for sort in sorts {
            let col = match crate::db::entity::game::Column::from_str(
                sort.replace("-", "").as_str(),
            ) {
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

    if let Some(page) = page {
        if let Some(size) = size {
            let offset = (page - 1) * size;
            sql = sql.offset(offset).limit(size);
        }
    }

    let games = sql.all(get_db()).await?;

    Ok((games, total))
}
