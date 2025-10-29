use std::str::FromStr;

use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, FromQueryResult, JoinType, Order,
    PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, RelationTrait,
};
use serde::{Deserialize, Serialize};

pub use super::team_user::find_teams as find_by_user_id;
pub use crate::entity::team::{ActiveModel, Model, State};
pub(crate) use crate::entity::team::{Column, Entity};
use crate::get_db;

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromQueryResult)]
pub struct Team {
    pub id: i64,
    pub game_id: i64,
    pub name: String,
    pub email: Option<String>,
    pub slogan: Option<String>,
    pub has_avatar: bool,
    pub has_write_up: bool,
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
    pub has_write_up: Option<bool>,
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

pub async fn find<T>(
    FindTeamOptions {
        id,
        name,
        state,
        has_write_up,
        game_id,
        user_id,
        page,
        size,
        sorts,
    }: FindTeamOptions,
) -> Result<(Vec<T>, u64), DbErr>
where
    T: FromQueryResult,
{
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

    if let Some(has_write_up) = has_write_up {
        sql = sql.filter(Column::HasWriteUp.eq(has_write_up));
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

    let total = sql.clone().count(get_db()).await?;

    if let Some(sorts) = sorts {
        for sort in sorts.split(",") {
            let (col_name, order) = if let Some(name) = sort.strip_prefix("-") {
                (name, Order::Desc)
            } else {
                (sort, Order::Asc)
            };
            
            let col = match Column::from_str(col_name) {
                Ok(col) => col,
                Err(_) => continue,
            };
            sql = sql.order_by(col, order);
        }
    }

    if let (Some(page), Some(size)) = (page, size) {
        let offset = (page - 1) * size;
        sql = sql.offset(offset).limit(size);
    }

    let teams = sql.into_model::<T>().all(get_db()).await?;

    Ok((teams, total))
}

pub async fn find_by_id<T>(team_id: i64, game_id: i64) -> Result<Option<T>, DbErr>
where
    T: FromQueryResult,
{
    Ok(Entity::find_by_id(team_id)
        .filter(Column::GameId.eq(game_id))
        .into_model::<T>()
        .one(get_db())
        .await?)
}

pub async fn create<T>(model: ActiveModel) -> Result<T, DbErr>
where
    T: FromQueryResult,
{
    let team = model.insert(get_db()).await?;

    Ok(find_by_id::<T>(team.id, team.game_id).await?.unwrap())
}

pub async fn update<T>(model: ActiveModel) -> Result<T, DbErr>
where
    T: FromQueryResult,
{
    let team = model.update(get_db()).await?;

    Ok(find_by_id::<T>(team.id, team.game_id).await?.unwrap())
}

pub async fn delete(team_id: i64) -> Result<(), DbErr> {
    Entity::delete_by_id(team_id).exec(get_db()).await?;

    Ok(())
}
