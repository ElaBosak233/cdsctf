//! SeaORM `team_user` entity — maps the `team_user` table and its relations.

use async_trait::async_trait;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "team_users")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub team_id: i64,
    #[sea_orm(primary_key, auto_increment = false)]
    pub user_id: i64,
    #[sea_orm(belongs_to, from = "team_id", to = "id", on_delete = "Cascade")]
    pub team: BelongsTo<super::team::Entity>,
    #[sea_orm(belongs_to, from = "user_id", to = "id", on_delete = "Cascade")]
    pub user: BelongsTo<super::user::Entity>,
}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {}
