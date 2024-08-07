use std::fmt::Display;

use fred::{
    prelude::{ClientLike, KeysInterface, RedisClient},
    types::{Expiration, RedisConfig, RedisKey},
};
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tracing::info;
use traits::CacheError;

pub mod traits;

static CLIENT: OnceCell<RedisClient> = OnceCell::new();

fn get_client() -> &'static RedisClient {
    return CLIENT.get().unwrap();
}

pub async fn get<T>(key: impl Into<RedisKey> + Send + Display) -> Result<Option<T>, CacheError>
where
    T: for<'de> Deserialize<'de>,
{
    let result = get_client().get::<Option<Value>, _>(key).await?;
    match result {
        Some(value) => Ok(Some(serde_json::from_value(value)?)),
        None => Ok(None),
    }
}

pub async fn getdel<T>(key: impl Into<RedisKey> + Send + Display) -> Result<Option<T>, CacheError>
where
    T: for<'de> Deserialize<'de>,
{
    let result = get_client().getdel::<Option<Value>, _>(key).await?;
    match result {
        Some(value) => return Ok(Some(serde_json::from_value(value)?)),
        None => return Ok(None),
    }
}

pub async fn set(
    key: impl Into<RedisKey> + Send + Display, value: impl Serialize + Send,
) -> Result<(), CacheError> {
    let value = serde_json::to_string(&value)?;
    get_client().set(key, value, None, None, false).await?;

    return Ok(());
}

pub async fn set_ex(
    key: impl Into<RedisKey> + Send + Display, value: impl Serialize + Send, expire: u64,
) -> Result<(), CacheError> {
    let value = serde_json::to_string(&value)?;
    get_client()
        .set(key, value, Some(Expiration::EX(expire as i64)), None, false)
        .await?;

    return Ok(());
}

pub async fn flush() -> Result<(), CacheError> {
    get_client().flushall(false).await?;

    return Ok(());
}

pub async fn init() {
    let config = RedisConfig::from_url(&crate::config::get_config().cache.url).unwrap();
    let client = RedisClient::new(config, None, None, None);
    client.init().await.unwrap();

    CLIENT.set(client).unwrap();
    info!("Cache initialized successfully.");
}
