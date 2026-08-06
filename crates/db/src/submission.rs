//! Database access for `submission` — SeaORM queries, updates, and DTOs.

use std::str::FromStr;

use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityLoaderTrait, EntityTrait, Order,
    PaginatorTrait, QueryFilter, QueryOrder, QuerySelect,
};
use serde::{Deserialize, Serialize};
use tracing::info;

pub use crate::entity::submission::{ActiveModel, Status};
pub(crate) use crate::entity::submission::{Column, Entity};
use crate::{sea_orm, sea_orm::FromQueryResult, traits::DbError};

#[derive(
    Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromQueryResult, utoipa::ToSchema,
)]
pub struct Submission {
    pub id: i64,
    pub content: String,
    pub status: Status,
    pub user_id: i64,
    pub user_name: String,
    pub user_avatar_hash: Option<String>,
    pub team_id: Option<i64>,
    pub team_name: Option<String>,
    pub team_avatar_hash: Option<String>,
    pub game_id: Option<i64>,
    pub game_title: Option<String>,
    pub challenge_id: i64,
    pub challenge_title: String,
    pub challenge_category: i32,
    pub created_at: i64,

    pub pts: i64,
    pub rank: i64,
}

impl TryFrom<crate::entity::submission::ModelEx> for Submission {
    type Error = DbError;

    fn try_from(submission: crate::entity::submission::ModelEx) -> Result<Self, Self::Error> {
        let user = submission.user.as_ref().ok_or_else(|| {
            DbError::Other(anyhow::anyhow!(
                "submission {} was loaded without its user relation",
                submission.id
            ))
        })?;
        let challenge = submission.challenge.as_ref().ok_or_else(|| {
            DbError::Other(anyhow::anyhow!(
                "submission {} was loaded without its challenge relation",
                submission.id
            ))
        })?;

        let team = if submission.team_id.is_some() {
            Some(submission.team.as_ref().ok_or_else(|| {
                DbError::Other(anyhow::anyhow!(
                    "submission {} was loaded without its team relation",
                    submission.id
                ))
            })?)
        } else {
            None
        };
        let game = if submission.game_id.is_some() {
            Some(submission.game.as_ref().ok_or_else(|| {
                DbError::Other(anyhow::anyhow!(
                    "submission {} was loaded without its game relation",
                    submission.id
                ))
            })?)
        } else {
            None
        };

        Ok(Self {
            id: submission.id,
            content: submission.content,
            status: submission.status,
            user_id: submission.user_id,
            user_name: user.name.clone(),
            user_avatar_hash: user.avatar_hash.clone(),
            team_id: submission.team_id,
            team_name: team.map(|team| team.name.clone()),
            team_avatar_hash: team.and_then(|team| team.avatar_hash.clone()),
            game_id: submission.game_id,
            game_title: game.map(|game| game.title.clone()),
            challenge_id: submission.challenge_id,
            challenge_title: challenge.title.clone(),
            challenge_category: challenge.category,
            created_at: submission.created_at,
            pts: submission.pts,
            rank: submission.rank,
        })
    }
}

/// Submission fields needed to render the public scoreboard timeline.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, utoipa::ToSchema)]
pub struct SubmissionPublic {
    pub id: i64,
    pub user_id: i64,
    pub user_name: String,
    pub user_avatar_hash: Option<String>,
    pub challenge_id: i64,
    pub challenge_title: String,
    pub pts: i64,
    pub created_at: i64,
}

/// Projects a SeaORM 2 loaded submission graph to the public scoreboard
/// contract. The graph supplies the related user and challenge without a
/// second hand-written join DTO.
impl TryFrom<&crate::entity::submission::ModelEx> for SubmissionPublic {
    type Error = DbError;

