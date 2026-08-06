//! Shared traits and error types for the `cache` crate.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum CacheError {
    #[error("valkey command failed: {0}")]
    Valkey(#[from] redis::RedisError),

    #[error("cache value serialization failed: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("cache TTL must be greater than zero")]
    InvalidTtl,

    #[error("rate limit must be greater than zero")]
    InvalidLimit,

    #[error("clearing cache keys requires a non-empty namespace")]
    MissingNamespace,
}
