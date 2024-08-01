pub mod request;

use axum::async_trait;
use sea_orm::{entity::prelude::*, Iterable, JoinType, QuerySelect, Set, TryIntoModel};
use serde::{Deserialize, Serialize};

use crate::database::get_db;

use super::{game, game_team, pod, submission, user, user_team};

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "teams")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub name: String,
    pub email: Option<String>,
    pub captain_id: i64,
    pub description: Option<String>,
    pub invite_token: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,

    #[sea_orm(ignore)]
    pub users: Vec<user::Model>,
    #[sea_orm(ignore)]
    pub captain: Option<user::Model>,
}

impl Model {
    pub fn simplify(&mut self) {
        self.invite_token = None;
        if let Some(captain) = self.captain.as_mut() {
            captain.simplify();
        }
        for user in self.users.iter_mut() {
            user.simplify();
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
        C: ConnectionTrait,
    {
        self.updated_at = Set(chrono::Utc::now().timestamp());
        Ok(self)
    }
}

async fn preload(
    mut teams: Vec<crate::model::team::Model>,
) -> Result<Vec<crate::model::team::Model>, DbErr> {
    let users = teams
        .load_many_to_many(
            crate::model::user::Entity,
            crate::model::user_team::Entity,
            &get_db().await,
        )
        .await?;

    for (i, team) in teams.iter_mut().enumerate() {
        team.users = users[i].clone();
        for user in team.users.iter_mut() {
            user.simplify();
            if user.id == team.captain_id {
                team.captain = Some(user.clone());
            }
        }
    }

    return Ok(teams);
}

pub async fn find(
    id: Option<i64>, name: Option<String>, email: Option<String>, page: Option<u64>,
    size: Option<u64>,
) -> Result<(Vec<crate::model::team::Model>, u64), DbErr> {
    let mut query = crate::model::team::Entity::find();

    if let Some(id) = id {
        query = query.filter(crate::model::team::Column::Id.eq(id));
    }

    if let Some(name) = name {
        query = query.filter(crate::model::team::Column::Name.contains(name));
    }

    if let Some(email) = email {
        query = query.filter(crate::model::team::Column::Email.eq(email));
    }

    let total = query.clone().count(&get_db().await).await?;

    if let Some(page) = page {
        if let Some(size) = size {
            let offset = (page - 1) * size;
            query = query.offset(offset).limit(size);
        }
    }

    let mut teams = query.all(&get_db().await).await?;

    teams = preload(teams).await?;

    return Ok((teams, total));
}

pub async fn find_by_ids(ids: Vec<i64>) -> Result<Vec<crate::model::team::Model>, DbErr> {
    let mut teams = crate::model::team::Entity::find()
        .filter(crate::model::team::Column::Id.is_in(ids))
        .all(&get_db().await)
        .await?;

    teams = preload(teams).await?;

    return Ok(teams);
}

pub async fn find_by_user_id(id: i64) -> Result<Vec<crate::model::team::Model>, DbErr> {
    let mut teams = crate::model::user_team::Entity::find()
        .select_only()
        .columns(crate::model::team::Column::iter())
        .filter(crate::model::user_team::Column::UserId.eq(id))
        .join(
            JoinType::InnerJoin,
            crate::model::user_team::Relation::Team.def(),
        )
        .into_model::<crate::model::team::Model>()
        .all(&get_db().await)
        .await
        .unwrap();

    teams = preload(teams).await?;

    return Ok(teams);
}

pub async fn create(
    team: crate::model::team::ActiveModel,
) -> Result<crate::model::team::Model, DbErr> {
    return team.insert(&get_db().await).await?.try_into_model();
}

pub async fn update(
    team: crate::model::team::ActiveModel,
) -> Result<crate::model::team::Model, DbErr> {
    return team.update(&get_db().await).await?.try_into_model();
}

pub async fn delete(id: i64) -> Result<(), DbErr> {
    let result = crate::model::team::Entity::delete_by_id(id)
        .exec(&get_db().await)
        .await?;
    return Ok(if result.rows_affected == 0 {
        return Err(DbErr::RecordNotFound(format!(
            "Team with id {} not found",
            id
        )));
    });
}
