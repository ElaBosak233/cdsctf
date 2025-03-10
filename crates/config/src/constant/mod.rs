pub mod cache;
pub mod cluster;
pub mod db;
pub mod logger;
pub mod media;
pub mod queue;
pub mod server;
pub mod telemetry;

use std::path::Path;

use anyhow::anyhow;
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use tokio::fs::{create_dir_all, metadata};

use crate::traits::ConfigError;

static CONSTANT: OnceCell<Constant> = OnceCell::new();

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Constant {
    pub server: server::Config,
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
        let content = cds_assets::get("configs/constant.toml").unwrap_or_default();
        tokio::fs::write(target_path, content).await?;
    }

    let content = tokio::fs::read_to_string(target_path).await?;
    CONSTANT
        .set(toml::from_str(&content)?)
        .map_err(|_| anyhow!("Failed to set constant config into OnceCell."))?;

    Ok(())
}
