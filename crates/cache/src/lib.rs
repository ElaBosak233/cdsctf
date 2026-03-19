use std::fmt::Display;

use fred::{
    prelude::{Client, ClientLike, KeysInterface},
    types::{Expiration, Key, config::Config},
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tracing::info;
use traits::CacheError;

pub mod session;
pub mod traits;

#[derive(Debug, Clone)]
pub struct Cache {
    pub client: Client,
}

pub async fn init(env: &cds_env::Env) -> Result<Cache, CacheError> {
    let config = Config::from_url(&env.cache.url)?;
    let client = Client::new(config, None, None, None);
    client.init().await?;
    info!("Cache initialized successfully.");

    Ok(Cache { client })
}

impl Cache {
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

    pub async fn incr(&self, key: impl Into<Key> + Send + Display) -> Result<i64, CacheError> {
        let result = self.client.incr(key).await?;

        Ok(result)
    }

    pub async fn exists(&self, key: impl Into<Key> + Send + Display) -> Result<bool, CacheError> {
        let result = self.client.exists(key).await?;

        Ok(result)
    }

    pub async fn flush(&self) -> Result<(), CacheError> {
        self.client.flushall::<()>(false).await?;

        Ok(())
    }
}
