//! Redis client wrapper (crate `fred`): JSON-serialized values for generic
//! cache operations.
//!
//! Typical uses in CdsCTF: HTTP session storage, distributed counters, and
//! short-lived cached data. Keys are Redis keys; values are JSON via
//! `serde_json`.

use std::fmt::Display;

use fred::{
    prelude::{Client, ClientLike, KeysInterface},
    types::{Expiration, Key, config::Config},
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tracing::info;
use traits::CacheError;

/// Defines the `session` submodule (see sibling `*.rs` files).
pub mod session;

/// Defines the `traits` submodule (see sibling `*.rs` files).
pub mod traits;

/// Connected Redis client handle; clone to share the same connection pool.
#[derive(Debug, Clone)]
pub struct Cache {
    pub client: Client,
}

/// Connects to Redis using `env.cache.url` and initializes the client.
pub async fn init(env: &cds_env::Env) -> Result<Cache, CacheError> {
    let config = Config::from_url(&env.cache.url)?;
    let client = Client::new(config, None, None, None);
    client.init().await?;
    info!("Cache initialized successfully.");

    Ok(Cache { client })
}

impl Cache {
    /// Reads JSON at `key` and deserializes into `T`, or returns `Ok(None)` if
    /// the key is absent.
    pub async fn get<T>(
        &self,
        key: impl Into<Key> + Send + Display,
    ) -> Result<Option<T>, CacheError>
    where
        T: for<'de> Deserialize<'de>, {
        let result = self.client.get::<Option<Value>, _>(key).await?;
        match result {
            Some(value) => Ok(Some(serde_json::from_value(value)?)),
            None => Ok(None),
        }
    }

    /// Like [`get`](Self::get) but **deletes** the key atomically when present
    /// (Redis `GETDEL`).
    pub async fn get_del<T>(
        &self,
        key: impl Into<Key> + Send + Display,
    ) -> Result<Option<T>, CacheError>
    where
        T: for<'de> Deserialize<'de>, {
        let result = self.client.getdel::<Option<Value>, _>(key).await?;
        match result {
            Some(value) => Ok(Some(serde_json::from_value(value)?)),
            None => Ok(None),
        }
    }

    /// Stores `value` as JSON at `key` with no TTL.
    pub async fn set(
        &self,
        key: impl Into<Key> + Send + Display,
        value: impl Serialize + Send,
    ) -> Result<(), CacheError> {
        let value: String = serde_json::to_string(&value)?;
        self.client
            .set::<(), _, _>(key, value, None, None, false)
            .await?;

        Ok(())
    }

    /// Stores JSON with an **expiration** in seconds (`EX`).
    pub async fn set_ex(
        &self,
        key: impl Into<Key> + Send + Display,
        value: impl Serialize + Send,
        expire: u64,
    ) -> Result<(), CacheError> {
        let value: String = serde_json::to_string(&value)?;
        self.client
            .set::<(), _, _>(key, value, Some(Expiration::EX(expire as i64)), None, false)
            .await?;

        Ok(())
    }

    /// Increments a numeric string key by one (creates `1` if missing).
    pub async fn incr(&self, key: impl Into<Key> + Send + Display) -> Result<i64, CacheError> {
        let result = self.client.incr(key).await?;

        Ok(result)
    }

    /// Returns whether **any** key exists among the resolved Redis key(s).
    pub async fn exists(&self, key: impl Into<Key> + Send + Display) -> Result<bool, CacheError> {
        let result = self.client.exists(key).await?;

        Ok(result)
    }

    /// **Dangerous**: wipes the current Redis database (`FLUSHALL`); intended
    /// for tests/admin tools.
    pub async fn flush(&self) -> Result<(), CacheError> {
        self.client.flushall::<()>(false).await?;

        Ok(())
    }
}
