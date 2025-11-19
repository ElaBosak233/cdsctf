use std::fmt::Display;

use anyhow::anyhow;
use fred::{
    prelude::{Client, ClientLike, KeysInterface},
    types::{Expiration, Key, config::Config},
};
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tracing::info;
use traits::CacheError;

pub mod traits;

static CLIENT: OnceCell<Client> = OnceCell::new();

pub fn get_client() -> Client {
    CLIENT
        .get()
        .expect("No cache client instance, forget to init?")
        .clone()
}

pub async fn init() -> Result<(), CacheError> {
    let config = Config::from_url(&cds_env::get_config().cache.url)?;
    let client = Client::new(config, None, None, None);
    client.init().await?;

    CLIENT
        .set(client)
        .map_err(|_| anyhow!("Failed to set client into OnceCell."))?;
    info!("Cache initialized successfully.");

    Ok(())
}

pub async fn get<T>(key: impl Into<Key> + Send + Display) -> Result<Option<T>, CacheError>
where
    T: for<'de> Deserialize<'de>, {
    let result = get_client().get::<Option<Value>, _>(key).await?;
    match result {
        Some(value) => Ok(Some(serde_json::from_value(value)?)),
        None => Ok(None),
    }
}

pub async fn get_del<T>(key: impl Into<Key> + Send + Display) -> Result<Option<T>, CacheError>
where
    T: for<'de> Deserialize<'de>, {
    let result = get_client().getdel::<Option<Value>, _>(key).await?;
    match result {
        Some(value) => Ok(Some(serde_json::from_value(value)?)),
        None => Ok(None),
    }
}

pub async fn set(
    key: impl Into<Key> + Send + Display,
    value: impl Serialize + Send,
) -> Result<(), CacheError> {
    let value: String = serde_json::to_string(&value)?;
    get_client()
        .set::<(), _, _>(key, value, None, None, false)
        .await?;

    Ok(())
}

pub async fn set_ex(
    key: impl Into<Key> + Send + Display,
    value: impl Serialize + Send,
    expire: u64,
) -> Result<(), CacheError> {
    let value: String = serde_json::to_string(&value)?;
    get_client()
        .set::<(), _, _>(key, value, Some(Expiration::EX(expire as i64)), None, false)
        .await?;

    Ok(())
}

pub async fn incr(key: impl Into<Key> + Send + Display) -> Result<i64, CacheError> {
    let result = get_client().incr(key).await?;

    Ok(result)
}

pub async fn exists(key: impl Into<Key> + Send + Display) -> Result<bool, CacheError> {
    let result = get_client().exists(key).await?;

    Ok(result)
}

pub async fn flush() -> Result<(), CacheError> {
    get_client().flushall::<()>(false).await?;

    Ok(())
}
