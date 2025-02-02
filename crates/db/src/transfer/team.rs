use std::str::FromStr;

use sea_orm::{Iterable, JoinType, Order, QueryOrder, QuerySelect, entity::prelude::*};
use serde::{Deserialize, Serialize};

use super::User;
use crate::{entity, get_db};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Team {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub slogan: Option<String>,
    pub description: Option<String>,
    #[serde(skip_serializing)]
    pub hashed_password: String,
    pub is_locked: bool,
    pub deleted_at: Option<i64>,
    pub created_at: i64,
    pub updated_at: i64,

    pub users: Vec<User>,
}

impl From<entity::team::Model> for Team {
    fn from(model: entity::team::Model) -> Self {
        Self {
            id: model.id,
            name: model.name,
            email: model.email,
            slogan: model.slogan,
            description: model.description,
            hashed_password: model.hashed_password,
            is_locked: model.is_locked,
            deleted_at: model.deleted_at,
            created_at: model.created_at,
            updated_at: model.updated_at,
            users: vec![],
        }
    }
}

impl From<Team> for entity::team::Model {
    fn from(team: Team) -> Self {
        Self {
            id: team.id,
            name: team.name,
            email: team.email,
            slogan: team.slogan,
            description: team.description,
            hashed_password: team.hashed_password,
            is_locked: team.is_locked,
            deleted_at: team.deleted_at,
            created_at: team.created_at,
            updated_at: team.updated_at,
        }
    }
}

impl Team {
    pub fn desensitize(&mut self) {
        for user in self.users.iter_mut() {
            user.desensitize();
        }
    }
}

pub async fn preload(mut teams: Vec<Team>) -> Result<Vec<Team>, DbErr> {
    let models = teams
        .clone()
        .into_iter()
        .map(entity::team::Model::from)
        .collect::<Vec<entity::team::Model>>();
    let users = models
        .load_many_to_many(entity::user::Entity, entity::team_user::Entity, get_db())
        .await?
        .into_iter()
        .map(|users| users.into_iter().map(User::from).collect::<Vec<User>>())
        .collect::<Vec<Vec<User>>>();

    for (i, team) in teams.iter_mut().enumerate() {
        team.users = users[i].clone();
        for user in team.users.iter_mut() {
            user.desensitize();
        }
    }

    Ok(teams)
}

pub async fn find_by_ids(ids: Vec<i64>) -> Result<Vec<Team>, DbErr> {
    let teams = entity::team::Entity::find()
        .filter(entity::team::Column::Id.is_in(ids))
        .all(get_db())
        .await?;

    let mut teams = teams.into_iter().map(Team::from).collect::<Vec<Team>>();

    teams = preload(teams).await?;

    Ok(teams)
}

pub async fn find_by_user_id(id: i64) -> Result<Vec<Team>, DbErr> {
    let teams = entity::team::Entity::find()
        .select_only()
        .columns(entity::team::Column::iter())
        .filter(entity::team_user::Column::UserId.eq(id))
        .join(JoinType::InnerJoin, entity::team_user::Relation::Team.def())
        .into_model::<entity::team::Model>()
        .all(get_db())
        .await?;

    let mut teams = teams.into_iter().map(Team::from).collect::<Vec<Team>>();

    teams = preload(teams).await?;

    Ok(teams)
}
