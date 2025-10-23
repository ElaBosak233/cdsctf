use std::str::FromStr;

use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, Order, PaginatorTrait, QueryFilter,
    QueryOrder, QuerySelect,
};
use serde::{Deserialize, Serialize};

pub use crate::entity::submission::{ActiveModel, Status};
pub(crate) use crate::entity::submission::{Column, Entity};
use crate::{get_db, sea_orm, sea_orm::FromQueryResult};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromQueryResult)]
pub struct Submission {
    pub id: i64,
    pub content: String,
    pub status: Status,
    pub user_id: i64,
    pub user_name: String,
    pub user_has_avatar: bool,
    pub team_id: Option<i64>,
    pub team_name: Option<String>,
    pub game_id: Option<i64>,
    pub game_title: Option<String>,
    pub challenge_id: i64,
    pub challenge_title: String,
    pub challenge_category: i32,
    pub created_at: i64,

    pub pts: i64,
    pub rank: i64,
}

impl Submission {
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

pub async fn find<T>(
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
) -> Result<(Vec<T>, u64), DbErr>
where
    T: FromQueryResult, {
    let mut sql = Entity::base_find();

    if let Some(id) = id {
        sql = sql.filter(Column::Id.eq(id));
    }

    if let Some(user_id) = user_id {
        sql = sql.filter(Column::UserId.eq(user_id));
    }

    if let Some(team_id) = team_id {
        match team_id {
            Some(team_id) => sql = sql.filter(Column::TeamId.eq(team_id)),
            None => sql = sql.filter(Column::TeamId.is_null()),
        }
    }

    if let Some(game_id) = game_id {
        match game_id {
            Some(game_id) => sql = sql.filter(Column::GameId.eq(game_id)),
            None => sql = sql.filter(Column::GameId.is_null()),
        }
    }

    if let Some(challenge_id) = challenge_id {
        sql = sql.filter(Column::ChallengeId.eq(challenge_id));
    }

    if let Some(status) = status {
        sql = sql.filter(Column::Status.eq(status));
    }

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

    let total = sql.clone().count(get_db()).await?;

    if let (Some(page), Some(size)) = (page, size) {
        let offset = (page - 1) * size;
        sql = sql.offset(offset).limit(size);
    }

    let submissions = sql.into_model::<T>().all(get_db()).await?;

    Ok((submissions, total))
}

pub async fn find_by_id<T>(submission_id: i64) -> Result<Option<T>, DbErr>
where
    T: FromQueryResult, {
    Ok(Entity::base_find()
        .filter(Column::Id.eq(submission_id))
        .into_model::<T>()
        .one(get_db())
        .await?)
}

pub async fn find_pending_by_id<T>(submission_id: i64) -> Result<Option<T>, DbErr>
where
    T: FromQueryResult, {
    Ok(Entity::base_find()
        .filter(Column::Id.eq(submission_id))
        .filter(Column::Status.eq(Status::Pending))
        .into_model::<T>()
        .one(get_db())
        .await?)
}

pub async fn find_correct_by_team_ids_and_game_id<T>(
    team_ids: Vec<i64>,
    game_id: i64,
) -> Result<Vec<T>, DbErr>
where
    T: FromQueryResult, {
    Ok(Entity::base_find()
        .filter(Column::TeamId.is_in(team_ids))
        .filter(Column::GameId.eq(game_id))
        .filter(Column::Status.eq(Status::Correct))
        .into_model::<T>()
        .all(get_db())
        .await?)
}

pub async fn find_correct_by_challenge_ids_and_optional_team_game<T>(
    challenge_ids: Vec<i64>,
    team_id: Option<i64>,
    game_id: Option<i64>,
) -> Result<Vec<T>, DbErr>
where
    T: FromQueryResult, {
    let mut sql = Entity::base_find().filter(Column::ChallengeId.is_in(challenge_ids));

    if let (Some(_), Some(game_id)) = (team_id, game_id) {
        sql = sql.filter(Column::GameId.eq(game_id));
    } else {
        sql = sql
            .filter(Column::GameId.is_null())
            .filter(Column::TeamId.is_null());
    }

    let submissions = sql
        .filter(Column::Status.eq(Status::Correct))
        .order_by_asc(Column::CreatedAt)
        .into_model::<T>()
        .all(get_db())
        .await?;

    Ok(submissions)
}

pub async fn count() -> Result<u64, DbErr> {
    Ok(Entity::base_find().count(get_db()).await?)
}

pub async fn count_correct() -> Result<u64, DbErr> {
    Ok(Entity::base_find()
        .filter(Column::Status.eq(Status::Correct))
        .count(get_db())
        .await?)
}

pub async fn create<T>(model: ActiveModel) -> Result<T, DbErr>
where
    T: FromQueryResult, {
    let submission = model.insert(get_db()).await?;

    Ok(find_by_id::<T>(submission.id).await?.unwrap())
}

pub async fn update<T>(model: ActiveModel) -> Result<T, DbErr>
where
    T: FromQueryResult, {
    let submission = model.update(get_db()).await?;

    Ok(find_by_id::<T>(submission.id).await?.unwrap())
}

pub async fn delete(submission_id: i64) -> Result<(), DbErr> {
    Entity::delete_by_id(submission_id).exec(get_db()).await?;

    Ok(())
}
