use sea_orm::{ActiveModelTrait, ConnectionTrait, EntityTrait, NotSet, PaginatorTrait, Unchanged};

pub(crate) use crate::entity::config::Entity;
pub use crate::entity::config::{ActiveModel, Model, auth, captcha, email, meta};
use crate::traits::DbError;

pub async fn get(conn: &impl ConnectionTrait) -> Result<Model, DbError> {
    Ok(Entity::find()
        .one(conn)
        .await?
        .ok_or_else(|| DbError::NotFound("config".to_string()))?)
}

pub async fn count(conn: &impl ConnectionTrait) -> Result<u64, DbError> {
    Ok(Entity::find().count(conn).await?)
}

pub async fn save(conn: &impl ConnectionTrait, mut model: ActiveModel) -> Result<Model, DbError> {
    model.id = if count(conn).await? == 0 {
        NotSet
    } else {
        Unchanged(1)
    };
    let _ = model.save(conn).await?;

    Ok(get(conn).await?)
}
