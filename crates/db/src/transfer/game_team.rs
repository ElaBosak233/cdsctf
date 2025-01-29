use std::str::FromStr;

use sea_orm::{Order, QueryOrder, QuerySelect, entity::prelude::*};
use serde::{Deserialize, Serialize};

use super::{Game, Team, team};
use crate::{entity, get_db};

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

pub async fn preload(mut game_teams: Vec<GameTeam>) -> Result<Vec<GameTeam>, DbErr> {
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