    fn try_from(submission: &crate::entity::submission::ModelEx) -> Result<Self, Self::Error> {
        let user = submission.user.as_ref().ok_or_else(|| {
            DbError::Other(anyhow::anyhow!(
                "submission {} was loaded without its user relation",
                submission.id
            ))
        })?;
        let challenge = submission.challenge.as_ref().ok_or_else(|| {
            DbError::Other(anyhow::anyhow!(
                "submission {} was loaded without its challenge relation",
                submission.id
            ))
        })?;

        Ok(Self {
            id: submission.id,
            user_id: submission.user_id,
            user_name: user.name.clone(),
            user_avatar_hash: user.avatar_hash.clone(),
            challenge_id: submission.challenge_id,
            challenge_title: challenge.title.clone(),
            pts: submission.pts,
            created_at: submission.created_at,
        })
    }
}

impl From<&Submission> for SubmissionPublic {
    fn from(submission: &Submission) -> Self {
        Self {
            id: submission.id,
            user_id: submission.user_id,
            user_name: submission.user_name.clone(),
            user_avatar_hash: submission.user_avatar_hash.clone(),
            challenge_id: submission.challenge_id,
            challenge_title: submission.challenge_title.clone(),
            pts: submission.pts,
            created_at: submission.created_at,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SubmissionPublic;

    #[test]
    fn public_submission_serialization_excludes_content_and_internal_scope() {
        let submission = SubmissionPublic {
            id: 1,
            user_id: 2,
            user_name: "user".to_owned(),
            user_avatar_hash: Some("avatar".to_owned()),
            challenge_id: 3,
            challenge_title: "challenge".to_owned(),
            pts: 100,
            created_at: 1_700_000_000,
        };

        assert_eq!(
            serde_json::to_value(submission).unwrap(),
            serde_json::json!({
                "id": 1,
                "user_id": 2,
                "user_name": "user",
                "user_avatar_hash": "avatar",
                "challenge_id": 3,
                "challenge_title": "challenge",
                "pts": 100,
                "created_at": 1_700_000_000,
            })
        );
    }
}

impl Submission {
    /// Strips secrets so configuration can be returned to clients.
    pub fn desensitize(&self) -> Self {
        Self {
            content: "".to_owned(),
            ..self.to_owned()
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct FindSubmissionsOptions {
    pub id: Option<i64>,
    pub user_id: Option<i64>,
    pub team_id: Option<Option<i64>>,
    pub game_id: Option<Option<i64>>,
    pub challenge_id: Option<i64>,
    pub status: Option<Status>,
    pub page: Option<u64>,
    pub size: Option<u64>,
    pub sorts: Option<String>,
}

/// Queries rows using filter options and returns `(rows, total_count)`.
pub async fn find(
    conn: &impl ConnectionTrait,
    FindSubmissionsOptions {
        id,
        user_id,
        team_id,
        game_id,
        challenge_id,
        status,
        page,
        size,
        sorts,
    }: FindSubmissionsOptions,
) -> Result<(Vec<Submission>, u64), DbError> {
    let mut loader = Entity::load()
        .with(crate::entity::user::Entity)
        .with(crate::entity::challenge::Entity)
        .with(crate::entity::team::Entity)
        .with(crate::entity::game::Entity);

    if let Some(id) = id {
        loader = loader.filter(Column::Id.eq(id));
    }

    if let Some(user_id) = user_id {
        loader = loader.filter(Column::UserId.eq(user_id));
    }

    if let Some(team_id) = team_id {
        match team_id {
            Some(team_id) => loader = loader.filter(Column::TeamId.eq(team_id)),
            None => loader = loader.filter(Column::TeamId.is_null()),
        }
    }

    if let Some(game_id) = game_id {
        match game_id {
            Some(game_id) => loader = loader.filter(Column::GameId.eq(game_id)),
            None => loader = loader.filter(Column::GameId.is_null()),
        }
    }

    if let Some(challenge_id) = challenge_id {
        loader = loader.filter(Column::ChallengeId.eq(challenge_id));
    }

    if let Some(status) = status {
        loader = loader.filter(Column::Status.eq(status));
    }

    if let Some(sorts) = sorts {
        let sorts = sorts.split(",").collect::<Vec<&str>>();
        for sort in sorts {
            let col = match Column::from_str(sort.replace("-", "").as_str()) {
                Ok(col) => col,
                Err(_) => continue,
            };
            if sort.starts_with("-") {
                loader = loader.order_by(col, Order::Desc);
            } else {
                loader = loader.order_by(col, Order::Asc);
            }
        }
    }

    let (models, total) = match (page, size) {
        (Some(_), Some(0)) => {
            let total = loader.clone().paginate(conn, 1).num_items().await?;
            (Vec::new(), total)
        }
        (Some(page), Some(size)) => {
            let paginator = loader.paginate(conn, size);
            let total = paginator.num_items().await?;
            let models = paginator.fetch_page(page.saturating_sub(1)).await?;
            (models, total)
        }
        _ => {
            let models = loader.all(conn).await?;
            let total = models.len() as u64;
            (models, total)
        }
    };

    let submissions = models
        .into_iter()
        .map(Submission::try_from)
        .collect::<Result<Vec<_>, _>>()?;

    Ok((submissions, total))
}

/// Looks up by id.

pub async fn find_by_id(
    conn: &impl ConnectionTrait,
    submission_id: i64,
) -> Result<Option<Submission>, DbError> {
    Ok(Entity::load()
        .with(crate::entity::user::Entity)
        .with(crate::entity::challenge::Entity)
        .with(crate::entity::team::Entity)
        .with(crate::entity::game::Entity)
        .filter(Column::Id.eq(submission_id))
        .one(conn)
        .await?
        .map(Submission::try_from)
        .transpose()?)
}

/// Looks up pending by id.

pub async fn find_pending_by_id(
    conn: &impl ConnectionTrait,
    submission_id: i64,
) -> Result<Option<Submission>, DbError> {
    Ok(Entity::load()
        .with(crate::entity::user::Entity)
        .with(crate::entity::challenge::Entity)
        .with(crate::entity::team::Entity)
        .with(crate::entity::game::Entity)
        .filter(Column::Id.eq(submission_id))
        .filter(Column::Status.eq(Status::Pending))
        .one(conn)
        .await?
        .map(Submission::try_from)
        .transpose()?)
}

/// Looks up correct by team ids and game id.

pub async fn find_correct_by_team_ids_and_game_id(
    conn: &impl ConnectionTrait,
    team_ids: Vec<i64>,
    game_id: i64,
) -> Result<Vec<Submission>, DbError> {
    Ok(Entity::load()
        .with(crate::entity::user::Entity)
        .with(crate::entity::challenge::Entity)
        .with(crate::entity::team::Entity)
        .with(crate::entity::game::Entity)
        .filter(Column::TeamId.is_in(team_ids))
        .filter(Column::GameId.eq(game_id))
        .filter(Column::Status.eq(Status::Correct))
        .all(conn)
        .await?
        .into_iter()
        .map(Submission::try_from)
        .collect::<Result<Vec<_>, _>>()?)
}

/// Looks up correct by challenge ids and optional team game.

pub async fn find_correct_by_challenge_ids_and_optional_team_game(
    conn: &impl ConnectionTrait,
    challenge_ids: Vec<i64>,
    team_id: Option<i64>,
    game_id: Option<i64>,
) -> Result<Vec<Submission>, DbError> {
    let mut loader = Entity::load()
        .with(crate::entity::user::Entity)
        .with(crate::entity::challenge::Entity)
        .with(crate::entity::team::Entity)
        .with(crate::entity::game::Entity)
        .filter(Column::ChallengeId.is_in(challenge_ids));

    if let (Some(_), Some(game_id)) = (team_id, game_id) {
        loader = loader.filter(Column::GameId.eq(game_id));
    } else {
        loader = loader
            .filter(Column::GameId.is_null())
            .filter(Column::TeamId.is_null());
    }

    let submissions = loader
        .filter(Column::Status.eq(Status::Correct))
        .order_by_asc(Column::CreatedAt)
        .all(conn)
        .await?
        .into_iter()
        .map(Submission::try_from)
        .collect::<Result<Vec<_>, _>>()?;

    Ok(submissions)
}

/// Checks if the given team+game scope has a `Cheat` submission for the
/// specified challenge.
pub async fn has_cheat(
    conn: &impl ConnectionTrait,
    challenge_id: i64,
    team_id: i64,
    game_id: i64,
) -> Result<bool, DbError> {
    let (submissions, _) = find(
        conn,
        FindSubmissionsOptions {
            challenge_id: Some(challenge_id),
            team_id: Some(Some(team_id)),
            game_id: Some(Some(game_id)),
            status: Some(Status::Cheat),
            page: Some(1),
            size: Some(1),
            ..Default::default()
        },
    )
    .await?;

    Ok(!submissions.is_empty())
}

/// Returns challenge_ids where the team has a `Cheat` submission, filtered to
/// the given set of candidate challenge_ids.
pub async fn find_cheat_challenge_ids(
    conn: &impl ConnectionTrait,
    challenge_ids: Vec<i64>,
    team_id: i64,
    game_id: i64,
) -> Result<Vec<i64>, DbError> {
    let submissions = Entity::find()
        .filter(Column::ChallengeId.is_in(challenge_ids))
        .filter(Column::Status.eq(Status::Cheat))
        .filter(Column::TeamId.eq(team_id))
        .filter(Column::GameId.eq(game_id))
        .select_only()
        .column(Column::ChallengeId)
        .into_tuple::<i64>()
        .all(conn)
        .await?;

    Ok(submissions)
}

/// Counts rows that match optional filters.
pub async fn count(conn: &impl ConnectionTrait) -> Result<u64, DbError> {
    Ok(Entity::find().count(conn).await?)
}

/// Counts submissions in `Correct` status for the given scope.
pub async fn count_correct(conn: &impl ConnectionTrait) -> Result<u64, DbError> {
    Ok(Entity::find()
        .filter(Column::Status.eq(Status::Correct))
        .count(conn)
        .await?)
}

/// Inserts a new row and returns the persisted model.
pub async fn create(
    conn: &impl ConnectionTrait,
    model: ActiveModel,
) -> Result<Submission, DbError> {
    let submission = model.insert(conn).await?;
    info!(
        submission_id = submission.id,
        user_id = submission.user_id,
        team_id = submission.team_id,
        game_id = submission.game_id,
        challenge_id = submission.challenge_id,
        status = ?submission.status,
        "submission created"
    );

    Ok(find_by_id(conn, submission.id)
        .await?
        .ok_or_else(|| DbError::NotFound(format!("submission_{}", submission.id)))?)
}

/// Applies an active model update to the database.
pub async fn update(
    conn: &impl ConnectionTrait,
    model: ActiveModel,
) -> Result<Submission, DbError> {
    let submission = model.update(conn).await?;
    info!(
        submission_id = submission.id,
        user_id = submission.user_id,
        team_id = submission.team_id,
        game_id = submission.game_id,
        challenge_id = submission.challenge_id,
        status = ?submission.status,
        pts = submission.pts,
        rank = submission.rank,
        "submission updated"
    );

    Ok(find_by_id(conn, submission.id)
        .await?
        .ok_or_else(|| DbError::NotFound(format!("submission_{}", submission.id)))?)
}

/// Deletes rows matching the provided identifier or filter.
pub async fn delete(conn: &impl ConnectionTrait, submission_id: i64) -> Result<(), DbError> {
    Entity::delete_by_id(submission_id).exec(conn).await?;
    info!(submission_id, "submission deleted");

    Ok(())
}
