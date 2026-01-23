pub mod challenge;
pub mod config;
pub mod traits;
pub mod util;

use std::path::{Component, Path, PathBuf};

use cds_env::Env;
use rust_embed::Embed;
use tokio::{
    fs::{File, create_dir_all, metadata, read_dir, remove_dir_all, remove_file, write},
    io::{AsyncReadExt, AsyncWriteExt},
};

use crate::{config::Config, traits::MediaError};

#[derive(Embed)]
#[folder = "./embed/"]
pub struct Embeds;

#[derive(Clone)]
pub struct Media {
    pub root: PathBuf,
}

pub async fn init(env: &Env) -> Result<Media, MediaError> {
    let path = PathBuf::from(&env.media.path);
    if metadata(&path).await.is_err() {
        create_dir_all(&path).await?;

        for file in Embeds::iter() {
            if let Some(content) = Embeds::get(&file) {
                let file_path = path.join(&file.as_ref());
                if let Some(parent) = file_path.parent() {
                    create_dir_all(parent).await?;
                }
                write(&file_path, content.data.as_ref()).await?;
            }
        }
    }

    Ok(Media { root: path })
}

impl Media {
    pub async fn get(&self, path: String, filename: String) -> Result<Vec<u8>, MediaError> {
        let joined = Path::new(&path).join(&filename);
        if joined.is_absolute()
            || joined
                .components()
                .any(|c| !matches!(c, Component::Normal(_)))
        {
            return Err(MediaError::NotFound(String::new()));
        }

        let filepath = PathBuf::from(&self.root).join(joined);

        match File::open(&filepath).await {
            Ok(mut file) => {
                let mut buffer = Vec::new();
                if file.read_to_end(&mut buffer).await.is_err() {
                    return Err(MediaError::InternalServerError(String::new()));
                }
                Ok(buffer)
            }
            Err(_) => Err(MediaError::NotFound(String::new())),
        }
    }

    pub async fn create_dir(&self, path: String) -> Result<(), MediaError> {
        let rel = Path::new(&path);
        if rel.is_absolute() || rel.components().any(|c| !matches!(c, Component::Normal(_))) {
            return Ok(());
        }

        let filepath = PathBuf::from(&self.root).join(rel);
        if metadata(&filepath).await.is_err() {
            create_dir_all(&filepath).await?;
        }

        Ok(())
    }

    pub async fn scan_dir(&self, path: String) -> Result<Vec<(String, u64)>, MediaError> {
        let rel = Path::new(&path);
        if rel.is_absolute() || rel.components().any(|c| !matches!(c, Component::Normal(_))) {
            return Ok(Vec::new());
        }

        let filepath = PathBuf::from(&self.root).join(rel);
        let mut files = Vec::new();

        if metadata(&filepath).await.is_err() {
            return Ok(files);
        }

        let mut dir = read_dir(&filepath).await?;

        while let Some(entry) = dir.next_entry().await? {
            let path = entry.path();
            let metadata = entry.metadata().await?;
            if metadata.is_file() {
                let file_name = path
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .into_owned();
                let file_size = metadata.len();
                files.push((file_name, file_size));
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
        let joined = Path::new(&path).join(&filename);
        if joined.is_absolute()
            || joined
                .components()
                .any(|c| !matches!(c, Component::Normal(_)))
        {
            return Err(MediaError::InternalServerError(String::new()));
        }

        let filepath = PathBuf::from(&self.root).join(joined);
        if let Some(parent) = filepath.parent() {
            if metadata(parent).await.is_err() {
                create_dir_all(parent).await?;
            }
        }
        let mut file = File::create(&filepath).await?;
        file.write_all(&data).await?;
        Ok(())
    }

    pub async fn delete(&self, path: String, filename: String) -> Result<(), MediaError> {
        let joined = Path::new(&path).join(&filename);
        if joined.is_absolute()
            || joined
                .components()
                .any(|c| !matches!(c, Component::Normal(_)))
        {
            return Ok(());
        }

        let filepath = PathBuf::from(&self.root).join(joined);
        if metadata(&filepath).await.is_ok() {
            remove_file(&filepath).await?;
        }
        Ok(())
    }

    pub async fn delete_dir(&self, path: String) -> Result<(), MediaError> {
        let rel = Path::new(&path);
        if rel.is_absolute() || rel.components().any(|c| !matches!(c, Component::Normal(_))) {
            return Ok(());
        }

        let filepath = PathBuf::from(&self.root).join(rel);
        if metadata(&filepath).await.is_ok() {
            remove_dir_all(&filepath).await?;
        }
        Ok(())
    }

    pub fn config(&self) -> Config<'_> {
        Config::new(&self)
    }
}
