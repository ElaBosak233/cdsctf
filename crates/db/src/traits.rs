use async_trait::async_trait;
use sea_orm::{ConnectionTrait, DbErr};

#[async_trait]
pub trait EagerLoading<T> {
    async fn eager_load<C>(self, db: &C) -> Result<T, DbErr>
    where 
        C: ConnectionTrait;
}