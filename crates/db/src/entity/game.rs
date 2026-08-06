//! SeaORM `game` entity — maps the `game` table and its relations.

use async_trait::async_trait;
use sea_orm::{FromJsonQueryResult, Set, entity::prelude::*};
use serde::{Deserialize, Serialize};

#[sea_orm::model]
#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "games")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub title: String,
    #[sea_orm(column_type = "Text")]
    pub sketch: Option<String>,
    #[sea_orm(column_type = "Text")]
    pub description: Option<String>,
    pub enabled: bool,
    pub public: bool,
    #[sea_orm(default_value = 3)]
    pub member_limit_min: i64,
    #[sea_orm(default_value = 3)]
    pub member_limit_max: i64,
    #[sea_orm(default_value = false)]
    pub writeup_required: bool,
    #[sea_orm(column_type = "JsonBinary")]
    pub timeslots: Vec<Timeslot>,
    pub started_at: i64,
    pub frozen_at: i64,
    pub ended_at: i64,
    pub icon_hash: Option<String>,
    pub poster_hash: Option<String>,
    pub created_at: i64,
    #[sea_orm(has_many)]
    pub submissions: HasMany<super::submission::Entity>,
    #[sea_orm(has_many)]
    pub teams: HasMany<super::team::Entity>,
    #[sea_orm(has_many, via = "game_challenge")]
    pub challenges: HasMany<super::challenge::Entity>,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    FromJsonQueryResult,
    utoipa::ToSchema,
)]
pub struct Timeslot {
    pub label: String,
    pub started_at: i64,
    pub ended_at: i64,
}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
    /// SeaORM lifecycle hook executed before insert/update.
    async fn before_save<C>(mut self, _db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait, {
        let ts = time::OffsetDateTime::now_utc().unix_timestamp();

        if insert {
            self.created_at = Set(ts);
        }

        Ok(self)
    }
}
