pub mod cache;
pub mod cluster;
pub mod db;
mod jwt;
pub mod logger;
pub mod media;
pub mod queue;
pub mod server;
pub mod telemetry;
pub mod traits;

use std::path::Path;

use anyhow::anyhow;
use nanoid::nanoid;
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use tokio::fs::{create_dir_all, metadata};

use crate::traits::ConfigError;

static CONSTANT: OnceCell<Constant> = OnceCell::new();

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Constant {
    pub server: server::Config,
    pub jwt: jwt::Config,
    pub db: db::Config,
    pub queue: queue::Config,
    pub cache: cache::Config,
    pub cluster: cluster::Config,
    pub media: media::Config,
    pub logger: logger::Config,
    pub telemetry: telemetry::Config,
}

pub fn get_constant() -> &'static Constant {
    CONSTANT.get().unwrap()
}

pub async fn init() -> Result<(), ConfigError> {
    let target_path = Path::new("data/configs/constant.toml");
    if !target_path.exists() {
        if let Some(parent) = target_path.parent() {
            if metadata(parent).await.is_err() {
                create_dir_all(parent).await?;
            }
        }
        let content =
            std::str::from_utf8(&cds_assets::get("configs/constant.toml").unwrap_or_default())?
                .replace("%nanoid%", &nanoid!(24));
        tokio::fs::write(target_path, content).await?;
    }

    let content = tokio::fs::read_to_string(target_path).await?;
    CONSTANT
        .set(toml::from_str(&content)?)
        .map_err(|_| anyhow!("Failed to set constant env into OnceCell."))?;

    Ok(())
}

pub fn get_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

pub fn get_commit() -> String {
    env!("GIT_COMMIT").to_string()
}

pub fn get_build_timestamp() -> i64 {
    env!("BUILD_AT").parse::<i64>().unwrap_or_default()
}
