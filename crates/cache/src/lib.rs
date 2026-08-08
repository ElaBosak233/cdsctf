//! Valkey-backed application cache.
//!
//! Values use JSON to remain compatible with the previous `fred` implementation
//! and to keep cache data inspectable. Counter and rate-limit keys are numeric
//! strings so Valkey can update them atomically.

use std::{sync::Arc, time::Duration};

use redis::{
    AsyncCommands, ExistenceCheck, FromRedisValue, SetExpiry, SetOptions,
    aio::{ConnectionManager, ConnectionManagerConfig},
};
use serde::{Serialize, de::DeserializeOwned};
use tracing::info;
use traits::CacheError;

pub mod session;
pub mod traits;

/// Re-exported for advanced Valkey features which intentionally stay outside
/// the typed cache API (streams, Pub/Sub, modules, Cluster and Sentinel).
pub use redis;

const FIXED_WINDOW_SCRIPT: &str = r#"
local current = tonumber(redis.call('GET', KEYS[1]))
local window = tonumber(ARGV[1])
local limit = tonumber(ARGV[2])

if current == nil then
  redis.call('SET', KEYS[1], 1, 'PX', window)
  return {1, window, 1}
end

local ttl = redis.call('PTTL', KEYS[1])
if ttl < 0 then
  redis.call('PEXPIRE', KEYS[1], window)
  ttl = window
end

if current >= limit then
  return {current, ttl, 0}
end

current = redis.call('INCR', KEYS[1])
return {current, ttl, 1}
"#;

#[derive(Debug)]
struct Inner {
    client: redis::Client,
    connection: ConnectionManager,
    key_prefix: String,
}

/// Cloneable cache handle backed by an automatically reconnecting multiplexed
/// connection.
#[derive(Debug, Clone)]
pub struct Cache {
    inner: Arc<Inner>,
}

/// Result of an atomic fixed-window rate-limit decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RateLimit {
    pub allowed: bool,
    pub used: u64,
    pub limit: u64,
    pub remaining: u64,
    pub retry_after: Duration,
}

/// Connects to the configured Valkey-compatible endpoint and verifies it with
/// `PING` before returning.
pub async fn init(env: &cds_env::Env) -> Result<Cache, CacheError> {
    let config = &env.cache;
    let cache = Cache::connect(
        &config.url,
        &config.key_prefix,
        Duration::from_millis(config.connection_timeout_ms),
        Duration::from_millis(config.response_timeout_ms),
        config.max_in_flight,
    )
    .await?;
    cache.ping().await?;
    info!(key_prefix = %config.key_prefix, "Valkey cache initialized");
    Ok(cache)
}

impl Cache {
    /// Builds a cache independently of the application environment. This is
    /// useful for tests and small tools.
    pub async fn connect(
        url: &str,
        key_prefix: &str,
        connection_timeout: Duration,
        response_timeout: Duration,
        max_in_flight: usize,
    ) -> Result<Self, CacheError> {
        let client = redis::Client::open(url)?;
        let manager_config = ConnectionManagerConfig::new()
            .set_connection_timeout(Some(connection_timeout))
            .set_response_timeout(Some(response_timeout))
            .set_max_delay(Duration::from_secs(2))
            .set_number_of_retries(8)
            .set_concurrency_limit(max_in_flight.max(1));
        let connection = client
            .get_connection_manager_with_config(manager_config)
            .await?;

        Ok(Self {
            inner: Arc::new(Inner {
                client,
                connection,
                key_prefix: normalize_prefix(key_prefix),
            }),
        })
    }

    /// Returns the single-node client for dedicated Pub/Sub or blocking stream
    /// connections. Cluster and Sentinel clients can be built through the
    /// re-exported [`redis`] crate.
    pub fn client(&self) -> redis::Client {
        self.inner.client.clone()
    }

    /// Returns a clone of the shared multiplexed connection for pipelining or
    /// commands not yet represented by this wrapper.
    pub fn connection(&self) -> ConnectionManager {
        self.inner.connection.clone()
    }

