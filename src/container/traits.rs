use async_trait::async_trait;
use std::error::Error;

#[async_trait]
pub trait Container: Send + Sync {
    async fn init(&self);
    async fn create(
        &self, name: String, challenge: crate::model::challenge::Model,
        injected_flag: crate::model::challenge::Flag,
    ) -> Result<Vec<crate::model::pod::Nat>, Box<dyn Error>>;
    async fn delete(&self, name: String);
}
