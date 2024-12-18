use sea_orm::{Iterable, JoinType, QuerySelect, entity::prelude::*};
use serde::{Deserialize, Serialize};

use super::User;
use crate::{entity, get_db};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Team {
    pub id: i64,
    pub name: String,
    pub email: Option<String>,
    pub captain_id: i64,
    pub slogan: Option<String>,
    pub invite_token: Option<String>,
    pub is_deleted: bool,
    pub created_at: i64,
    pub updated_at: i64,

    pub users: Vec<User>,
    pub captain: Option<User>,
}

impl From<entity::team::Model> for Team {
    fn from(model: entity::team::Model) -> Self {
        Self {
            id: model.id,
            name: model.name,
            email: model.email,
            captain_id: model.captain_id,
            slogan: model.slogan,
            invite_token: model.invite_token,
            is_deleted: model.is_deleted,
            created_at: model.created_at,
            updated_at: model.updated_at,
            users: vec![],
            captain: None,
        }
    }
}

impl From<Team> for entity::team::Model {
    fn from(team: Team) -> Self {
        Self {
            id: team.id,
            name: team.name,
            email: team.email,
            captain_id: team.captain_id,
            slogan: team.slogan,
            invite_token: team.invite_token,
            is_deleted: team.is_deleted,
            created_at: team.created_at,
            updated_at: team.updated_at,
        }
    }
}

impl Team {
    pub fn desensitize(&mut self) {
        self.invite_token = None;
        if let Some(captain) = self.captain.as_mut() {
            captain.desensitize();
        }
        for user in self.users.iter_mut() {
            user.desensitize();
        }
    }
}

async fn preload(mut teams: Vec<Team>) -> Result<Vec<Team>, DbErr> {
    let models = teams
        .clone()
        .into_iter()
        .map(entity::team::Model::from)
        .collect::<Vec<entity::team::Model>>();
    let users = models
        .load_many_to_many(entity::user::Entity, entity::user_team::Entity, get_db())
        .await?
        .into_iter()
        .map(|users| users.into_iter().map(User::from).collect::<Vec<User>>())
        .collect::<Vec<Vec<User>>>();

    for (i, team) in teams.iter_mut().enumerate() {
        team.users = users[i].clone();
        for user in team.users.iter_mut() {
            user.desensitize();
            if user.id == team.captain_id {
                team.captain = Some(user.clone());
            }
        }
    }

    Ok(teams)
}

pub async fn find(
    id: Option<i64>, name: Option<String>, email: Option<String>, page: Option<u64>,
    size: Option<u64>,
) -> Result<(Vec<Team>, u64), DbErr> {
    let mut sql = entity::team::Entity::find();

    if let Some(id) = id {
        sql = sql.filter(entity::team::Column::Id.eq(id));
    }

    if let Some(name) = name {
        sql = sql.filter(entity::team::Column::Name.contains(name));
    }

    if let Some(email) = email {
        sql = sql.filter(entity::team::Column::Email.eq(email));
    }

    let total = sql.clone().count(get_db()).await?;

    if let Some(page) = page {
        if let Some(size) = size {
            let offset = (page - 1) * size;
            sql = sql.offset(offset).limit(size);
        }
    }

    let teams = sql.all(get_db()).await?;
    let mut teams = teams.into_iter().map(Team::from).collect::<Vec<Team>>();

    teams = preload(teams).await?;

    Ok((teams, total))
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
        .filter(entity::user_team::Column::UserId.eq(id))
        .join(JoinType::InnerJoin, entity::user_team::Relation::Team.def())
        .into_model::<entity::team::Model>()
        .all(get_db())
        .await?;

    let mut teams = teams.into_iter().map(Team::from).collect::<Vec<Team>>();

    teams = preload(teams).await?;

    Ok(teams)
}
