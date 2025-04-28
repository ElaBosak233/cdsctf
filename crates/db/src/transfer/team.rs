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
        }
    }
}
