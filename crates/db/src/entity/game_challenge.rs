use async_trait::async_trait;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use crate::{entity, get_db};
use crate::traits::EagerLoading;
use crate::transfer::{Challenge, GameChallenge};
use super::{challenge, game, user};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "game_challenges")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub game_id: i64,
    #[sea_orm(primary_key)]
    pub challenge_id: Uuid,
    pub contact_id: Option<i64>,
    #[sea_orm(default_value = 1)]
    pub difficulty: i64,
    #[sea_orm(default_value = 2000)]
    pub max_pts: i64,
    #[sea_orm(default_value = 200)]
    pub min_pts: i64,
    pub bonus_ratios: Vec<i64>,
    #[sea_orm(default_value = false)]
    pub is_enabled: bool,
    pub frozen_at: Option<i64>,

    #[sea_orm(default_value = 0)]
    pub pts: i64,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Game,
    Challenge,
    User,
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
            Self::User => Entity::belongs_to(user::Entity)
                .from(Column::ContactId)
                .to(user::Column::Id)
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

impl Related<user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {}

#[async_trait]
impl EagerLoading<Vec<GameChallenge>> for Vec<Model> {
    async fn eager_load<C>(self, db: &C) -> Result<Vec<GameChallenge>, DbErr>
    where C: ConnectionTrait{
        let challenges = self
            .load_one(challenge::Entity, db)
            .await?
            .into_iter()
            .map(|c| c.map(Challenge::from))
            .collect::<Vec<Option<Challenge>>>();

        let mut game_challenges = self
            .into_iter()
            .map(GameChallenge::from)
            .collect::<Vec<GameChallenge>>();

        for (i, game_challenge) in game_challenges.iter_mut().enumerate() {
            game_challenge.challenge = challenges[i].clone();
        }

        Ok(game_challenges)
    }
}