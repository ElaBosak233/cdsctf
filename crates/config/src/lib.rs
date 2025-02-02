pub mod auth;
pub mod cache;
pub mod cluster;
pub mod db;
pub mod media;
pub mod meta;
pub mod queue;
pub mod server;
pub mod telemetry;
mod traits;

use std::path::Path;

use anyhow::anyhow;
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use tokio::fs::{self};

use crate::traits::ConfigError;

static APP_CONFIG: OnceCell<Config> = OnceCell::new();

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub server: server::Config,
    pub meta: meta::Config,
    pub auth: auth::Config,
    pub db: db::Config,
    pub queue: queue::Config,
    pub cache: cache::Config,
    pub cluster: cluster::Config,
    pub media: media::Config,
    pub telemetry: telemetry::Config,
}

pub fn get_config() -> &'static Config {
    APP_CONFIG.get().unwrap()
}

pub async fn init() -> Result<(), ConfigError> {
    let target_path = Path::new("application.toml");
    if !target_path.exists() {
        return Err(ConfigError::ConfigNotFound());
    }

    let content = fs::read_to_string("application.toml").await?;
    APP_CONFIG
        .set(toml::from_str(&content)?)
        .map_err(|_| anyhow!("Failed to set config into OnceCell."))?;

    Ok(())
}
