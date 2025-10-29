use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, FromQueryResult, PaginatorTrait, QueryFilter,
};
use serde::{Deserialize, Serialize};

pub(crate) use crate::entity::game_challenge::Entity;
pub use crate::entity::game_challenge::{ActiveModel, Column, Model, Relation};
use crate::get_db;

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromQueryResult)]
pub struct GameChallenge {
    pub game_id: i64,
    pub challenge_id: i64,
    pub challenge_title: String,
    pub challenge_category: i32,
    pub difficulty: i64,
    pub bonus_ratios: Vec<i64>,
    pub max_pts: i64,
    pub min_pts: i64,
    pub pts: i64,
    pub is_enabled: bool,
    pub frozen_at: Option<i64>,
}

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromQueryResult)]
pub struct GameChallengeMini {
    pub game_id: i64,
    pub challenge_id: i64,
    pub challenge_title: String,
    pub challenge_category: i32,
    pub pts: i64,
    pub frozen_at: Option<i64>,
}

#[derive(Clone, Debug, Default)]
pub struct FindGameChallengeOptions {
    pub game_id: Option<i64>,
    pub challenge_id: Option<i64>,
    pub is_enabled: Option<bool>,
    pub category: Option<i32>,
}

pub async fn find<T>(
    FindGameChallengeOptions {
        game_id,
        challenge_id,
        is_enabled,
        category,
    }: FindGameChallengeOptions,
) -> Result<(Vec<T>, u64), DbErr>
where
    T: FromQueryResult,
{
    // Using inner join to access fields in related tables.
    let mut sql = Entity::base_find();

    sql = sql.filter(Column::GameId.eq(game_id));

    if let Some(challenge_id) = challenge_id {
        sql = sql.filter(Column::ChallengeId.eq(challenge_id));
    }

    if let Some(is_enabled) = is_enabled {
        sql = sql.filter(Column::IsEnabled.eq(is_enabled));
    }

    if let Some(category) = category {
        sql = sql.filter(super::challenge::Column::Category.eq(category));
    }

    let total = sql.clone().count(get_db()).await?;

    let game_challenges = sql.into_model::<T>().all(get_db()).await?;

    Ok((game_challenges, total))
}

pub async fn find_by_id<T>(game_id: i64, challenge_id: i64) -> Result<Option<T>, DbErr>
where
    T: FromQueryResult,
{
    Ok(Entity::base_find()
        .filter(Column::GameId.eq(game_id))
        .filter(Column::ChallengeId.eq(challenge_id))
        .into_model::<T>()
        .one(get_db())
        .await?)
}

pub async fn count() -> Result<u64, DbErr> {
    Ok(Entity::find().count(get_db()).await?)
}

pub async fn create<T>(model: ActiveModel) -> Result<T, DbErr>
where
    T: FromQueryResult,
{
    let game_challenge = model.insert(get_db()).await?;

    Ok(
        find_by_id::<T>(game_challenge.game_id, game_challenge.challenge_id)
            .await?
            .unwrap(),
    )
}

pub async fn update<T>(model: ActiveModel) -> Result<T, DbErr>
where
    T: FromQueryResult,
{
    let game_challenge = model.update(get_db()).await?;

    Ok(
        find_by_id::<T>(game_challenge.game_id, game_challenge.challenge_id)
            .await?
            .unwrap(),
    )
}

pub async fn delete(game_id: i64, challenge_id: i64) -> Result<(), DbErr> {
    Entity::delete_many()
        .filter(Column::GameId.eq(game_id))
        .filter(Column::ChallengeId.eq(challenge_id))
        .exec(get_db())
        .await?;

    Ok(())
}
