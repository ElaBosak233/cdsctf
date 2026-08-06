//! SeaORM `team` entity — maps the `team` table and its relations.

use async_trait::async_trait;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "teams")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub game_id: i64,
    pub name: String,
    pub email: Option<String>,
    pub slogan: Option<String>,
    pub avatar_hash: Option<String>,
    pub has_writeup: bool,

    pub state: State,

    #[sea_orm(default_value = 0)]
    pub pts: i64,
    #[sea_orm(default_value = 0)]
    pub rank: i64,
    #[sea_orm(belongs_to, from = "game_id", to = "id", on_delete = "Cascade")]
    pub game: BelongsTo<super::game::Entity>,
    #[sea_orm(has_many)]
    pub submissions: HasMany<super::submission::Entity>,
    #[sea_orm(has_many, via = "team_user")]
    pub users: HasMany<super::user::Entity>,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    Eq,
    Serialize_repr,
    Deserialize_repr,
    EnumIter,
    DeriveActiveEnum,
    utoipa::ToSchema,
)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
#[repr(i32)]
pub enum State {
    Banned    = 0,
    #[default]
    Preparing = 1,
    Pending   = 2,
    Passed    = 3,
}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {}
