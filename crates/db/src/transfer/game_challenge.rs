use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use super::Challenge;
use crate::{entity, get_db};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GameChallenge {
    pub game_id: i64,
    pub challenge_id: Uuid,
    pub contact_id: Option<i64>,
    pub difficulty: i64,
    pub bonus_ratios: Vec<i64>,
    pub max_pts: i64,
    pub min_pts: i64,
    pub pts: i64,
    pub is_enabled: bool,
    pub frozen_at: i64,
    pub challenge: Option<Challenge>,
}

impl From<entity::game_challenge::Model> for GameChallenge {
    fn from(entity: entity::game_challenge::Model) -> Self {
        Self {
            game_id: entity.game_id,
            challenge_id: entity.challenge_id,
            contact_id: entity.contact_id,
            difficulty: entity.difficulty,
            is_enabled: entity.is_enabled,
            bonus_ratios: entity.bonus_ratios,
            max_pts: entity.max_pts,
            min_pts: entity.min_pts,
            pts: entity.pts,
            frozen_at: entity.frozen_at,
            challenge: None,
        }
    }
}

pub async fn preload(
    models: Vec<entity::game_challenge::Model>,
) -> Result<Vec<GameChallenge>, DbErr> {
    let challenges = models
        .load_one(entity::challenge::Entity, get_db())
        .await?
        .into_iter()
        .map(|c| c.map(Challenge::from))
        .collect::<Vec<Option<Challenge>>>();

    let mut game_challenges = models
        .into_iter()
        .map(GameChallenge::from)
        .collect::<Vec<GameChallenge>>();

    for (i, game_challenge) in game_challenges.iter_mut().enumerate() {
        game_challenge.challenge = challenges[i].clone();
    }

    Ok(game_challenges)
}

pub async fn find(
    game_id: Option<i64>, challenge_id: Option<i64>, category: Option<i32>,
    is_enabled: Option<bool>,
) -> Result<(Vec<GameChallenge>, u64), DbErr> {
    let mut sql = entity::game_challenge::Entity::find()
        .inner_join(entity::challenge::Entity)
        .inner_join(entity::game::Entity);

    if let Some(game_id) = game_id {
        sql = sql.filter(entity::game_challenge::Column::GameId.eq(game_id));
    }

    if let Some(challenge_id) = challenge_id {
        sql = sql.filter(entity::game_challenge::Column::ChallengeId.eq(challenge_id));
    }

    if let Some(is_enabled) = is_enabled {
        sql = sql.filter(entity::game_challenge::Column::IsEnabled.eq(is_enabled));
    }

    if let Some(category) = category {
        sql = sql.filter(entity::challenge::Column::Category.eq(category));
    }

    let total = sql.clone().count(get_db()).await?;

    let models = sql.all(get_db()).await?;

    let game_challenges = preload(models).await?;

    Ok((game_challenges, total))
}
