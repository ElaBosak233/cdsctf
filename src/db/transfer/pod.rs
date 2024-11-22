use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use super::{Challenge, Team, User};
use crate::db::{entity, entity::pod::Nat, get_db};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Pod {
    pub id: i64,
    pub name: String,
    pub flag: Option<String>,
    pub user_id: i64,
    pub team_id: Option<i64>,
    pub game_id: Option<i64>,
    pub challenge_id: i64,
    pub nats: Vec<Nat>,
    pub removed_at: i64,
    pub created_at: i64,

    pub user: Option<User>,
    pub team: Option<Team>,
    pub challenge: Option<Challenge>,
}

impl Pod {
    pub fn desensitize(&mut self) {
        self.flag = None;
        // for nat in self.nats.iter_mut() {
        //     nat.desensitize();
        // }
    }
}

impl From<entity::pod::Model> for Pod {
    fn from(entity: entity::pod::Model) -> Self {
        Self {
            id: entity.id,
            name: entity.name,
            flag: entity.flag,
            user_id: entity.user_id,
            team_id: entity.team_id,
            game_id: entity.game_id,
            challenge_id: entity.challenge_id,
            nats: entity.nats,
            removed_at: entity.removed_at,
            created_at: entity.created_at,
            user: None,
            team: None,
            challenge: None,
        }
    }
}

impl From<Pod> for entity::pod::Model {
    fn from(pod: Pod) -> Self {
        Self {
            id: pod.id,
            name: pod.name,
            flag: pod.flag,
            user_id: pod.user_id,
            team_id: pod.team_id,
            game_id: pod.game_id,
            challenge_id: pod.challenge_id,
            nats: pod.nats,
            removed_at: pod.removed_at,
            created_at: pod.created_at,
        }
    }
}

async fn preload(mut pods: Vec<Pod>) -> Result<Vec<Pod>, DbErr> {
    let models = pods
        .clone()
        .into_iter()
        .map(|pod| entity::pod::Model::from(pod))
        .collect::<Vec<entity::pod::Model>>();

    let users = models
        .load_one(entity::user::Entity, get_db())
        .await?
        .into_iter()
        .map(|u| u.map(|user| User::from(user)))
        .collect::<Vec<Option<User>>>();

    let teams = models
        .load_one(entity::team::Entity, get_db())
        .await?
        .into_iter()
        .map(|t| t.map(|team| Team::from(team)))
        .collect::<Vec<Option<Team>>>();

    let challenges = models
        .load_one(entity::challenge::Entity, get_db())
        .await?
        .into_iter()
        .map(|c| c.map(|challenge| Challenge::from(challenge)))
        .collect::<Vec<Option<Challenge>>>();

    for (i, pod) in pods.iter_mut().enumerate() {
        pod.user = users[i].clone();
        pod.team = teams[i].clone();
        pod.challenge = challenges[i].clone();
    }

    Ok(pods)
}

pub async fn find(
    id: Option<i64>, name: Option<String>, user_id: Option<i64>, team_id: Option<i64>,
    game_id: Option<i64>, challenge_id: Option<i64>, is_available: Option<bool>,
) -> Result<(Vec<Pod>, u64), DbErr> {
    let mut sql = entity::pod::Entity::find();
    if let Some(id) = id {
        sql = sql.filter(entity::pod::Column::Id.eq(id));
    }

    if let Some(name) = name {
        sql = sql.filter(entity::pod::Column::Name.eq(name));
    }

    if let Some(user_id) = user_id {
        sql = sql.filter(entity::pod::Column::UserId.eq(user_id));
    }

    if let Some(team_id) = team_id {
        sql = sql.filter(entity::pod::Column::TeamId.eq(team_id));
    }

    if let Some(game_id) = game_id {
        sql = sql.filter(entity::pod::Column::GameId.eq(game_id));
    }

    if let Some(challenge_id) = challenge_id {
        sql = sql.filter(entity::pod::Column::ChallengeId.eq(challenge_id));
    }

    if let Some(is_available) = is_available {
        match is_available {
            true => {
                sql = sql.filter(entity::pod::Column::RemovedAt.gte(chrono::Utc::now().timestamp()))
            }
            false => {
                sql = sql.filter(entity::pod::Column::RemovedAt.lte(chrono::Utc::now().timestamp()))
            }
        }
    }

    let total = sql.clone().count(get_db()).await?;

    let pods = sql.all(get_db()).await?;
    let mut pods = pods
        .into_iter()
        .map(|pod| Pod::from(pod))
        .collect::<Vec<Pod>>();

    pods = preload(pods).await?;

    Ok((pods, total))
}
