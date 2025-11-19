use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, FromQueryResult, PaginatorTrait, QueryFilter,
};
use serde::{Deserialize, Serialize};

use super::{
    team::{Column as TeamColumn, Entity as TeamEntity},
    user::{Column as UserColumn, Entity as UserEntity},
};
pub use crate::entity::team_user::ActiveModel;
pub(crate) use crate::entity::team_user::{Column, Entity, Relation};
use crate::{get_db, traits::DbError};

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromQueryResult)]
pub struct TeamUser {
    pub team_id: i64,
    pub user_id: i64,
}

#[derive(Clone, Debug, Default)]
pub struct FindTeamUserOptions {
    pub team_id: Option<i64>,
    pub user_id: Option<i64>,
}

pub async fn find<T>(
    FindTeamUserOptions { team_id, user_id }: FindTeamUserOptions,
) -> Result<(Vec<T>, u64), DbError>
where
    T: FromQueryResult, {
    let mut sql = Entity::find();

    if let Some(team_id) = team_id {
        sql = sql.filter(Column::TeamId.eq(team_id));
    }

    if let Some(user_id) = user_id {
        sql = sql.filter(Column::UserId.eq(user_id));
    }

    let total = sql.clone().count(get_db()).await?;
    let team_users = sql.into_model::<T>().all(get_db()).await?;

    Ok((team_users, total))
}

pub async fn find_by_id<T>(team_id: i64, user_id: i64) -> Result<Option<T>, DbError>
where
    T: FromQueryResult, {
    Ok(Entity::find()
        .filter(Column::TeamId.eq(team_id))
        .filter(Column::UserId.eq(user_id))
        .into_model::<T>()
        .one(get_db())
        .await?)
}

pub async fn find_users<T>(team_id: i64) -> Result<Vec<T>, DbError>
where
    T: FromQueryResult, {
    Ok(UserEntity::find()
        .inner_join(TeamEntity)
        .filter(TeamColumn::Id.eq(team_id))
        .into_model::<T>()
        .all(get_db())
        .await?)
}

pub async fn find_teams<T>(user_id: i64) -> Result<Vec<T>, DbError>
where
    T: FromQueryResult, {
    Ok(TeamEntity::find()
        .inner_join(UserEntity)
        .filter(UserColumn::Id.eq(user_id))
        .into_model::<T>()
        .all(get_db())
        .await?)
}

pub async fn create<T>(model: ActiveModel) -> Result<T, DbError>
where
    T: FromQueryResult, {
    let team_user = model.insert(get_db()).await?;

    Ok(find_by_id::<T>(team_user.team_id, team_user.user_id)
        .await?
        .ok_or_else(|| {
            DbError::NotFound(format!(
                "team_user_{}_{}",
                team_user.team_id, team_user.user_id
            ))
        })?)
}

pub async fn delete(team_id: i64, user_id: i64) -> Result<(), DbError> {
    Entity::delete_many()
        .filter(Column::TeamId.eq(team_id))
        .filter(Column::UserId.eq(user_id))
        .exec(get_db())
        .await?;

    Ok(())
}

pub async fn delete_by_team_id(team_id: i64) -> Result<(), DbError> {
    Entity::delete_many()
        .filter(Column::TeamId.eq(team_id))
        .exec(get_db())
        .await?;

    Ok(())
}
