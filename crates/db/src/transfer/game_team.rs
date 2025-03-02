use std::str::FromStr;

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{entity, get_db};
use crate::transfer::{User};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GameTeam {
    pub id: i64,
    pub game_id: i64,
    pub name: String,
    pub email: Option<String>,
    pub slogan: Option<String>,
    pub description: Option<String>,
    pub is_allowed: bool,
    pub pts: i64,
    pub rank: i64,
    pub deleted_at: Option<i64>,

    pub users: Vec<User>
}

impl From<entity::game_team::Model> for GameTeam {
    fn from(entity: entity::game_team::Model) -> Self {
        Self {
            id: entity.id,
            game_id: entity.game_id,
            name: entity.name,
            is_allowed: entity.is_allowed,
            pts: entity.pts,
            rank: entity.rank,
            email: entity.email,
            slogan: entity.slogan,
            description: entity.description,
            deleted_at: entity.deleted_at,
            users: vec![]
        }
    }
}

impl From<GameTeam> for entity::game_team::Model {
    fn from(game_team: GameTeam) -> Self {
        Self {
            id: game_team.id,
            game_id: game_team.game_id,
            name: game_team.name,
            is_allowed: game_team.is_allowed,
            pts: game_team.pts,
            rank: game_team.rank,
            email: game_team.email,
            slogan: game_team.slogan,
            description: game_team.description,
            deleted_at: game_team.deleted_at,
        }
    }
}

pub async fn preload(mut game_teams: Vec<GameTeam>) -> Result<Vec<GameTeam>, DbErr> {
    let models = game_teams
        .clone()
        .into_iter()
        .map(|game_team| entity::game_team::Model::from(game_team))
        .collect::<Vec<entity::game_team::Model>>();

    let users = models
        .load_many_to_many(entity::user::Entity, entity::game_team_user::Entity, get_db())
        .await?
        .into_iter()
        .map(|users| {
            users
                .into_iter()
                .map(|user| super::User::from(user))
                .collect::<Vec<User>>()
        })
        .collect::<Vec<Vec<User>>>();

    for (i, game_team) in game_teams.iter_mut().enumerate() {
        game_team.users = users[i].clone();
        for user in game_team.users.iter_mut() {
            user.desensitize();
        }
    }

    Ok(game_teams)
}