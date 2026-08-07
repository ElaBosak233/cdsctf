//! Database access for `team` — SeaORM queries, updates, and DTOs.

use std::str::FromStr;

use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityLoaderTrait, EntityTrait,
    FromQueryResult, JoinType, LoaderTraitEx, Order, PaginatorTrait, QueryFilter, QueryOrder,
    QuerySelect, RelationTrait,
    sea_query::{
        Alias, Condition, Expr, ExprTrait, IntoIden, Query, TableRef, UpdateStatement, ValueTuple,
    },
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
        // A freshly passed team can briefly retain the default rank of zero
        // until the asynchronous score calculation is applied. Keep it out
        // of the first place while that calculation is pending.
        .order_by_asc(Into::<Expr>::into(
            Expr::case(team::Column::Rank.eq(0), 1).finally(0),
        ))
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
    use sea_orm::{DbBackend, QueryTrait};

    use super::{ScoreUpdate, State, score_update_statements, user_game_membership_query};
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

    #[test]
    fn scoreboard_orders_unranked_teams_after_ranked_teams() {
        use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QueryOrder, QueryTrait};

        use crate::entity::team;

        let statement = team::Entity::find()
            .filter(team::Column::GameId.eq(7))
            .order_by_asc(Into::<sea_orm::prelude::Expr>::into(
                sea_orm::prelude::Expr::case(team::Column::Rank.eq(0), 1).finally(0),
            ))
            .order_by_asc(team::Column::Rank)
            .build(DbBackend::Postgres);

        assert!(statement.sql.contains("CASE WHEN"));
        assert!(statement.sql.contains("\"teams\".\"rank\" = $"));
    }

    #[test]
    fn score_updates_use_bound_values_and_change_checks() {
        let statements = score_update_statements(
            5,
            &[
                ScoreUpdate {
                    id: 1,
                    pts: 100,
                    rank: 1,
                },
                ScoreUpdate {
                    id: 2,
                    pts: 50,
                    rank: 2,
                },
            ],
        );

        let statement = DbBackend::Postgres.build(&statements[0]);
        assert_eq!(statements.len(), 1);
        assert_eq!(statement.values.unwrap().0.len(), 8);
        assert!(statement.sql.starts_with("UPDATE \"teams\""));
        assert!(statement.sql.contains("FROM (VALUES"));
        assert!(statement.sql.contains(" OR "));
    }

    #[test]
    fn game_membership_query_joins_team_users_and_filters_state() {
        let statement =
            user_game_membership_query(7, 9, Some(State::Passed)).build(DbBackend::Postgres);

        assert!(statement.sql.contains("INNER JOIN \"team_users\""));
        assert!(statement.sql.contains("\"team_users\".\"user_id\" = $1"));
        assert!(statement.sql.contains("\"teams\".\"game_id\" = $2"));
        assert!(statement.sql.contains("\"teams\".\"state\" = $3"));
        assert_eq!(statement.values.unwrap().0.len(), 3);
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

/// Returns whether a user belongs to a team in the requested game and state.
pub async fn contains_user_in_game(
    conn: &impl ConnectionTrait,
    game_id: i64,
    user_id: i64,
    state: Option<State>,
) -> Result<bool, DbError> {
    Ok(user_game_membership_query(game_id, user_id, state)
        .count(conn)
        .await?
        > 0)
}

fn user_game_membership_query(
    game_id: i64,
    user_id: i64,
    state: Option<State>,
) -> sea_orm::Select<Entity> {
    let mut query = Entity::find()
        .join(
            JoinType::InnerJoin,
            super::team_user::Relation::Team.def().rev(),
        )
        .filter(super::team_user::Column::UserId.eq(user_id))
        .filter(Column::GameId.eq(game_id));

    if let Some(state) = state {
        query = query.filter(Column::State.eq(state));
    }

    query
}

/// Narrow team projection used by score recomputation.
#[derive(Clone, Debug, PartialEq, Eq, FromQueryResult)]
pub struct ScoreInput {
    pub id: i64,
    pub pts: i64,
    pub rank: i64,
}

/// Persisted score fields for one team.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScoreUpdate {
    pub id: i64,
    pub pts: i64,
    pub rank: i64,
}

/// Loads passed teams using only fields needed for score recomputation.
pub async fn find_score_inputs(
    conn: &impl ConnectionTrait,
    game_id: i64,
) -> Result<Vec<ScoreInput>, DbError> {
    Ok(Entity::find()
        .select_only()
        .columns([Column::Id, Column::Pts, Column::Rank])
        .filter(Column::GameId.eq(game_id))
        .filter(Column::State.eq(State::Passed))
        .order_by_asc(Column::Id)
        .into_model::<ScoreInput>()
        .all(conn)
        .await?)
}

/// Applies changed team scores in bounded, set-based updates.
pub async fn update_scores(
    conn: &impl ConnectionTrait,
    game_id: i64,
    updates: &[ScoreUpdate],
) -> Result<u64, DbError> {
    let mut rows_affected = 0;
    for statement in score_update_statements(game_id, updates) {
        rows_affected += conn.execute(&statement).await?.rows_affected();
    }
    Ok(rows_affected)
}

fn score_update_statements(game_id: i64, updates: &[ScoreUpdate]) -> Vec<UpdateStatement> {
    updates
        .chunks(super::BULK_UPDATE_BATCH_SIZE)
        .map(|chunk| {
            let source = Alias::new("team_scores");
            let values = chunk
                .iter()
                .map(|update| {
                    ValueTuple::Many(vec![
                        game_id.into(),
                        update.id.into(),
                        update.pts.into(),
                        update.rank.into(),
                    ])
                })
                .collect();
            let source_game_id = Expr::col((source.clone(), Alias::new("column1")));
            let source_id = Expr::col((source.clone(), Alias::new("column2")));
            let source_pts = Expr::col((source.clone(), Alias::new("column3")));
            let source_rank = Expr::col((source.clone(), Alias::new("column4")));

            Query::update()
                .table(Entity)
                .value(Column::Pts, source_pts.clone())
                .value(Column::Rank, source_rank.clone())
                .from(TableRef::ValuesList(values, source.into_iden()))
                .cond_where(Expr::col(Column::GameId).eq(source_game_id))
                .cond_where(Expr::col(Column::Id).eq(source_id))
                .cond_where(
                    Condition::any()
                        .add(Expr::col(Column::Pts).ne(source_pts))
                        .add(Expr::col(Column::Rank).ne(source_rank)),
                )
                .to_owned()
        })
        .collect()
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
