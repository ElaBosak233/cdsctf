use std::str::FromStr;

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{entity, entity::team::State, get_db, transfer::User};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Team {
    pub id: i64,
    pub game_id: i64,
    pub name: String,
    pub email: Option<String>,
    pub slogan: Option<String>,
    pub description: Option<String>,
    pub state: State,
    pub pts: i64,
    pub rank: i64,

    pub users: Vec<User>,
}

impl From<entity::team::Model> for Team {
    fn from(entity: entity::team::Model) -> Self {
        Self {
            id: entity.id,
            game_id: entity.game_id,
            name: entity.name,
            state: entity.state,
            pts: entity.pts,
            rank: entity.rank,
            email: entity.email,
            slogan: entity.slogan,
            description: entity.description,
            users: vec![],
        }
    }
}

impl From<Team> for entity::team::Model {
    fn from(team: Team) -> Self {
        Self {
            id: team.id,
            game_id: team.game_id,
            name: team.name,
            state: team.state,
            pts: team.pts,
            rank: team.rank,
            email: team.email,
            slogan: team.slogan,
            description: team.description,
        }
    }
}

pub async fn preload(mut teams: Vec<Team>) -> Result<Vec<Team>, DbErr> {
    let models = teams
        .clone()
        .into_iter()
        .map(|team| entity::team::Model::from(team))
        .collect::<Vec<entity::team::Model>>();

    let users = models
        .load_many_to_many(entity::user::Entity, entity::team_user::Entity, get_db())
        .await?
        .into_iter()
        .map(|users| {
            users
                .into_iter()
                .map(|user| super::User::from(user))
                .collect::<Vec<User>>()
        })
        .collect::<Vec<Vec<User>>>();

    for (i, team) in teams.iter_mut().enumerate() {
        team.users = users[i].clone();
        for user in team.users.iter_mut() {
            user.desensitize();
        }
    }

    Ok(teams)
}
