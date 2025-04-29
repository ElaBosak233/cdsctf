use async_trait::async_trait;
use sea_orm::{entity::prelude::*, QuerySelect};
use serde::{Deserialize, Serialize};

use super::{challenge, game};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "game_challenges")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub game_id: i64,
    #[sea_orm(primary_key)]
    pub challenge_id: Uuid,
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

impl Entity {
    pub fn base_find() -> Select<Self> {
        Self::find()
            .inner_join(game::Entity)
            .inner_join(challenge::Entity)
            .column_as(challenge::Column::Title, "challenge_title")
            .column_as(challenge::Column::Category, "challenge_category")
            .column_as(challenge::Column::Tags, "challenge_tags")
    }
}
