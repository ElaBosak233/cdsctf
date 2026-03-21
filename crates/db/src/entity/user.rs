//! SeaORM `user` entity — maps the `user` table and its relations.

use async_trait::async_trait;
use sea_orm::{QuerySelect, Set, entity::prelude::*, sea_query::Query};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::{email, note, submission, team, team_user};

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
    pub has_avatar: bool,
    pub deleted_at: Option<i64>,
    pub created_at: i64,
    pub updated_at: i64,
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

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Email,
    Submission,
    Note,
}

impl RelationTrait for Relation {
    /// Returns the [`RelationDef`] for this relation variant.
    fn def(&self) -> RelationDef {
        match self {
            Self::Email => Entity::has_one(email::Entity).into(),
            Self::Submission => Entity::has_many(submission::Entity).into(),
            Self::Note => Entity::has_many(note::Entity).into(),
        }
    }
}

impl Related<team::Entity> for Entity {
    /// Returns the [`RelationDef`] linking to the related [`Entity`].
    fn to() -> RelationDef {
        team_user::Relation::Team.def()
    }

    /// Declares a `via` join path for related entities.
    fn via() -> Option<RelationDef> {
        Some(team_user::Relation::User.def().rev())
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
