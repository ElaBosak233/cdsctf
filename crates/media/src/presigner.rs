//! Presigned URL generator.
//! Only present when [media.presigned](cds_env::media::Config::presigned) is
//! true. If [presigned_endpoint](cds_env::media::Config::presigned_endpoint) is
//! set, URLs are signed with that endpoint (typically public); otherwise the
//! main bucket endpoint is used.

use std::{collections::HashMap, sync::Arc};

use s3::bucket::Bucket;

use crate::traits::MediaError;

/// Reuses a bucket instance for signing only; used solely to generate presigned
/// URLs.
#[derive(Clone)]
pub struct Presigner(Arc<Bucket>);

impl Presigner {
    pub(crate) fn new(bucket: Arc<Bucket>) -> Self {
        Self(bucket)
    }

    /// Generates a presigned GET URL (for downloads).
    pub async fn presign_get(
        &self,
        key: &str,
        expiry_secs: u32,
        response_content_disposition: Option<&str>,
    ) -> Result<String, MediaError> {
        let custom_queries = response_content_disposition.map(|v| {
            let mut m = HashMap::new();
            m.insert("response-content-disposition".to_string(), v.to_string());
            m
        });
        self.0
            .presign_get(key, expiry_secs, custom_queries)
            .await
            .map_err(|e| MediaError::InternalServerError(e.to_string()))
    }

    /// Generates a presigned PUT URL (for uploads).
    pub async fn presign_put(&self, key: &str, expiry_secs: u32) -> Result<String, MediaError> {
        self.0
            .presign_put(key, expiry_secs, None, None)
            .await
            .map_err(|e| MediaError::InternalServerError(e.to_string()))
    }
}
