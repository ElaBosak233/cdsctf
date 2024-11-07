pub mod status;

use axum::async_trait;
use sea_orm::{
    entity::prelude::*, IntoActiveModel, QueryOrder, QuerySelect, Set, TryIntoModel,
};
use serde::{Deserialize, Serialize};
pub use status::Status;

use super::{challenge, game, team, user};
use crate::database::get_db;

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "submissions")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub flag: String,
    pub status: Status,
    pub user_id: i64,
    pub team_id: Option<i64>,
    pub game_id: Option<i64>,
    pub challenge_id: i64,
    pub created_at: i64,
    pub updated_at: i64,

    #[sea_orm(default_value = 0)]
    pub pts: i64,
    #[sea_orm(default_value = 0)]
    pub rank: i64,

    #[sea_orm(ignore)]
    pub user: Option<user::Model>,
    #[sea_orm(ignore)]
    pub team: Option<team::Model>,
    #[sea_orm(ignore)]
    pub game: Option<game::Model>,
    #[sea_orm(ignore)]
    pub challenge: Option<challenge::Model>,
}

impl Model {
    pub fn desensitize(&mut self) {
        self.flag.clear();
    }
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
                .on_delete(ForeignKeyAction::Cascade)
                .into(),
            Self::User => Entity::belongs_to(user::Entity)
                .from(Column::UserId)
                .to(user::Column::Id)
                .on_delete(ForeignKeyAction::Cascade)
                .into(),
            Self::Team => Entity::belongs_to(team::Entity)
                .from(Column::TeamId)
                .to(team::Column::Id)
                .on_delete(ForeignKeyAction::Cascade)
                .into(),
            Self::Game => Entity::belongs_to(game::Entity)
                .from(Column::GameId)
                .to(game::Column::Id)
                .on_delete(ForeignKeyAction::Cascade)
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
        self.updated_at = Set(chrono::Utc::now().timestamp());
        Ok(self)
    }
}

async fn preload(
    mut submissions: Vec<Model>,
) -> Result<Vec<Model>, DbErr> {
    let users = submissions
        .load_one(user::Entity, &get_db())
        .await?;
    let challenges = submissions
        .load_one(challenge::Entity, &get_db())
        .await?;
    let teams = submissions
        .load_one(team::Entity, &get_db())
        .await?;
    let games = submissions
        .load_one(game::Entity, &get_db())
        .await?;

    for (i, submission) in submissions.iter_mut().enumerate() {
        submission.user = users[i].clone();
        if let Some(user) = submission.user.as_mut() {
            user.desensitize();
        }
        submission.challenge = challenges[i].clone();
        if let Some(challenge) = submission.challenge.as_mut() {
            challenge.desensitize();
        }
        submission.team = teams[i].clone();
        if let Some(team) = submission.team.as_mut() {
            team.desensitize();
        }
        submission.game = games[i].clone();
    }
    Ok(submissions)
}

pub async fn find(
    id: Option<i64>, user_id: Option<i64>, team_id: Option<i64>, game_id: Option<i64>,
    challenge_id: Option<i64>, status: Option<Status>, page: Option<u64>, size: Option<u64>,
) -> Result<(Vec<Model>, u64), DbErr> {
    let mut sql = Entity::find();

    if let Some(id) = id {
        sql = sql.filter(Column::Id.eq(id));
    }

    if let Some(user_id) = user_id {
        sql = sql.filter(Column::UserId.eq(user_id));
    }

    if let Some(team_id) = team_id {
        sql = sql.filter(Column::TeamId.eq(team_id));
    }

    if let Some(game_id) = game_id {
        sql = sql.filter(Column::GameId.eq(game_id));
    }

    if let Some(challenge_id) = challenge_id {
        sql = sql.filter(Column::ChallengeId.eq(challenge_id));
    }

    if let Some(status) = status {
        sql = sql.filter(Column::Status.eq(status));
    }

    let total = sql.clone().count(&get_db()).await?;

    if let Some(page) = page {
        if let Some(size) = size {
            let offset = (page - 1) * size;
            sql = sql.offset(offset).limit(size);
        }
    }

    let mut submissions = sql.all(&get_db()).await?;

    submissions = preload(submissions).await?;

    Ok((submissions, total))
}

pub async fn get_by_challenge_ids(
    challenge_ids: Vec<i64>,
) -> Result<Vec<Model>, DbErr> {
    let mut submissions = Entity::find()
        .filter(Column::ChallengeId.is_in(challenge_ids))
        .order_by_asc(Column::CreatedAt)
        .all(&get_db())
        .await?;
    submissions = preload(submissions).await?;
    Ok(submissions)
}