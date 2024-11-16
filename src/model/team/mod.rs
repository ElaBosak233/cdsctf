use axum::async_trait;
use sea_orm::{entity::prelude::*, Iterable, JoinType, QuerySelect, Set};
use serde::{Deserialize, Serialize};

use super::{game, game_team, pod, submission, user, user_team};
use crate::db::get_db;

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "teams")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub name: String,
    pub email: Option<String>,
    pub captain_id: i64,
    pub slogan: Option<String>,
    pub invite_token: Option<String>,
    #[sea_orm(default_value = false)]
    pub is_deleted: bool,
    pub created_at: i64,
    pub updated_at: i64,

    #[sea_orm(ignore)]
    pub users: Vec<user::Model>,
    #[sea_orm(ignore)]
    pub captain: Option<user::Model>,
}

impl Model {
    pub fn desensitize(&mut self) {
        self.invite_token = None;
        if let Some(captain) = self.captain.as_mut() {
            captain.desensitize();
        }
        for user in self.users.iter_mut() {
            user.desensitize();
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Submission,
    Pod,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Submission => Entity::has_many(submission::Entity).into(),
            Self::Pod => Entity::has_many(pod::Entity).into(),
        }
    }
}

impl Related<user::Entity> for Entity {
    fn to() -> RelationDef {
        user_team::Relation::User.def()
    }

    fn via() -> Option<RelationDef> {
        Some(user_team::Relation::Team.def().rev())
    }
}

impl Related<game::Entity> for Entity {
    fn to() -> RelationDef {
        game_team::Relation::Game.def()
    }

    fn via() -> Option<RelationDef> {
        Some(game_team::Relation::Team.def().rev())
    }
}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        Self {
            created_at: Set(chrono::Utc::now().timestamp()),
            updated_at: Set(chrono::Utc::now().timestamp()),
            ..ActiveModelTrait::default()
        }
    }

    async fn before_save<C>(mut self, _db: &C, _insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait, {
        Ok(Self {
            updated_at: Set(chrono::Utc::now().timestamp()),
            ..self
        })
    }
}

async fn preload(mut teams: Vec<Model>) -> Result<Vec<Model>, DbErr> {
    let users = teams
        .load_many_to_many(user::Entity, user_team::Entity, &get_db())
        .await?;

    for (i, team) in teams.iter_mut().enumerate() {
        team.users = users[i].clone();
        for user in team.users.iter_mut() {
            user.desensitize();
            if user.id == team.captain_id {
                team.captain = Some(user.clone());
            }
        }
    }

    Ok(teams)
}

pub async fn find(
    id: Option<i64>, name: Option<String>, email: Option<String>, page: Option<u64>,
    size: Option<u64>,
) -> Result<(Vec<Model>, u64), DbErr> {
    let mut sql = Entity::find();

    if let Some(id) = id {
        sql = sql.filter(Column::Id.eq(id));
    }

    if let Some(name) = name {
        sql = sql.filter(Column::Name.contains(name));
    }

    if let Some(email) = email {
        sql = sql.filter(Column::Email.eq(email));
    }

    let total = sql.clone().count(&get_db()).await?;

    if let Some(page) = page {
        if let Some(size) = size {
            let offset = (page - 1) * size;
            sql = sql.offset(offset).limit(size);
        }
    }

    let mut teams = sql.all(&get_db()).await?;

    teams = preload(teams).await?;

    Ok((teams, total))
}

pub async fn find_by_ids(ids: Vec<i64>) -> Result<Vec<Model>, DbErr> {
    let mut teams = Entity::find()
        .filter(Column::Id.is_in(ids))
        .all(&get_db())
        .await?;

    teams = preload(teams).await?;

    Ok(teams)
}

pub async fn find_by_user_id(id: i64) -> Result<Vec<Model>, DbErr> {
    let mut teams = Entity::find()
        .select_only()
        .columns(Column::iter())
        .filter(user_team::Column::UserId.eq(id))
        .join(JoinType::InnerJoin, user_team::Relation::Team.def())
        .into_model::<Model>()
        .all(&get_db())
        .await?;

    teams = preload(teams).await?;

    Ok(teams)
}
