use sea_orm::{Condition, QueryOrder, entity::prelude::*};
use serde::{Deserialize, Serialize};

use super::{Challenge, Game, Team, User};
use crate::{entity, entity::submission::Status, get_db};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Submission {
    pub id: i64,
    pub content: String,
    pub status: Status,
    pub user_id: i64,
    pub team_id: Option<i64>,
    pub game_id: Option<i64>,
    pub challenge_id: Uuid,
    pub created_at: i64,
    pub updated_at: i64,

    pub pts: i64,
    pub rank: i64,

    pub user: Option<User>,
    pub game: Option<Game>,
    pub team: Option<Team>,
    pub challenge: Option<Challenge>,
}

impl Submission {
    pub fn desensitize(&self) -> Self {
        Self {
            content: "".to_owned(),
            ..self.to_owned()
        }
    }
}

impl From<entity::submission::Model> for Submission {
    fn from(model: entity::submission::Model) -> Self {
        Self {
            id: model.id,
            content: model.content,
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
            content: submission.content,
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

pub async fn preload(mut submissions: Vec<Submission>) -> Result<Vec<Submission>, DbErr> {
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
            *challenge = challenge.desensitize();
        }
        submission.team = teams[i].clone();
        submission.game = games[i].clone();
    }
    Ok(submissions)
}

pub async fn get_by_challenge_ids(challenge_ids: Vec<Uuid>) -> Result<Vec<Submission>, DbErr> {
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

pub async fn get_by_game_id_and_team_ids(
    game_id: i64,
    team_ids: Vec<i64>,
    status: Option<Status>,
) -> Result<Vec<Submission>, DbErr> {
    let mut sql = entity::submission::Entity::find().filter(
        Condition::all()
            .add(entity::submission::Column::GameId.eq(game_id))
            .add(entity::submission::Column::TeamId.is_in(team_ids)),
    );

    if let Some(status) = status {
        sql = sql.filter(entity::submission::Column::Status.eq(status));
    }

    let submissions = sql.all(get_db()).await?;

    let mut submissions = submissions
        .into_iter()
        .map(Submission::from)
        .collect::<Vec<Submission>>();

    submissions = preload(submissions).await?;

    Ok(submissions)
}
