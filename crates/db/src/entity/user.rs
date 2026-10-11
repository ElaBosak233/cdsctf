//! SeaORM `user` entity — maps the `user` table and its relations.

use async_trait::async_trait;
use sea_orm::{Set, entity::prelude::*};
use serde::{Deserialize, Serialize};

use crate::dto::user::Permission;

#[sea_orm::model]
#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub name: String,
    #[sea_orm(unique)]
    pub username: String,
    #[sea_orm(column_type = "Text")]
    pub description: Option<String>,
    /// Whether the account is prohibited from authenticating.
    #[sea_orm(default_value = false)]
    pub banned: bool,
    /// Coarse-grained capabilities assigned to the account.
    pub permissions: Vec<Permission>,
    pub hashed_password: String,
    pub avatar_hash: Option<String>,
    #[serde(with = "crate::time_format::option")]
    pub deleted_at: Option<time::OffsetDateTime>,
    #[serde(with = "crate::time_format")]
    pub created_at: time::OffsetDateTime,
    #[serde(with = "crate::time_format")]
    pub updated_at: time::OffsetDateTime,
    #[sea_orm(has_one)]
    pub email: HasOne<super::email::Entity>,
    #[sea_orm(has_many)]
    pub submissions: HasMany<super::submission::Entity>,
    #[sea_orm(has_many)]
    pub notes: HasMany<super::note::Entity>,
    #[sea_orm(has_many, via = "team_user")]
    pub teams: HasMany<super::team::Entity>,
}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
    /// SeaORM lifecycle hook executed before insert/update.
    async fn before_save<C>(mut self, _db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait, {
        let ts = time::OffsetDateTime::now_utc();

        self.updated_at = Set(ts);

        if insert {
            self.created_at = Set(ts);
        }

        Ok(self)
    }
}
