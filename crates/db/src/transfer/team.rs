use std::str::FromStr;

use sea_orm::{Iterable, JoinType, Order, QueryOrder, QuerySelect, entity::prelude::*};
use serde::{Deserialize, Serialize};

use super::User;
use crate::{entity, get_db};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Team {
    pub id: i64,
    pub name: String,
    pub email: Option<String>,
    pub slogan: Option<String>,
    pub description: Option<String>,
    pub invite_token: Option<String>,
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
            invite_token: model.invite_token,
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
            invite_token: team.invite_token,
            is_locked: team.is_locked,
            deleted_at: team.deleted_at,
            created_at: team.created_at,
            updated_at: team.updated_at,
        }
    }
}

impl Team {
    pub fn desensitize(&mut self) {
        self.invite_token = None;
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
        .load_many_to_many(entity::user::Entity, entity::user_team::Entity, get_db())
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

/// Find existing teams with given params.
///
/// This function will automatically exclude teams which `is_deleted` is `true`.
pub async fn find(
    id: Option<i64>, name: Option<String>, email: Option<String>, sorts: Option<String>,
    page: Option<u64>, size: Option<u64>,
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

    // Exclude teams which has been deleted.
    sql = sql.filter(entity::team::Column::DeletedAt.is_null());

    let total = sql.clone().count(get_db()).await?;

    // Sort according to the `sorts` parameter.
    if let Some(sorts) = sorts {
        let sorts = sorts.split(",").collect::<Vec<&str>>();
        for sort in sorts {
            let col = match crate::entity::team::Column::from_str(sort.replace("-", "").as_str()) {
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

    // Paginate according to the `page` and `size` parameters.
    if let (Some(page), Some(size)) = (page, size) {
        let offset = (page - 1) * size;
        sql = sql.offset(offset).limit(size);
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
