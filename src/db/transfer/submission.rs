use sea_orm::{entity::prelude::*, QueryOrder, QuerySelect};
use serde::{Deserialize, Serialize};

use super::{Challenge, Game, Team, User};
use crate::db::{entity, entity::submission::Status, get_db};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Submission {
    pub id: i64,
    pub flag: String,
    pub status: Status,
    pub user_id: i64,
    pub team_id: Option<i64>,
    pub game_id: Option<i64>,
    pub challenge_id: i64,
    pub created_at: i64,
    pub updated_at: i64,

    pub pts: i64,
    pub rank: i64,

    pub user: Option<User>,
    pub team: Option<Team>,
    pub game: Option<Game>,
    pub challenge: Option<Challenge>,
}

impl Submission {
    pub fn desensitize(&mut self) {
        self.flag.clear();
    }
}

impl From<entity::submission::Model> for Submission {
    fn from(model: entity::submission::Model) -> Self {
        Self {
            id: model.id,
            flag: model.flag,
            status: model.status,
            user_id: model.user_id,
            team_id: model.team_id,
            game_id: model.game_id,
            challenge_id: model.challenge_id,
            created_at: model.created_at,
            updated_at: model.updated_at,
            pts: model.pts,
            rank: model.rank,
            user: None,
            team: None,
            game: None,
            challenge: None,
        }
    }
}

impl From<Submission> for entity::submission::Model {
    fn from(submission: Submission) -> Self {
        Self {
            id: submission.id,
            flag: submission.flag,
            status: submission.status,
            user_id: submission.user_id,
            team_id: submission.team_id,
            game_id: submission.game_id,
            challenge_id: submission.challenge_id,
            created_at: submission.created_at,
            updated_at: submission.updated_at,
            pts: submission.pts,
            rank: submission.rank,
        }
    }
}

async fn preload(mut submissions: Vec<Submission>) -> Result<Vec<Submission>, DbErr> {
    let models = submissions
        .clone()
        .into_iter()
        .map(entity::submission::Model::from)
        .collect::<Vec<entity::submission::Model>>();

    let users = models
        .load_one(entity::user::Entity, get_db())
        .await?
        .into_iter()
        .map(|u| u.map(User::from))
        .collect::<Vec<Option<User>>>();
    let challenges = models
        .load_one(entity::challenge::Entity, get_db())
        .await?
        .into_iter()
        .map(|c| c.map(Challenge::from))
        .collect::<Vec<Option<Challenge>>>();
    let teams = models
        .load_one(entity::team::Entity, get_db())
        .await?
        .into_iter()
        .map(|t| t.map(Team::from))
        .collect::<Vec<Option<Team>>>();
    let games = models
        .load_one(entity::game::Entity, get_db())
        .await?
        .into_iter()
        .map(|g| g.map(Game::from))
        .collect::<Vec<Option<Game>>>();

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
) -> Result<(Vec<Submission>, u64), DbErr> {
    let mut sql = entity::submission::Entity::find();

    if let Some(id) = id {
        sql = sql.filter(entity::submission::Column::Id.eq(id));
    }

    if let Some(user_id) = user_id {
        sql = sql.filter(entity::submission::Column::UserId.eq(user_id));
    }

    if let Some(team_id) = team_id {
        sql = sql.filter(entity::submission::Column::TeamId.eq(team_id));
    }

    if let Some(game_id) = game_id {
        sql = sql.filter(entity::submission::Column::GameId.eq(game_id));
    }

    if let Some(challenge_id) = challenge_id {
        sql = sql.filter(entity::submission::Column::ChallengeId.eq(challenge_id));
    }

    if let Some(status) = status {
        sql = sql.filter(entity::submission::Column::Status.eq(status));
    }

    let total = sql.clone().count(get_db()).await?;

    if let Some(page) = page {
        if let Some(size) = size {
            let offset = (page - 1) * size;
            sql = sql.offset(offset).limit(size);
        }
    }

    let submissions = sql.all(get_db()).await?;
    let mut submissions = submissions
        .into_iter()
        .map(Submission::from)
        .collect::<Vec<Submission>>();

    submissions = preload(submissions).await?;

    Ok((submissions, total))
}

pub async fn get_by_challenge_ids(challenge_ids: Vec<i64>) -> Result<Vec<Submission>, DbErr> {
    let submissions = entity::submission::Entity::find()
        .filter(entity::submission::Column::ChallengeId.is_in(challenge_ids))
        .order_by_asc(entity::submission::Column::CreatedAt)
        .all(get_db())
        .await?;

    let mut submissions = submissions
        .into_iter()
        .map(Submission::from)
        .collect::<Vec<Submission>>();
    submissions = preload(submissions).await?;
    Ok(submissions)
}
