//! Database access for `team` — SeaORM queries, updates, and DTOs.

use std::str::FromStr;

use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityLoaderTrait, EntityTrait,
    FromQueryResult, JoinType, LoaderTraitEx, Order, PaginatorTrait, QueryFilter, QueryOrder,
    QuerySelect, RelationTrait,
};
use tracing::info;

pub use super::team_user::find_teams as find_by_user_id;
pub(crate) use crate::entity::team::{Column, Entity};
use crate::traits::DbError;
pub use crate::{
    dto::{
        scoreboard::{ScoreboardEntry, ScoreboardTeam},
        team::TeamView,
    },
    entity::team::{ActiveModel, Model, State},
};

impl From<&crate::entity::team::ModelEx> for ScoreboardTeam {
    fn from(team: &crate::entity::team::ModelEx) -> Self {
        Self {
            id: team.id,
            name: team.name.clone(),
            slogan: team.slogan.clone(),
            avatar_hash: team.avatar_hash.clone(),
            pts: team.pts,
            rank: team.rank,
        }
    }
}

/// Loads the public scoreboard graph with SeaORM 2's batch loader.
///
/// Teams are paginated in the database. Correct submissions are loaded in one
/// `WHERE team_id IN (...)` query, then their user and challenge relations are
/// loaded in batches. This avoids the previous parent query + global child
/// query + per-parent filtering pipeline without exposing internal columns.
pub async fn find_scoreboard(
    conn: &impl ConnectionTrait,
    game_id: i64,
    page: Option<u64>,
    size: Option<u64>,
) -> Result<(Vec<ScoreboardEntry>, u64), DbError> {
    use crate::entity::{submission, team};

    let loader = team::Entity::load()
        .filter(team::Column::GameId.eq(game_id))
        .filter(team::Column::State.eq(team::State::Passed))
        .order_by_asc(team::Column::Rank)
        .order_by_desc(team::Column::Pts)
        .order_by_asc(team::Column::Id);

    let (teams, total) = match (page, size) {
        (Some(_page), Some(0)) => {
            let total = loader.clone().paginate(conn, 1).num_items().await?;
            (Vec::new(), total)
        }
        (Some(page), Some(size)) => {
            let paginator = loader.paginate(conn, size);
            let total = paginator.num_items().await?;
            let teams = paginator.fetch_page(page.saturating_sub(1)).await?;
            (teams, total)
        }
        _ => {
            let teams = loader.all(conn).await?;
            let total = teams.len() as u64;
            (teams, total)
        }
    };

    if teams.is_empty() {
        return Ok((Vec::new(), total));
    }

    let submission_groups = teams
        .as_slice()
        .load_many_ex(
            submission::Entity::find()
                .filter(submission::Column::GameId.eq(game_id))
                .filter(submission::Column::Status.eq(submission::Status::Correct))
                .order_by_asc(submission::Column::CreatedAt)
                .order_by_asc(submission::Column::Id),
            conn,
        )
        .await?;

    let mut submission_with = submission::EntityLoaderWith::default();
    submission_with.user = true;
    submission_with.challenge = true;
    let submission_groups =
        submission::EntityLoader::load_nest_nest(submission_groups, &submission_with, conn).await?;

    let records = teams
        .into_iter()
        .zip(submission_groups)
        .map(|(team, submissions)| {
            let submissions = submissions
                .into_iter()
                .map(|submission| {
                    crate::dto::scoreboard::ScoreboardSubmission::try_from(&submission)
                })
                .collect::<Result<Vec<_>, _>>()?;

            Ok(ScoreboardEntry {
                team: ScoreboardTeam::from(&team),
                submissions,
            })
        })
        .collect::<Result<Vec<_>, DbError>>()?;

    Ok((records, total))
}

#[cfg(test)]
mod tests {
    use crate::dto::scoreboard::{ScoreboardEntry, ScoreboardSubmission, ScoreboardTeam};

    #[test]
    fn scoreboard_serialization_is_an_explicit_public_contract() {
        let record = ScoreboardEntry {
            team: ScoreboardTeam {
                id: 1,
                name: "team".to_owned(),
                slogan: Some("hello".to_owned()),
                avatar_hash: Some("team-avatar".to_owned()),
                pts: 100,
                rank: 2,
            },
            submissions: vec![ScoreboardSubmission {
                id: 10,
                user_id: 20,
                user_name: "user".to_owned(),
                user_avatar_hash: Some("user-avatar".to_owned()),
                challenge_id: 30,
                challenge_title: "challenge".to_owned(),
                pts: 100,
                created_at: 1_700_000_000,
            }],
        };
        assert_eq!(
            serde_json::to_value(record).unwrap(),
            serde_json::json!({"team":{"id":1,"name":"team","slogan":"hello","avatar_hash":"team-avatar","pts":100,"rank":2},"submissions":[{"id":10,"user_id":20,"user_name":"user","user_avatar_hash":"user-avatar","challenge_id":30,"challenge_title":"challenge","pts":100,"created_at":1_700_000_000_i64}]})
        );
    }
}

