use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use super::{Challenge, Team, User};
use crate::{entity, entity::pod::Nat, get_db};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Pod {
    pub id: Uuid,
    pub flag: Option<String>,
    pub user_id: i64,
    pub team_id: Option<i64>,
    pub game_id: Option<i64>,
    pub challenge_id: Uuid,
    pub phase: String,
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
    }
}

impl From<entity::pod::Model> for Pod {
    fn from(entity: entity::pod::Model) -> Self {
        Self {
            id: entity.id,
            flag: entity.flag,
            user_id: entity.user_id,
            team_id: entity.team_id,
            game_id: entity.game_id,
            challenge_id: entity.challenge_id,
            phase: entity.phase,
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
            flag: pod.flag,
            user_id: pod.user_id,
            team_id: pod.team_id,
            game_id: pod.game_id,
            challenge_id: pod.challenge_id,
            phase: pod.phase,
            nats: pod.nats,
            removed_at: pod.removed_at,
            created_at: pod.created_at,
        }
    }
}

pub async fn preload(mut pods: Vec<Pod>) -> Result<Vec<Pod>, DbErr> {
    let models = pods
        .clone()
        .into_iter()
        .map(entity::pod::Model::from)
        .collect::<Vec<entity::pod::Model>>();

    let users = models
        .load_one(entity::user::Entity, get_db())
        .await?
        .into_iter()
        .map(|u| u.map(User::from))
        .collect::<Vec<Option<User>>>();

    let teams = models
        .load_one(entity::team::Entity, get_db())
        .await?
        .into_iter()
        .map(|t| t.map(Team::from))
        .collect::<Vec<Option<Team>>>();

    let challenges = models
        .load_one(entity::challenge::Entity, get_db())
        .await?
        .into_iter()
        .map(|c| c.map(Challenge::from))
        .collect::<Vec<Option<Challenge>>>();

    for (i, pod) in pods.iter_mut().enumerate() {
        pod.user = users[i].clone();
        pod.team = teams[i].clone();
        pod.challenge = challenges[i].clone();
    }

    Ok(pods)
}
