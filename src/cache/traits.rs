use fred::error::RedisError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CacheError {
    #[error("redis error: {0}")]
    RedisError(#[from] RedisError),
    #[error("serde_json error: {0}")]
    SerdeJsonError(#[from] serde_json::Error),
}
