//! Database access for `team` — SeaORM queries, updates, and DTOs.

use std::str::FromStr;

use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, FromQueryResult, JoinType, Order,
    PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, RelationTrait,
};
use serde::{Deserialize, Serialize};

pub use super::team_user::find_teams as find_by_user_id;
pub use crate::entity::team::{ActiveModel, Model, State};
pub(crate) use crate::entity::team::{Column, Entity};
use crate::traits::DbError;

#[allow(dead_code)]
#[derive(
    Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromQueryResult, utoipa::ToSchema,
)]
pub struct Team {
    pub id: i64,
    pub game_id: i64,
    pub name: String,
    pub email: Option<String>,
    pub slogan: Option<String>,
    pub has_avatar: bool,
    pub has_writeup: bool,
    pub state: State,
    pub pts: i64,
    pub rank: i64,
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

    Ok(find_by_id::<T>(conn, team.id, team.game_id)
        .await?
        .ok_or_else(|| DbError::NotFound(format!("team_{}", team.id)))?)
}

/// Applies an active model update to the database.
pub async fn update<T>(conn: &impl ConnectionTrait, model: ActiveModel) -> Result<T, DbError>
where
    T: FromQueryResult, {
    let team = model.update(conn).await?;

    Ok(find_by_id::<T>(conn, team.id, team.game_id)
        .await?
        .ok_or_else(|| DbError::NotFound(format!("team_{}", team.id)))?)
}

/// Deletes rows matching the provided identifier or filter.
pub async fn delete(conn: &impl ConnectionTrait, team_id: i64) -> Result<(), DbError> {
    Entity::delete_by_id(team_id).exec(conn).await?;

    Ok(())
}
