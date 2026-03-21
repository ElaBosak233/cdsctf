//! Redis cache helpers — `session`.

use std::fmt::Debug;

use async_trait::async_trait;
use fred::{
    clients::Client,
    prelude::KeysInterface,
    types::{Expiration, SetOptions},
};
use time::OffsetDateTime;
use tower_sessions_core::{
    SessionStore,
    session::{Id, Record},
    session_store,
};

use crate::Cache;

#[derive(Debug, thiserror::Error)]
pub enum RedisStoreError {
    #[error(transparent)]
    Redis(#[from] fred::error::Error),

    #[error(transparent)]
    Decode(#[from] rmp_serde::decode::Error),

    #[error(transparent)]
    Encode(#[from] rmp_serde::encode::Error),
}

impl From<RedisStoreError> for session_store::Error {
    /// Converts from the input into `Self`.
    fn from(err: RedisStoreError) -> Self {
        match err {
            RedisStoreError::Redis(inner) => session_store::Error::Backend(inner.to_string()),
            RedisStoreError::Decode(inner) => session_store::Error::Decode(inner.to_string()),
            RedisStoreError::Encode(inner) => session_store::Error::Encode(inner.to_string()),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct RedisStore {
    client: Client,
    prefix: Option<String>,
}

impl RedisStore {
    /// Constructs a new value.
    pub fn new(cache: Cache) -> Self {
        Self {
            client: cache.client.clone(),
            prefix: None,
        }
    }

    /// Prefixes a logical key with the configured bucket prefix.
    pub fn with_prefix(cache: Cache, prefix: String) -> Self {
        Self {
            client: cache.client.clone(),
            prefix: Some(prefix),
        }
    }

    /// Returns key.

    fn get_key(&self, id: &Id) -> String {
        if let Some(prefix) = &self.prefix {
            format!("{}{}", prefix, id)
        } else {
            id.to_string()
        }
    }

    /// Persists a session record with Redis `SET` options.
    async fn save_with_options(
        &self,
        record: &Record,
        options: Option<SetOptions>,
    ) -> session_store::Result<bool> {
        let expire = Some(Expiration::EXAT(OffsetDateTime::unix_timestamp(
            record.expiry_date,
        )));

        Ok(self
            .client
            .set(
                self.get_key(&record.id),
                rmp_serde::to_vec(&record)
                    .map_err(RedisStoreError::Encode)?
                    .as_slice(),
                expire,
                options,
                false,
            )
            .await
            .map_err(RedisStoreError::Redis)?)
    }
}

#[async_trait]
impl SessionStore for RedisStore {
    /// Inserts a new row and returns the persisted model.
    async fn create(&self, record: &mut Record) -> session_store::Result<()> {
        loop {
            if !self.save_with_options(record, Some(SetOptions::NX)).await? {
                record.id = Id::default();
                continue;
            }
            break;
        }
        Ok(())
    }

    /// Persists the current value to the backing store.
    async fn save(&self, record: &Record) -> session_store::Result<()> {
        self.save_with_options(record, Some(SetOptions::XX)).await?;
        Ok(())
    }

    /// Loads a value from the backing store.
    async fn load(&self, session_id: &Id) -> session_store::Result<Option<Record>> {
        let data = self
            .client
            .get::<Option<Vec<u8>>, _>(self.get_key(session_id))
            .await
            .map_err(RedisStoreError::Redis)?;

        if let Some(data) = data {
            Ok(Some(
                rmp_serde::from_slice(&data).map_err(RedisStoreError::Decode)?,
            ))
        } else {
            Ok(None)
        }
    }

    /// Deletes rows matching the provided identifier or filter.
    async fn delete(&self, session_id: &Id) -> session_store::Result<()> {
        let _: () = self
            .client
            .del(self.get_key(session_id))
            .await
            .map_err(RedisStoreError::Redis)?;
        Ok(())
    }
}
