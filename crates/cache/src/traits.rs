use thiserror::Error;

#[derive(Debug, Error)]
pub enum CacheError {
    #[error("no instance")]
    NoInstance,
    #[error("redis error: {0}")]
    RedisError(#[from] fred::error::Error),
    #[error("serde_json error: {0}")]
    SerdeJsonError(#[from] serde_json::Error),
    #[error("env error")]
    EnvError(#[from] cds_env::traits::EnvError),
    #[error("other error: {0}")]
    OtherError(#[from] anyhow::Error),
}
