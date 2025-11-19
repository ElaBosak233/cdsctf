use sea_orm::{ActiveModelTrait, EntityTrait, NotSet, PaginatorTrait, Unchanged};

pub(crate) use crate::entity::config::Entity;
pub use crate::entity::config::{ActiveModel, Model, auth, captcha, email, meta};
use crate::{get_db, traits::DbError};

pub async fn get() -> Result<Model, DbError> {
    Ok(Entity::find().one(get_db()).await?.unwrap())
}

pub async fn count() -> Result<u64, DbError> {
    Ok(Entity::find().count(get_db()).await?)
}

pub async fn save(mut model: ActiveModel) -> Result<Model, DbError> {
    model.id = if count().await? == 0 {
        NotSet
    } else {
        Unchanged(1)
    };
    let _ = model.save(get_db()).await?;

    Ok(get().await?)
}
