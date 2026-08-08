//! SeaORM `email` entity — maps the `email` table and its relations.

use async_trait::async_trait;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[sea_orm::model]
#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "emails")]
pub struct Model {
    #[sea_orm(primary_key, unique)]
    pub email: String,
    pub verified: bool,
    pub user_id: i64,
    #[sea_orm(belongs_to, from = "user_id", to = "id", on_delete = "Cascade")]
    pub user: BelongsTo<super::user::Entity>,
}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {}
