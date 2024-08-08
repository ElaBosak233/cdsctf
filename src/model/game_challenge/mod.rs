use axum::async_trait;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use crate::database::get_db;

use super::{challenge, game};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "game_challenges")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub game_id: i64,
    #[sea_orm(primary_key)]
    pub challenge_id: i64,
    #[sea_orm(default_value = 1)]
    pub difficulty: i64,
    #[sea_orm(default_value = false)]
    pub is_enabled: bool,
    #[sea_orm(default_value = 5)]
    pub first_blood_reward_ratio: i64,
    #[sea_orm(default_value = 3)]
    pub second_blood_reward_ratio: i64,
    #[sea_orm(default_value = 1)]
    pub third_blood_reward_ratio: i64,
    #[sea_orm(default_value = 2000)]
    pub max_pts: i64,
    #[sea_orm(default_value = 200)]
    pub min_pts: i64,

    #[sea_orm(ignore)]
    pub challenge: Option<challenge::Model>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Game,
    Challenge,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Game => Entity::belongs_to(game::Entity)
                .from(Column::GameId)
                .to(game::Column::Id)
                .on_delete(ForeignKeyAction::Cascade)
                .into(),
            Self::Challenge => Entity::belongs_to(challenge::Entity)
                .from(Column::ChallengeId)
                .to(challenge::Column::Id)
                .on_delete(ForeignKeyAction::Cascade)
                .into(),
        }
    }
}

impl Related<challenge::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Challenge.def()
    }
}

impl Related<game::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Game.def()
    }
}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {}

async fn preload(
    mut game_challenges: Vec<crate::model::game_challenge::Model>,
) -> Result<Vec<crate::model::game_challenge::Model>, DbErr> {
    let challenges = game_challenges
        .load_one(crate::model::challenge::Entity, &get_db())
        .await?;

    for (i, game_challenge) in game_challenges.iter_mut().enumerate() {
        game_challenge.challenge = challenges[i].clone();
    }

    return Ok(game_challenges);
}

pub async fn find(
    game_id: Option<i64>, challenge_id: Option<i64>, is_enabled: Option<bool>,
) -> Result<(Vec<crate::model::game_challenge::Model>, u64), DbErr> {
    let mut query = crate::model::game_challenge::Entity::find();

    if let Some(game_id) = game_id {
        query = query.filter(crate::model::game_challenge::Column::GameId.eq(game_id));
    }

    if let Some(challenge_id) = challenge_id {
        query = query.filter(crate::model::game_challenge::Column::ChallengeId.eq(challenge_id));
    }

    if let Some(is_enabled) = is_enabled {
        query = query.filter(crate::model::game_challenge::Column::IsEnabled.eq(is_enabled));
    }

    let total = query.clone().count(&get_db()).await?;

    let mut game_challenges = query.all(&get_db()).await?;

    game_challenges = preload(game_challenges).await?;

    Ok((game_challenges, total))
}
