use std::str::FromStr;

use sea_orm::{entity::prelude::*, Order, QueryOrder, QuerySelect};
use serde::{Deserialize, Serialize};

use super::{team, Game, Submission, Team};
use crate::db::{entity, get_db};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GameTeam {
    pub game_id: i64,
    pub team_id: i64,
    pub is_allowed: bool,

    pub pts: i64,
    pub rank: i64,

    pub game: Option<Game>,
    pub team: Option<Team>,
}

impl From<entity::game_team::Model> for GameTeam {
    fn from(entity: entity::game_team::Model) -> Self {
        Self {
            game_id: entity.game_id,
            team_id: entity.team_id,
            is_allowed: entity.is_allowed,
            pts: entity.pts,
            rank: entity.rank,
            game: None,
            team: None,
        }
    }
}

async fn preload(mut game_teams: Vec<GameTeam>) -> Result<Vec<GameTeam>, DbErr> {
    let team_ids: Vec<i64> = game_teams
        .iter()
        .map(|game_team| game_team.team_id)
        .collect();

    let teams = team::find_by_ids(team_ids).await?;

    for game_team in game_teams.iter_mut() {
        game_team.team = teams
            .iter()
            .find(|team| team.id == game_team.team_id)
            .cloned();
    }

    Ok(game_teams)
}

pub async fn find(
    game_id: Option<i64>, team_id: Option<i64>, is_allowed: Option<bool>, sorts: Option<String>,
    page: Option<u64>, size: Option<u64>,
) -> Result<(Vec<GameTeam>, u64), DbErr> {
    let mut sql = entity::game_team::Entity::find();

    if let Some(game_id) = game_id {
        sql = sql.filter(entity::game_team::Column::GameId.eq(game_id));
    }

    if let Some(team_id) = team_id {
        sql = sql.filter(entity::game_team::Column::TeamId.eq(team_id));
    }

    if let Some(is_allowed) = is_allowed {
        sql = sql.filter(entity::game_team::Column::IsAllowed.eq(is_allowed));
    }

    if let Some(sorts) = sorts {
        let sorts = sorts.split(",").collect::<Vec<&str>>();
        for sort in sorts {
            let col = match crate::db::entity::game_team::Column::from_str(
                sort.replace("-", "").as_str(),
            ) {
                Ok(col) => col,
                Err(_) => return Err(DbErr::Custom("invalid sort column".to_string())),
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

    let game_teams = sql.all(get_db()).await?;
    let mut game_teams = game_teams
        .into_iter()
        .map(GameTeam::from)
        .collect::<Vec<GameTeam>>();

    game_teams = preload(game_teams).await?;

    Ok((game_teams, total))
}
