//! Database access for `team` — SeaORM queries, updates, and DTOs.
//!
//! Team scores (`pts`) and standings (`rank`) are no longer persisted on the
//! `teams` table. Queries here return raw team metadata with `pts = 0, rank =
//! 0` and expose narrow ID/batch loaders so the CQRS score service can rank,
//! paginate, and hydrate teams without full-table sorts or write amplification.

use std::{collections::HashMap, str::FromStr};

use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, JoinType, LoaderTraitEx, Order,
    PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, RelationTrait, sea_query::Expr,
};

pub use super::team_user::find_teams as find_by_user_id;
pub(crate) use crate::entity::team::{Column, Entity};
use crate::traits::DbError;
pub use crate::{
    dto::{
        scoreboard::{ScoreboardEntry, ScoreboardTeam},
        team::{AdminTeamView, TeamView},
        user::UserSummary,
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
            pts: 0,
            rank: 0,
        }
    }
}

/// Batch-loads scoreboard display metadata (team profiles + correct solves +
/// user/challenge names) for a single pre-ranked page of `team_ids`.
///
/// Output preserves the caller's exact `team_ids` ordering. Derived `pts` and
/// `rank` fields are left at `0` for the CQRS score service to hydrate.
pub async fn find_scoreboard_entries_by_ids(
    conn: &impl ConnectionTrait,
    game_id: i64,
    team_ids: &[i64],
) -> Result<Vec<ScoreboardEntry>, DbError> {
    use crate::entity::{submission, team};

    if team_ids.is_empty() {
        return Ok(Vec::new());
    }

    let teams = team::Entity::load()
        .filter(team::Column::GameId.eq(game_id))
        .filter(team::Column::Id.is_in(team_ids.iter().copied()))
        .all(conn)
        .await?;

    if teams.is_empty() {
        return Ok(Vec::new());
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

    let mut by_id: HashMap<i64, ScoreboardEntry> = HashMap::with_capacity(teams.len());
    for (team, submissions) in teams.into_iter().zip(submission_groups) {
        let submissions = submissions
            .into_iter()
            .map(|submission| crate::dto::scoreboard::ScoreboardSubmission::try_from(&submission))
            .collect::<Result<Vec<_>, _>>()?;
        let team_id = team.id;
        by_id.insert(
            team_id,
            ScoreboardEntry {
                team: ScoreboardTeam::from(&team),
                submissions,
            },
        );
    }

    // Reorder to match the pre-ranked slice supplied by the score snapshot.
    Ok(team_ids.iter().filter_map(|id| by_id.remove(id)).collect())
}

#[derive(Clone, Debug, Default)]
pub struct FindTeamOptions {
    /// The team id of expected game teams.
    pub id: Option<i64>,
    pub name: Option<String>,
    pub state: Option<State>,
    pub has_writeup: Option<bool>,
    pub game_id: Option<i64>,

    /// The user id of expected game teams (`INNER JOIN team_users`).
    pub user_id: Option<i64>,

    pub page: Option<u64>,
    pub size: Option<u64>,
    pub sorts: Option<String>,
}

/// Applies common filter predicates shared by `find`, `find_ids`, and admin
/// rank-sorted pagination.
fn apply_filters(
    mut sql: sea_orm::Select<Entity>,
    game_id: Option<i64>,
    id: Option<i64>,
    name: Option<String>,
    state: Option<State>,
    has_writeup: Option<bool>,
    user_id: Option<i64>,
) -> sea_orm::Select<Entity> {
    if let Some(game_id) = game_id {
        sql = sql.filter(Column::GameId.eq(game_id));
    }

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
        sql = sql
            .join(
                JoinType::InnerJoin,
                super::team_user::Relation::Team.def().rev(),
            )
            .filter(super::team_user::Column::UserId.eq(user_id));
    }

    sql
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

/// Loads a passed team only when the user is its member in this game.
/// Submission admission needs the team's display fields, not a separate count.
pub async fn find_passed_member_in_game(
    conn: &impl ConnectionTrait,
    team_id: i64,
    game_id: i64,
    user_id: i64,
) -> Result<Option<TeamView>, DbError> {
    Ok(
        user_game_membership_query(game_id, user_id, Some(State::Passed))
            .filter(Column::Id.eq(team_id))
            .one(conn)
            .await?
            .map(TeamView::from),
    )
}

/// Updates only the policy state of one team in the requested game.
///
/// The game predicate prevents a stale or cross-game identifier from changing
/// an unrelated team. Callers can use the Boolean result as a transactional
/// precondition for a larger policy transition.
pub async fn set_state_in_game(
    conn: &impl ConnectionTrait,
    team_id: i64,
    game_id: i64,
    state: State,
) -> Result<bool, DbError> {
    Ok(Entity::update_many()
        .col_expr(Column::State, Expr::value(state))
        .filter(Column::Id.eq(team_id))
        .filter(Column::GameId.eq(game_id))
        .exec(conn)
        .await?
        .rows_affected
        == 1)
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

/// Loads IDs of all `Passed` teams in a game via index-only scan
/// (`idx_teams_game_state`).
pub async fn find_passed_ids(
    conn: &impl ConnectionTrait,
    game_id: i64,
) -> Result<Vec<i64>, DbError> {
    Ok(Entity::find()
        .select_only()
        .column(Column::Id)
        .filter(Column::GameId.eq(game_id))
        .filter(Column::State.eq(State::Passed))
        .order_by_asc(Column::Id)
        .into_tuple::<i64>()
        .all(conn)
        .await?)
}

/// Loads only matching team IDs for filter-then-rank sorting in the admin UI.
pub async fn find_ids(
    conn: &impl ConnectionTrait,
    options: &FindTeamOptions,
) -> Result<Vec<i64>, DbError> {
    let sql = apply_filters(
        Entity::find().select_only().column(Column::Id),
        options.game_id,
        options.id,
        options.name.clone(),
        options.state.clone(),
        options.has_writeup,
        options.user_id,
    )
    .order_by_asc(Column::Id);

    Ok(sql.into_tuple::<i64>().all(conn).await?)
}

/// Batch-loads full admin team rows + member lists for one page of `team_ids`,
/// preserving the exact input order.
pub async fn find_with_users_by_ids(
    conn: &impl ConnectionTrait,
    game_id: i64,
    team_ids: &[i64],
) -> Result<Vec<AdminTeamView>, DbError> {
    use crate::entity::user;

    if team_ids.is_empty() {
        return Ok(Vec::new());
    }

    let teams = Entity::find()
        .filter(Column::GameId.eq(game_id))
        .filter(Column::Id.is_in(team_ids.iter().copied()))
        .all(conn)
        .await?;
    let members = teams.as_slice().load_many_ex(user::Entity, conn).await?;

    let mut by_id: HashMap<i64, AdminTeamView> = HashMap::with_capacity(teams.len());
    for (team, members) in teams.into_iter().zip(members) {
        let team_id = team.id;
        by_id.insert(
            team_id,
            AdminTeamView {
                team: TeamView::from(team),
                members: members
                    .into_iter()
                    .map(|member| UserSummary {
                        id: member.id,
                        name: member.name,
                        username: member.username,
                        avatar_hash: member.avatar_hash,
                    })
                    .collect(),
            },
        );
    }

    Ok(team_ids.iter().filter_map(|id| by_id.remove(id)).collect())
}

/// Queries rows using filter options and returns `(rows, total_count)`.
///
/// Sorting by derived fields (`pts`, `rank`) is handled by the caller via the
/// CQRS score snapshot; unrecognized column names in `sorts` are skipped.
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
    T: From<Model>, {
    let mut sql = apply_filters(
        Entity::find(),
        game_id,
        id,
        name,
        state,
        has_writeup,
        user_id,
    );

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
        let offset = page.saturating_sub(1).saturating_mul(size);
        sql = sql.offset(offset).limit(size);
    }

    let teams = sql.all(conn).await?.into_iter().map(T::from).collect();

    Ok((teams, total))
}

/// Queries paginated admin teams and batch-loads their members.
pub async fn find_with_users(
    conn: &impl ConnectionTrait,
    options: FindTeamOptions,
) -> Result<(Vec<AdminTeamView>, u64), DbError> {
    use crate::entity::user;

    let (teams, total) = find::<Model>(conn, options).await?;
    let members = teams.as_slice().load_many_ex(user::Entity, conn).await?;

    let teams = teams
        .into_iter()
        .zip(members)
        .map(|(team, members)| AdminTeamView {
            team: TeamView::from(team),
            members: members
                .into_iter()
                .map(|member| UserSummary {
                    id: member.id,
                    name: member.name,
                    username: member.username,
                    avatar_hash: member.avatar_hash,
                })
                .collect(),
        })
        .collect();

    Ok((teams, total))
}

/// Looks up by id.
pub async fn find_by_id<T>(
    conn: &impl ConnectionTrait,
    team_id: i64,
    game_id: i64,
) -> Result<Option<T>, DbError>
where
    T: From<Model>, {
    Ok(Entity::find_by_id(team_id)
        .filter(Column::GameId.eq(game_id))
        .one(conn)
        .await?
        .map(T::from))
}

/// Inserts a new row and returns the persisted model.
pub async fn create<T>(conn: &impl ConnectionTrait, model: ActiveModel) -> Result<T, DbError>
where
    T: From<Model>, {
    let team = model.insert(conn).await?;
    tracing::info!(
        resource = "team",
        action = "create",
        team_id = team.id,
        game_id = team.game_id,
        name = ?team.name,
        state = ?team.state,
        "database step completed",
    );

    Ok(T::from(team))
}

/// Applies an active model update to the database.
pub async fn update<T>(conn: &impl ConnectionTrait, model: ActiveModel) -> Result<T, DbError>
where
    T: From<Model>, {
    let changed_fields = [
        ("game_id", model.game_id.is_set()),
        ("name", model.name.is_set()),
        ("email", model.email.is_set()),
        ("slogan", model.slogan.is_set()),
        ("avatar_hash", model.avatar_hash.is_set()),
        ("has_writeup", model.has_writeup.is_set()),
        ("state", model.state.is_set()),
    ]
    .into_iter()
    .filter_map(|(name, changed)| changed.then_some(name))
    .collect::<Vec<_>>();
    let team = model.update(conn).await?;
    tracing::info!(
        resource = "team",
        action = "update",
        changed_fields = ?changed_fields,
        team_id = team.id,
        game_id = team.game_id,
        name = ?team.name,
        state = ?team.state,
        "database step completed",
    );

    Ok(T::from(team))
}

/// Deletes rows matching the provided identifier or filter.
pub async fn delete(conn: &impl ConnectionTrait, team_id: i64) -> Result<(), DbError> {
    Entity::delete_by_id(team_id).exec(conn).await?;
    tracing::info!(
        resource = "team",
        action = "delete",
        team_id = team_id,
        "resource step completed"
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use sea_orm::{DbBackend, QueryTrait};

    use super::{State, user_game_membership_query};
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
                created_at: time::OffsetDateTime::from_unix_timestamp(1_700_000_000).unwrap(),
            }],
        };
        assert_eq!(
            serde_json::to_value(record).unwrap(),
            serde_json::json!({"team":{"id":1,"name":"team","slogan":"hello","avatar_hash":"team-avatar","pts":100,"rank":2},"submissions":[{"id":10,"user_id":20,"user_name":"user","user_avatar_hash":"user-avatar","challenge_id":30,"challenge_title":"challenge","pts":100,"created_at":"2023-11-14T22:13:20Z"}]})
        );
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
