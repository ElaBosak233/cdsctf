pub mod traits;
pub mod util;

use std::{error::Error, path::PathBuf};

use tokio::{
    fs::{create_dir_all, metadata, read_dir, remove_dir_all, remove_file, File},
    io::{AsyncReadExt, AsyncWriteExt},
};

use crate::media::traits::MediaError;

pub async fn get(path: String, filename: String) -> Result<Vec<u8>, MediaError> {
    let filepath =
        PathBuf::from(crate::env::consts::path::MEDIA).join(format!("{}/{}", path, filename));

    match File::open(&filepath).await {
        Ok(mut file) => {
            let mut buffer = Vec::new();
            if (file.read_to_end(&mut buffer).await).is_err() {
                return Err(MediaError::InternalServerError(String::new()));
            }
            Ok(buffer)
        }
        Err(_) => Err(MediaError::NotFound(String::new())),
    }
}

pub async fn scan_dir(path: String) -> Result<Vec<(String, u64)>, MediaError> {
    let filepath = PathBuf::from(crate::env::consts::path::MEDIA).join(path);
    let mut files = Vec::new();

    if metadata(&filepath).await.is_err() {
        return Ok(files);
    }

    let mut dir = read_dir(&filepath).await?;

    while let Some(entry) = dir.next_entry().await? {
        let path = entry.path();
        let metadata = entry.metadata().await?;
        if metadata.is_file() {
            let file_name = path.file_name().unwrap().to_string_lossy().into_owned();
            let file_size = metadata.len();
            files.push((file_name, file_size));
        }
    }
    Ok(files)
}

pub async fn save(path: String, filename: String, data: Vec<u8>) -> Result<(), MediaError> {
    let filepath =
        PathBuf::from(crate::env::consts::path::MEDIA).join(format!("{}/{}", path, filename));
    if let Some(parent) = filepath.parent() {
        if metadata(parent).await.is_err() {
            create_dir_all(parent).await?;
        }
    }
    let mut file = File::create(&filepath).await?;
    file.write_all(&data).await?;
    Ok(())
}

pub async fn delete(path: String, filename: String) -> Result<(), MediaError> {
    let filepath =
        PathBuf::from(crate::env::consts::path::MEDIA).join(format!("{}/{}", path, filename));
    if metadata(&filepath).await.is_ok() {
        remove_file(&filepath).await?;
    }
    Ok(())
}

pub async fn delete_dir(path: String) -> Result<(), Box<dyn Error>> {
    let filepath = PathBuf::from(crate::env::consts::path::MEDIA).join(path);
    if metadata(&filepath).await.is_ok() {
        remove_dir_all(&filepath).await?;
    }
    Ok(())
}
