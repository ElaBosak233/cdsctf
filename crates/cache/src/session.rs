//! `tower-sessions` store backed by the cache connection manager.

use async_trait::async_trait;
use redis::{AsyncCommands, ExistenceCheck, SetExpiry, SetOptions, aio::ConnectionManager};
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
    Redis(#[from] redis::RedisError),

    #[error(transparent)]
    Decode(#[from] rmp_serde::decode::Error),

    #[error(transparent)]
    Encode(#[from] rmp_serde::encode::Error),

    #[error("session expiry is before the Unix epoch")]
    InvalidExpiry,
}

impl From<RedisStoreError> for session_store::Error {
    fn from(err: RedisStoreError) -> Self {
        match err {
            RedisStoreError::Redis(inner) => session_store::Error::Backend(inner.to_string()),
            RedisStoreError::Decode(inner) => session_store::Error::Decode(inner.to_string()),
            RedisStoreError::Encode(inner) => session_store::Error::Encode(inner.to_string()),
            RedisStoreError::InvalidExpiry => session_store::Error::Backend(err.to_string()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct RedisStore {
    connection: ConnectionManager,
    prefix: String,
}

impl RedisStore {
    pub fn new(cache: Cache) -> Self {
        Self::with_prefix(cache, "session")
    }

    pub fn with_prefix(cache: Cache, prefix: impl AsRef<str>) -> Self {
        let scope = cache.scoped(prefix);
        Self {
            connection: scope.connection(),
            prefix: scope.key(""),
        }
    }

    fn key(&self, id: &Id) -> String {
        format!("{}{}", self.prefix, id)
    }

    async fn save_with_options(
        &self,
        record: &Record,
        existence: Option<ExistenceCheck>,
    ) -> Result<bool, RedisStoreError> {
        let timestamp = OffsetDateTime::unix_timestamp(record.expiry_date);
        let timestamp = u64::try_from(timestamp).map_err(|_| RedisStoreError::InvalidExpiry)?;
        let mut options = SetOptions::default().with_expiration(SetExpiry::EXAT(timestamp));
        if let Some(existence) = existence {
            options = options.conditional_set(existence);
        }

        let mut connection = self.connection.clone();
        let result: Option<String> = connection
            .set_options(self.key(&record.id), rmp_serde::to_vec(record)?, options)
            .await?;
        Ok(result.is_some())
    }
}

#[async_trait]
impl SessionStore for RedisStore {
    async fn create(&self, record: &mut Record) -> session_store::Result<()> {
        loop {
            if !self
                .save_with_options(record, Some(ExistenceCheck::NX))
                .await?
            {
                record.id = Id::default();
                continue;
            }
            return Ok(());
        }
    }

    async fn save(&self, record: &Record) -> session_store::Result<()> {
        self.save_with_options(record, Some(ExistenceCheck::XX))
            .await?;
        Ok(())
    }

    async fn load(&self, session_id: &Id) -> session_store::Result<Option<Record>> {
        let mut connection = self.connection.clone();
        let data: Option<Vec<u8>> = connection
            .get(self.key(session_id))
            .await
            .map_err(RedisStoreError::Redis)?;
        data.map(|data| rmp_serde::from_slice(&data).map_err(RedisStoreError::Decode))
            .transpose()
            .map_err(Into::into)
    }

    async fn delete(&self, session_id: &Id) -> session_store::Result<()> {
        let mut connection = self.connection.clone();
        connection
            .unlink::<_, usize>(self.key(session_id))
            .await
            .map_err(RedisStoreError::Redis)?;
        Ok(())
    }
}
