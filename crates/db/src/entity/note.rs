//! SeaORM `note` entity — maps the `note` table and its relations.

use async_trait::async_trait;
use sea_orm::{EnumIter, QuerySelect, Set, entity::prelude::*};
use serde::{Deserialize, Serialize};

use super::{challenge, user};

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "notes")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub content: String,
    pub public: bool,
    pub user_id: i64,
    pub challenge_id: i64,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Challenge,
    User,
}

impl RelationTrait for Relation {
    /// Returns the [`RelationDef`] for this relation variant.
    fn def(&self) -> RelationDef {
        match self {
            Self::Challenge => Entity::belongs_to(challenge::Entity)
                .from(Column::ChallengeId)
                .to(challenge::Column::Id)
                .on_delete(ForeignKeyAction::Cascade)
                .into(),
            Self::User => Entity::belongs_to(user::Entity)
                .from(Column::UserId)
                .to(user::Column::Id)
                .on_delete(ForeignKeyAction::Cascade)
                .into(),
        }
    }
}

impl Related<challenge::Entity> for Entity {
    /// Returns the [`RelationDef`] linking to the related [`Entity`].
    fn to() -> RelationDef {
        Relation::Challenge.def()
    }
}

impl Related<user::Entity> for Entity {
    /// Returns the [`RelationDef`] linking to the related [`Entity`].
    fn to() -> RelationDef {
        Relation::User.def()
    }
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
    pub fn base_find() -> Select<Self> {
        Self::find()
            .inner_join(user::Entity)
            .inner_join(challenge::Entity)
            .column_as(user::Column::Name, "user_name")
            .column_as(user::Column::HasAvatar, "user_has_avatar")
            .column_as(challenge::Column::Title, "challenge_title")
            .column_as(challenge::Column::Category, "challenge_category")
    }
}