    /// Resolves an application key into the configured namespace.
    pub fn key(&self, key: impl AsRef<str>) -> String {
        format!("{}{}", self.inner.key_prefix, key.as_ref())
    }

    /// Creates a handle with an additional namespace while sharing the same
    /// connection.
    pub fn scoped(&self, scope: impl AsRef<str>) -> Self {
        let scope = normalize_prefix(scope.as_ref());
        Self {
            inner: Arc::new(Inner {
                client: self.inner.client.clone(),
                connection: self.inner.connection.clone(),
                key_prefix: format!("{}{}", self.inner.key_prefix, scope),
            }),
        }
    }

    pub async fn ping(&self) -> Result<(), CacheError> {
        let mut connection = self.connection();
        let response: String = connection.ping().await?;
        if response == "PONG" {
            Ok(())
        } else {
            Err(redis::RedisError::from((
                redis::ErrorKind::UnexpectedReturnType,
                "unexpected PING response",
                response,
            ))
            .into())
        }
    }

    /// Reads and deserializes a JSON value, returning `None` for a cache miss.
    pub async fn get<T: DeserializeOwned>(
        &self,
        key: impl AsRef<str>,
    ) -> Result<Option<T>, CacheError> {
        let mut connection = self.connection();
        let value: Option<Vec<u8>> = connection.get(self.key(key)).await?;
        value
            .map(|bytes| serde_json::from_slice(&bytes))
            .transpose()
            .map_err(Into::into)
    }

    /// Atomically reads and deletes a JSON value (`GETDEL`).
    pub async fn take<T: DeserializeOwned>(
        &self,
        key: impl AsRef<str>,
    ) -> Result<Option<T>, CacheError> {
        let mut connection = self.connection();
        let value: Option<Vec<u8>> = connection.get_del(self.key(key)).await?;
        value
            .map(|bytes| serde_json::from_slice(&bytes))
            .transpose()
            .map_err(Into::into)
    }

    /// Stores a JSON value without expiry. Prefer
    /// [`set_with_ttl`](Self::set_with_ttl) for ordinary cache data.
    pub async fn set<T: Serialize>(
        &self,
        key: impl AsRef<str>,
        value: T,
    ) -> Result<(), CacheError> {
        let mut connection = self.connection();
        let payload = serde_json::to_vec(&value)?;
        let _: () = connection.set(self.key(key), payload).await?;
        Ok(())
    }

    /// Stores a JSON value with millisecond TTL precision.
    pub async fn set_with_ttl<T: Serialize>(
        &self,
        key: impl AsRef<str>,
        value: T,
        ttl: Duration,
    ) -> Result<(), CacheError> {
        let ttl_ms = duration_ms(ttl)?;
        let mut connection = self.connection();
        let payload = serde_json::to_vec(&value)?;
        let _: () = connection.pset_ex(self.key(key), payload, ttl_ms).await?;
        Ok(())
    }

    /// Stores a value only when the key does not exist. The value and TTL are
    /// committed by one atomic `SET NX PX` command.
    pub async fn set_if_absent<T: Serialize>(
        &self,
        key: impl AsRef<str>,
        value: T,
        ttl: Duration,
    ) -> Result<bool, CacheError> {
        let ttl_ms = duration_ms(ttl)?;
        let options = SetOptions::default()
            .conditional_set(ExistenceCheck::NX)
            .with_expiration(SetExpiry::PX(ttl_ms));
        let mut connection = self.connection();
        let result: Option<String> = connection
            .set_options(self.key(key), serde_json::to_vec(&value)?, options)
            .await?;
        Ok(result.is_some())
    }

    /// Removes a key asynchronously on the server (`UNLINK`).
    pub async fn delete(&self, key: impl AsRef<str>) -> Result<bool, CacheError> {
        let mut connection = self.connection();
        let deleted: usize = connection.unlink(self.key(key)).await?;
        Ok(deleted != 0)
    }

