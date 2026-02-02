//! Presigned URL 生成器。
//! 仅当 [media.presigned](cds_env::media::Config::presigned) 为 true 时存在，
//! 复用主 bucket，使用相同 endpoint 签名（要求该 endpoint 对用户可访问）。

use std::{collections::HashMap, sync::Arc};

use s3::bucket::Bucket;

use crate::traits::MediaError;

/// 复用已有 bucket 的签名器，仅用于生成 presigned URL。
#[derive(Clone)]
pub struct Presigner(Arc<Bucket>);

impl Presigner {
    pub(crate) fn new(bucket: Arc<Bucket>) -> Self {
        Self(bucket)
    }

    /// 生成 GET 预签名 URL（用于下载）。
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

    /// 生成 PUT 预签名 URL（用于上传）。
    pub async fn presign_put(&self, key: &str, expiry_secs: u32) -> Result<String, MediaError> {
        self.0
            .presign_put(key, expiry_secs, None, None)
            .await
            .map_err(|e| MediaError::InternalServerError(e.to_string()))
    }
}
