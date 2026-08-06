//! SeaORM `game_challenge` entity — maps the `game_challenge` table and its
//! relations.

use async_trait::async_trait;
use sea_orm::{QuerySelect, entity::prelude::*};
use serde::{Deserialize, Serialize};

use super::{challenge, game};

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "game_challenges")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub game_id: i64,
    #[sea_orm(primary_key, auto_increment = false)]
    pub challenge_id: i64,
    pub difficulty: i64,
    pub max_pts: i64,
    pub min_pts: i64,
    pub bonus_ratios: Vec<i64>,
    #[sea_orm(default_value = false)]
    pub enabled: bool,
    pub frozen_at: Option<i64>,

    #[sea_orm(default_value = 0)]
    pub pts: i64,
    #[sea_orm(belongs_to, from = "game_id", to = "id", on_delete = "Cascade")]
    pub game: BelongsTo<super::game::Entity>,
    #[sea_orm(belongs_to, from = "challenge_id", to = "id", on_delete = "Cascade")]
    pub challenge: BelongsTo<super::challenge::Entity>,
}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {}

impl Entity {
    /// Begins the canonical query with standard joins and projections.
    pub fn base_find() -> Select<Self> {
        Self::find()
            .inner_join(game::Entity)
            .inner_join(challenge::Entity)
            .column_as(challenge::Column::Title, "challenge_title")
            .column_as(challenge::Column::Category, "challenge_category")
            .column_as(challenge::Column::Tags, "challenge_tags")
    }
}