    pub async fn exists(&self, key: impl AsRef<str>) -> Result<bool, CacheError> {
        let mut connection = self.connection();
        Ok(connection.exists(self.key(key)).await?)
    }

    /// Applies an atomic fixed-window limit without refreshing the window on
    /// each request. Exactly `limit` calls are admitted per window.
    pub async fn fixed_window(
        &self,
        key: impl AsRef<str>,
        limit: u64,
        window: Duration,
    ) -> Result<RateLimit, CacheError> {
        if limit == 0 {
            return Err(CacheError::InvalidLimit);
        }
        let window_ms = duration_ms(window)?;
        let mut connection = self.connection();
        let (used, retry_after_ms, allowed): (u64, u64, u8) =
            redis::Script::new(FIXED_WINDOW_SCRIPT)
                .key(self.key(key))
                .arg(window_ms)
                .arg(limit)
                .invoke_async(&mut connection)
                .await?;

        Ok(RateLimit {
            allowed: allowed == 1,
            used,
            limit,
            remaining: limit.saturating_sub(used),
            retry_after: Duration::from_millis(retry_after_ms),
        })
    }

    /// Deletes only keys owned by this cache namespace using incremental
    /// `SCAN` plus `UNLINK`. It never issues `FLUSHALL` or blocks on `KEYS`.
    pub async fn clear_namespace(&self) -> Result<u64, CacheError> {
        if self.inner.key_prefix.is_empty() {
            return Err(CacheError::MissingNamespace);
        }
        let mut connection = self.connection();
        let pattern = format!("{}*", glob_escape(&self.inner.key_prefix));
        let mut cursor = 0_u64;
        let mut deleted = 0_u64;

        loop {
            let (next_cursor, keys): (u64, Vec<String>) = redis::cmd("SCAN")
                .arg(cursor)
                .arg("MATCH")
                .arg(&pattern)
                .arg("COUNT")
                .arg(256)
                .query_async(&mut connection)
                .await?;
            if !keys.is_empty() {
                deleted += connection.unlink::<_, usize>(keys).await? as u64;
            }
            cursor = next_cursor;
            if cursor == 0 {
                break;
            }
        }

        Ok(deleted)
    }

    /// Runs an arbitrary command on the shared connection. Keys are not
    /// automatically namespaced; call [`key`](Self::key) explicitly.
    pub async fn query<T: FromRedisValue>(
        &self,
        command: &mut redis::Cmd,
    ) -> Result<T, CacheError> {
        let mut connection = self.connection();
        Ok(command.query_async(&mut connection).await?)
    }
}

fn normalize_prefix(prefix: &str) -> String {
    let prefix = prefix.trim_matches(':');
    if prefix.is_empty() {
        String::new()
    } else {
        format!("{prefix}:")
    }
}

fn duration_ms(duration: Duration) -> Result<u64, CacheError> {
    u64::try_from(duration.as_millis())
        .ok()
        .filter(|millis| *millis > 0)
        .ok_or(CacheError::InvalidTtl)
}

fn glob_escape(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());
    for character in value.chars() {
        if matches!(character, '*' | '?' | '[' | ']' | '\\') {
            escaped.push('\\');
        }
        escaped.push(character);
    }
    escaped
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::{duration_ms, glob_escape, normalize_prefix};

    #[test]
    fn normalizes_key_prefixes() {
        assert_eq!(normalize_prefix("cdsctf"), "cdsctf:");
        assert_eq!(normalize_prefix(":cdsctf:"), "cdsctf:");
        assert_eq!(normalize_prefix(""), "");
    }

    #[test]
    fn validates_ttl_precision() {
        assert_eq!(duration_ms(Duration::from_millis(1)).unwrap(), 1);
        assert!(duration_ms(Duration::ZERO).is_err());
        assert!(duration_ms(Duration::from_nanos(1)).is_err());
    }

    #[test]
    fn escapes_namespace_globs() {
        assert_eq!(glob_escape("tenant[*]?:"), "tenant\\[\\*\\]\\?:");
    }
}
