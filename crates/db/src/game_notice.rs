use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, FromQueryResult, QueryFilter};
use serde::{Deserialize, Serialize};

pub use crate::entity::game_notice::{ActiveModel, Model};
pub(crate) use crate::entity::game_notice::{Column, Entity};
use crate::{get_db, traits::DbError};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromQueryResult)]
pub struct GameNotice {
    pub id: i64,
    pub game_id: i64,
    pub title: String,
    pub content: String,
    pub created_at: i64,
}

pub async fn find_by_id<T>(notice_id: i64, game_id: i64) -> Result<Option<T>, DbError>
where
    T: FromQueryResult, {
    Ok(Entity::find_by_id(notice_id)
        .filter(Column::GameId.eq(game_id))
        .into_model::<T>()
        .one(get_db())
        .await?)
}

pub async fn find_by_game_id<T>(game_id: i64) -> Result<Vec<T>, DbError>
where
    T: FromQueryResult, {
    Ok(Entity::find()
        .filter(Column::GameId.eq(game_id))
        .into_model::<T>()
        .all(get_db())
        .await?)
}

pub async fn create<T>(model: ActiveModel) -> Result<T, DbError>
where
    T: FromQueryResult, {
    let game_notice = model.insert(get_db()).await?;

    Ok(find_by_id::<T>(game_notice.id, game_notice.game_id)
        .await?
        .unwrap())
}

pub async fn delete(notice_id: i64, game_id: i64) -> Result<(), DbError> {
    Entity::delete_many()
        .filter(Column::Id.eq(notice_id))
        .filter(Column::GameId.eq(game_id))
        .exec(get_db())
        .await?;

    Ok(())
}
