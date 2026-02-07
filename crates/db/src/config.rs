use sea_orm::{ConnectionTrait, EntityTrait, PaginatorTrait, Set, sea_query::OnConflict};

pub use crate::entity::config::{ActiveModel, Config, Model, auth, captcha, email, meta};
pub(crate) use crate::entity::config::{Column, Entity};
use crate::traits::DbError;

pub async fn get(conn: &impl ConnectionTrait) -> Result<Config, DbError> {
    Ok(Entity::find()
        .one(conn)
        .await?
        .ok_or_else(|| DbError::NotFound("config".to_string()))?
        .data)
}

pub async fn count(conn: &impl ConnectionTrait) -> Result<u64, DbError> {
    Ok(Entity::find().count(conn).await?)
}

pub async fn save(conn: &impl ConnectionTrait, config: Config) -> Result<Config, DbError> {
    let model = ActiveModel {
        id: Set(true),
        data: Set(config),
    };

    Entity::insert(model.clone())
        .on_conflict(
            OnConflict::column(Column::Id)
                .update_column(Column::Data)
                .to_owned(),
        )
        .exec(conn)
        .await?;

    Ok(get(conn).await?)
}
