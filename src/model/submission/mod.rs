pub mod request;
pub mod response;
pub mod status;

use axum::async_trait;
use sea_orm::{entity::prelude::*, Condition, QuerySelect, Set};
use serde::{Deserialize, Serialize};

use crate::database::get_db;

use super::{challenge, game, team, user};
pub use status::Status;

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
    pub fn simplify(&mut self) {
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
        C: ConnectionTrait,
    {
        self.updated_at = Set(chrono::Utc::now().timestamp());
        Ok(self)
    }
}

async fn preload(
    mut submissions: Vec<crate::model::submission::Model>,
) -> Result<Vec<crate::model::submission::Model>, DbErr> {
    let users = submissions
        .load_one(crate::model::user::Entity, &get_db())
        .await?;
    let challenges = submissions
        .load_one(crate::model::challenge::Entity, &get_db())
        .await?;
    let teams = submissions
        .load_one(crate::model::team::Entity, &get_db())
        .await?;
    let games = submissions
        .load_one(crate::model::game::Entity, &get_db())
        .await?;

    for (i, submission) in submissions.iter_mut().enumerate() {
        submission.user = users[i].clone();
        if let Some(user) = submission.user.as_mut() {
            user.simplify();
        }
        submission.challenge = challenges[i].clone();
        if let Some(challenge) = submission.challenge.as_mut() {
            challenge.simplify();
        }
        submission.team = teams[i].clone();
        if let Some(team) = submission.team.as_mut() {
            team.simplify();
        }
        submission.game = games[i].clone();
        // if let Some(game) = submission.game.as_mut() {
        //     game.simplify();
        // }
    }
    return Ok(submissions);
}

pub async fn find(
    id: Option<i64>, user_id: Option<i64>, team_id: Option<i64>, game_id: Option<i64>,
    challenge_id: Option<i64>, status: Option<Status>, page: Option<u64>, size: Option<u64>,
) -> Result<(Vec<crate::model::submission::Model>, u64), DbErr> {
    let mut query = crate::model::submission::Entity::find();

    if let Some(id) = id {
        query = query.filter(crate::model::submission::Column::Id.eq(id));
    }

    if let Some(user_id) = user_id {
        query = query.filter(crate::model::submission::Column::UserId.eq(user_id));
    }

    if let Some(team_id) = team_id {
        query = query.filter(crate::model::submission::Column::TeamId.eq(team_id));
    }

    if let Some(game_id) = game_id {
        query = query.filter(crate::model::submission::Column::GameId.eq(game_id));
    }

    if let Some(challenge_id) = challenge_id {
        query = query.filter(crate::model::submission::Column::ChallengeId.eq(challenge_id));
    }

    if let Some(status) = status {
        query = query.filter(crate::model::submission::Column::Status.eq(status));
    }

    let total = query.clone().count(&get_db()).await?;

    if let Some(page) = page {
        if let Some(size) = size {
            let offset = (page - 1) * size;
            query = query.offset(offset).limit(size);
        }
    }

    let mut submissions = query.all(&get_db()).await?;

    submissions = preload(submissions).await?;

    return Ok((submissions, total));
}

pub async fn find_by_challenge_ids(
    challenge_ids: Vec<i64>,
) -> Result<Vec<crate::model::submission::Model>, DbErr> {
    let mut submissions = crate::model::submission::Entity::find()
        .filter(crate::model::submission::Column::ChallengeId.is_in(challenge_ids))
        .all(&get_db())
        .await?;
    submissions = preload(submissions).await?;
    return Ok(submissions);
}

pub async fn get_game_submission_model(
    game_id: i64, status: Option<Status>,
) -> Result<Vec<crate::model::submission::response::GameSubmissionModel>, DbErr> {
    let mut submissions = crate::model::submission::Entity::find()
        .filter(
            Condition::all()
                .add(crate::model::submission::Column::GameId.eq(game_id))
                .add(status.map_or(Condition::all(), |status| {
                    Condition::all().add(crate::model::submission::Column::Status.eq(status))
                })),
        )
        .all(&get_db())
        .await?;

    submissions = preload(submissions).await?;

    let mut submissions = submissions
        .into_iter()
        .map(|submission| crate::model::submission::response::GameSubmissionModel::from(submission))
        .collect::<Vec<crate::model::submission::response::GameSubmissionModel>>();

    let game_challenges = crate::model::game_challenge::Entity::find()
        .filter(Condition::all().add(crate::model::game_challenge::Column::GameId.eq(game_id)))
        .all(&get_db())
        .await?;

    for game_challenge in game_challenges {
        let mut submissions = submissions
            .iter_mut()
            .filter(|submission| {
                submission.challenge_id == game_challenge.challenge_id
                    && submission.status == Status::Correct
            })
            .collect::<Vec<&mut crate::model::submission::response::GameSubmissionModel>>();
        submissions.sort_by_key(|submission| submission.created_at);

        let base_points = crate::util::math::curve(
            game_challenge.max_pts,
            game_challenge.min_pts,
            game_challenge.difficulty,
            submissions.len() as i64,
        );

        for (i, submission) in submissions.iter_mut().enumerate() {
            submission.rank = i as i64 + 1;
            let bonus_multiplier = match submission.rank {
                1 => 100 + game_challenge.first_blood_reward_ratio,
                2 => 100 + game_challenge.second_blood_reward_ratio,
                3 => 100 + game_challenge.third_blood_reward_ratio,
                _ => 100,
            };
            submission.pts = base_points * bonus_multiplier / 100;
        }
    }

    return Ok(submissions);
}
