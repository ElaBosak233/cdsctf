pub mod challenge;
pub mod config;
pub mod presigner;
pub mod traits;
pub mod util;

use std::{
    collections::HashSet,
    path::{Component, Path},
    sync::Arc,
};

use cds_env::Env;
use rust_embed::Embed;
use s3::{
    bucket::Bucket,
    bucket_ops::BucketConfiguration,
    creds::Credentials,
    error::S3Error,
    region::Region,
};

use crate::{config::Config, presigner::Presigner, traits::MediaError};

#[derive(Embed)]
#[folder = "./embed/"]
pub struct Embeds;

#[derive(Clone)]
pub struct Media {
    bucket: Arc<Bucket>,
    prefix: String,
    presigner: Option<Presigner>,
}

pub async fn init(env: &Env) -> Result<Media, MediaError> {
    let region = Region::Custom {
        region: env.media.region.clone(),
        endpoint: env.media.endpoint.clone(),
    };
    let credentials = Credentials::new(
        Some(&env.media.access_key),
        Some(&env.media.secret_key),
        None,
        None,
        None,
    )
    .map_err(|err| MediaError::OtherError(err.into()))?;

    let mut bucket = Bucket::new(&env.media.bucket, region.clone(), credentials)
        .map_err(|err| MediaError::OtherError(err.into()))?;
    if env.media.path_style {
        bucket = bucket.with_path_style();
    }

    let bucket = match bucket.exists().await {
        Ok(true) => bucket,
        Ok(false) => {
            let config = BucketConfiguration::private();
            if env.media.path_style {
                Bucket::create_with_path_style(
                    &env.media.bucket,
                    region.clone(),
                    bucket
                        .credentials()
                        .await
                        .map_err(|err| MediaError::InternalServerError(err.to_string()))?,
                    config,
                )
                .await
                .map_err(|err| MediaError::InternalServerError(err.to_string()))?
                .bucket
            } else {
                Bucket::create(
                    &env.media.bucket,
                    region.clone(),
                    bucket
                        .credentials()
                        .await
                        .map_err(|err| MediaError::InternalServerError(err.to_string()))?,
                    config,
                )
                .await
                .map_err(|err| MediaError::InternalServerError(err.to_string()))?
                .bucket
            }
        }
        Err(err) => return Err(MediaError::InternalServerError(err.to_string())),
    };

    let prefix = normalize_prefix(&env.media.prefix);
    let bucket: Arc<Bucket> = Arc::from(bucket);

    let presigner = if env.media.presigned {
        Some(Presigner::new(bucket.clone()))
    } else {
        None
    };

    let media = Media {
        bucket,
        prefix,
        presigner,
    };

    media.ensure_embeds().await?;

    Ok(media)
}

impl Media {
    pub async fn get(&self, path: String, filename: String) -> Result<Vec<u8>, MediaError> {
        let key = self.build_key(&path, &filename, true)?;
        let data = self
            .bucket
            .get_object(&key)
            .await
            .map_err(|err| map_s3_error(err, true))?;
        Ok(data.to_vec())
    }

    pub async fn create_dir(&self, path: String) -> Result<(), MediaError> {
        if normalize_path(Path::new(&path)).is_none() {
            return Ok(());
        }
        Ok(())
    }

    pub async fn scan_dir(&self, path: String) -> Result<Vec<(String, u64)>, MediaError> {
        let rel = match normalize_path(Path::new(&path)) {
            Some(rel) => rel,
            None => return Ok(Vec::new()),
        };
        let mut prefix = self.with_prefix(&rel);
        if !prefix.is_empty() {
            prefix.push('/');
        }

        let results = self
            .bucket
            .list(prefix.clone(), Some("/".to_string()))
            .await
            .map_err(|err| map_s3_error(err, false))?;

        let mut files = Vec::new();
        for result in results {
            for object in result.contents {
                let key = object.key;
                let name = key.strip_prefix(&prefix).unwrap_or(&key);
                if name.is_empty() || name.contains('/') {
                    continue;
                }
                let size: u64 = object.size.try_into().unwrap_or_else(|_| 0);
                files.push((name.to_string(), size));
            }
        }

        Ok(files)
    }

    pub async fn save(
        &self,
        path: String,
        filename: String,
        data: Vec<u8>,
    ) -> Result<(), MediaError> {
        let key = self.build_key(&path, &filename, false)?;
        self.bucket
            .put_object(&key, &data)
            .await
            .map_err(|err| map_s3_error(err, false))?;
        Ok(())
    }