#[derive(Clone, Debug, Default)]
pub struct FindTeamOptions {
    /// The team id of expected game teams.
    pub id: Option<i64>,
    pub name: Option<String>,
    pub state: Option<State>,
    pub has_writeup: Option<bool>,
    pub game_id: Option<i64>,

    /// The user id of expected game teams.
    ///
    /// `user_id` is not in table `teams`, so it relies on JOIN queries.
    /// Essentially, it is unrelated to game team.
    ///
    /// ```sql
    /// SELECT *
    /// FROM "teams"
    ///     INNER JOIN "team_users" ON "teams"."id" = "team_users"."team_id"
    /// WHERE "team_users"."game_id" = ? AND "team_users"."user_id" = ?;
    /// ```
    pub user_id: Option<i64>,

    pub page: Option<u64>,
    pub size: Option<u64>,
    pub sorts: Option<String>,
}

/// Queries rows using filter options and returns `(rows, total_count)`.
pub async fn find<T>(
    conn: &impl ConnectionTrait,
    FindTeamOptions {
        id,
        name,
        state,
        has_writeup,
        game_id,
        user_id,
        page,
        size,
        sorts,
    }: FindTeamOptions,
) -> Result<(Vec<T>, u64), DbError>
where
    T: FromQueryResult, {
    let mut sql = Entity::find();

    sql = sql.filter(Column::GameId.eq(game_id));

    if let Some(id) = id {
        sql = sql.filter(Column::Id.eq(id));
    }

    if let Some(name) = name {
        sql = sql.filter(Column::Name.contains(name));
    }

    if let Some(state) = state {
        sql = sql.filter(Column::State.eq(state));
    }

    if let Some(has_writeup) = has_writeup {
        sql = sql.filter(Column::HasWriteup.eq(has_writeup));
    }

    if let Some(user_id) = user_id {
        // If you are a little confused about the following statement,
        // you can refer to the comments on the field `user_id` in `GetTeamRequest`
        sql = sql
            .join(
                JoinType::InnerJoin,
                super::team_user::Relation::Team.def().rev(),
            )
            .filter(super::team_user::Column::UserId.eq(user_id))
    }

    let total = sql.clone().count(conn).await?;

    if let Some(sorts) = sorts {
        let sorts = sorts.split(",").collect::<Vec<&str>>();
        for sort in sorts {
            let col = match Column::from_str(sort.replace("-", "").as_str()) {
                Ok(col) => col,
                Err(_) => continue,
            };
            if sort.starts_with("-") {
                sql = sql.order_by(col, Order::Desc);
            } else {
                sql = sql.order_by(col, Order::Asc);
            }
        }
    }

    if let (Some(page), Some(size)) = (page, size) {
        let offset = (page - 1) * size;
        sql = sql.offset(offset).limit(size);
    }

    let teams = sql.into_model::<T>().all(conn).await?;

    Ok((teams, total))
}

/// Looks up by id.

pub async fn find_by_id<T>(
    conn: &impl ConnectionTrait,
    team_id: i64,
    game_id: i64,
) -> Result<Option<T>, DbError>
where
    T: FromQueryResult, {
    Ok(Entity::find_by_id(team_id)
        .filter(Column::GameId.eq(game_id))
        .into_model::<T>()
        .one(conn)
        .await?)
}

/// Inserts a new row and returns the persisted model.
pub async fn create<T>(conn: &impl ConnectionTrait, model: ActiveModel) -> Result<T, DbError>
where
    T: FromQueryResult, {
    let team = model.insert(conn).await?;
    info!(
        team_id = team.id,
        game_id = team.game_id,
        name = %team.name,
        state = ?team.state,
        "team created"
    );

    Ok(find_by_id::<T>(conn, team.id, team.game_id)
        .await?
        .ok_or_else(|| DbError::NotFound(format!("team_{}", team.id)))?)
}

/// Applies an active model update to the database.
pub async fn update<T>(conn: &impl ConnectionTrait, model: ActiveModel) -> Result<T, DbError>
where
    T: FromQueryResult, {
    let team = model.update(conn).await?;
    info!(
        team_id = team.id,
        game_id = team.game_id,
        name = %team.name,
        state = ?team.state,
        pts = team.pts,
        rank = team.rank,
        "team updated"
    );

    Ok(find_by_id::<T>(conn, team.id, team.game_id)
        .await?
        .ok_or_else(|| DbError::NotFound(format!("team_{}", team.id)))?)
}

/// Deletes rows matching the provided identifier or filter.
pub async fn delete(conn: &impl ConnectionTrait, team_id: i64) -> Result<(), DbError> {
    Entity::delete_by_id(team_id).exec(conn).await?;
    info!(team_id, "team deleted");

    Ok(())
}
