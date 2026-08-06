//! SeaORM `user` entity — maps the `user` table and its relations.

use async_trait::async_trait;
use sea_orm::{ExprTrait, QuerySelect, Set, entity::prelude::*, sea_query::Query};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::email;

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
    pub group: Group,
    pub hashed_password: String,
    pub avatar_hash: Option<String>,
    pub deleted_at: Option<i64>,
    pub created_at: i64,
    pub updated_at: i64,
    #[sea_orm(has_one)]
    pub email: HasOne<super::email::Entity>,
    #[sea_orm(has_many)]
    pub submissions: HasMany<super::submission::Entity>,
    #[sea_orm(has_many)]
    pub notes: HasMany<super::note::Entity>,
    #[sea_orm(has_many, via = "team_user")]
    pub teams: HasMany<super::team::Entity>,
}

#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Serialize_repr,
    Deserialize_repr,
    EnumIter,
    DeriveActiveEnum,
    utoipa::ToSchema,
)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
#[repr(i32)]
pub enum Group {
    #[default]
    Guest  = 0,
    Banned = 1,
    User   = 2,
    Admin  = 3,
}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
    /// SeaORM lifecycle hook executed before insert/update.
    async fn before_save<C>(mut self, _db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait, {
        let ts = time::OffsetDateTime::now_utc().unix_timestamp();

        self.updated_at = Set(ts);

        if insert {
            self.created_at = Set(ts);
        }

        Ok(self)
    }
}

impl Entity {
    /// Begins the canonical query with standard joins and projections.
    pub fn base_find() -> Select<Entity> {
        Self::find().column_as(
            Expr::exists(
                Query::select()
                    .expr(Expr::val(1))
                    .from(email::Entity.table_name())
                    .and_where(
                        Expr::col((email::Entity.table_name(), email::Column::UserId))
                            .eq(Expr::col((Entity.table_name(), Column::Id))),
                    )
                    .and_where(
                        Expr::col((email::Entity.table_name(), email::Column::Verified)).eq(true),
                    )
                    .to_owned(),
            ),
            "verified",
        )
    }
}