    pub async fn delete(&self, path: String, filename: String) -> Result<(), MediaError> {
        let key = match self.build_key(&path, &filename, true) {
            Ok(key) => key,
            Err(_) => return Ok(()),
        };
        let _ = self
            .bucket
            .delete_object(&key)
            .await
            .map_err(|err| map_s3_error(err, false))?;
        Ok(())
    }

    pub async fn delete_dir(&self, path: String) -> Result<(), MediaError> {
        let rel = match normalize_path(Path::new(&path)) {
            Some(rel) => rel,
            None => return Ok(()),
        };
        let mut prefix = self.with_prefix(&rel);
        if !prefix.is_empty() {
            prefix.push('/');
        }

        let results = self
            .bucket
            .list(prefix.clone(), None)
            .await
            .map_err(|err| map_s3_error(err, false))?;

        for result in results {
            for object in result.contents {
                let _ = self
                    .bucket
                    .delete_object(&object.key)
                    .await
                    .map_err(|err| map_s3_error(err, false))?;
            }
        }

        Ok(())
    }

    pub fn config(&self) -> Config<'_> {
        Config::new(&self)
    }

    pub fn presigned_enabled(&self) -> bool {
        self.presigner.is_some()
    }

    pub async fn presign_get(
        &self,
        path: &str,
        filename: &str,
        expiry_secs: u32,
    ) -> Result<String, MediaError> {
        let presigner = self
            .presigner
            .as_ref()
            .ok_or_else(|| MediaError::InternalServerError("presigned url not configured".into()))?;
        let key = self.build_key(path, filename, false)?;
        let disposition = format!("attachment; filename=\"{}\"", filename.replace('"', "\\\""));
        presigner
            .presign_get(&key, expiry_secs, Some(&disposition))
            .await
    }

    pub async fn presign_put(
        &self,
        path: &str,
        filename: &str,
        expiry_secs: u32,
    ) -> Result<String, MediaError> {
        let presigner = self
            .presigner
            .as_ref()
            .ok_or_else(|| MediaError::InternalServerError("presigned url not configured".into()))?;
        let key = self.build_key(path, filename, false)?;
        presigner.presign_put(&key, expiry_secs).await
    }

    fn build_key(
        &self,
        path: &str,
        filename: &str,
        not_found_on_error: bool,
    ) -> Result<String, MediaError> {
        let joined = Path::new(path).join(filename);
        let rel = normalize_path(&joined).ok_or_else(|| {
            if not_found_on_error {
                MediaError::NotFound(String::new())
            } else {
                MediaError::InternalServerError(String::new())
            }
        })?;
        Ok(self.with_prefix(&rel))
    }

    fn with_prefix(&self, key: &str) -> String {
        if self.prefix.is_empty() {
            key.to_string()
        } else if key.is_empty() {
            self.prefix.clone()
        } else {
            format!("{}/{}", self.prefix, key)
        }
    }

    async fn ensure_embeds(&self) -> Result<(), MediaError> {
        let existing = self.list_existing_embed_keys().await?;

        for file in Embeds::iter() {
            if let Some(content) = Embeds::get(&file) {
                let key = self.with_prefix(file.as_ref());
                if existing.contains(&key) {
                    continue;
                }
                self.bucket
                    .put_object(&key, content.data.as_ref())
                    .await
                    .map_err(|err| map_s3_error(err, false))?;
            }
        }

        Ok(())
    }

    async fn list_existing_embed_keys(&self) -> Result<HashSet<String>, MediaError> {
        let mut existing = HashSet::new();
        for file in Embeds::iter() {
            let key = self.with_prefix(file.as_ref());
            let results = self
                .bucket
                .list(key.clone(), Some("/".to_string()))
                .await
                .map_err(|err| map_s3_error(err, false))?;
            if results
                .iter()
                .any(|result| result.contents.iter().any(|object| object.key == key))
            {
                existing.insert(key);
            }
        }
        Ok(existing)
    }
}

fn normalize_prefix(prefix: &str) -> String {
    let mut prefix = prefix.trim().trim_matches('/').to_string();
    if prefix == "." {
        prefix.clear();
    }
    prefix
}

fn normalize_path(path: &Path) -> Option<String> {
    if path.is_absolute() {
        return None;
    }
    let mut parts = Vec::new();
    for component in path.components() {
        match component {
            Component::Normal(part) => parts.push(part.to_string_lossy().into_owned()),
            _ => return None,
        }
    }
    Some(parts.join("/"))
}

fn map_s3_error(err: S3Error, treat_not_found_as_missing: bool) -> MediaError {
    let message = err.to_string();
    if treat_not_found_as_missing && (message.contains("NoSuchKey") || message.contains("404")) {
        MediaError::NotFound(String::new())
    } else {
        MediaError::InternalServerError(message)
    }
}
