use async_trait::async_trait;
use sea_orm::{FromJsonQueryResult, entity::prelude::*};
use sea_orm::ActiveValue::Set;
use serde::{Deserialize, Serialize};

use super::{challenge, game, team, user};

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "pods")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub flag: Option<String>, // The generated flag, which will be injected into the cluster.
    pub user_id: i64,
    pub team_id: Option<i64>,
    pub game_id: Option<i64>,
    pub challenge_id: Uuid,

    /// Pod phase
    ///
    /// - Pending
    /// - Running
    /// - Succeeded
    /// - Failed
    /// - Unknown
    ///
    /// https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle/#pod-phase
    pub phase: String,
    #[sea_orm(column_type = "JsonBinary")]
    pub nats: Vec<Nat>,
    pub removed_at: i64,
    pub created_at: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult, Default)]
pub struct Nat {
    pub src: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dst: Option<String>,
    pub proxy: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Challenge,
    User,
    Team,
    Game,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Challenge => Entity::belongs_to(challenge::Entity)
                .from(Column::ChallengeId)
                .to(challenge::Column::Id)
                .into(),
            Self::User => Entity::belongs_to(user::Entity)
                .from(Column::UserId)
                .to(user::Column::Id)
                .into(),
            Self::Team => Entity::belongs_to(team::Entity)
                .from(Column::TeamId)
                .to(team::Column::Id)
                .into(),
            Self::Game => Entity::belongs_to(game::Entity)
                .from(Column::GameId)
                .to(game::Column::Id)
                .into(),
        }
    }
}

impl Related<challenge::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Challenge.def()
    }
}

impl Related<user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl Related<team::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Team.def()
    }
}

impl Related<game::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Game.def()
    }
}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(mut self, _db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait, {
        let ts = chrono::Utc::now().timestamp();

        if insert {
            self.id = Set(Uuid::new_v4());
            self.created_at = Set(ts);
        }

        Ok(self)
    }
}
