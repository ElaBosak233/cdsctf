use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use super::{team, Game, Team};
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
    game_id: Option<i64>, team_id: Option<i64>,
) -> Result<(Vec<GameTeam>, u64), DbErr> {
    let mut sql = entity::game_team::Entity::find();

    if let Some(game_id) = game_id {
        sql = sql.filter(entity::game_team::Column::GameId.eq(game_id));
    }

    if let Some(team_id) = team_id {
        sql = sql.filter(entity::game_team::Column::TeamId.eq(team_id));
    }

    let total = sql.clone().count(get_db()).await?;

    let game_teams = sql.all(get_db()).await?;
    let mut game_teams = game_teams
        .into_iter()
        .map(GameTeam::from)
        .collect::<Vec<GameTeam>>();

    game_teams = preload(game_teams).await?;

    Ok((game_teams, total))
}
