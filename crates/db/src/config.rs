use sea_orm::{ActiveModelTrait, DbErr, EntityTrait, PaginatorTrait, Set};

pub(crate) use crate::entity::config::Entity;
pub use crate::entity::config::{ActiveModel, Model, auth, captcha, email, meta};
use crate::get_db;

pub async fn get() -> Result<Model, DbErr> {
    Ok(Entity::find().one(get_db()).await?.unwrap())
}

pub async fn count() -> Result<u64, DbErr> {
    Ok(Entity::find().count(get_db()).await?)
}

pub async fn save(mut model: ActiveModel) -> Result<Model, DbErr> {
    model.id = Set(1);
    let _ = model.save(get_db()).await?;

    Ok(get().await?)
}
