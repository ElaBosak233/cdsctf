pub mod auth;
pub mod cache;
pub mod cluster;
pub mod db;
pub mod media;
pub mod meta;
pub mod metric;
pub mod queue;
pub mod server;

use std::{path::Path, process};

use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use tokio::fs::{self};
use tracing::error;

static APP_CONFIG: OnceCell<Config> = OnceCell::new();

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub server: server::Config,
    pub auth: auth::Config,
    pub db: db::Config,
    pub queue: queue::Config,
    pub cache: cache::Config,
    pub metric: metric::Config,
    pub cluster: cluster::Config,
    pub media: media::Config,
    pub meta: meta::Config,
}

pub async fn init() {
    let target_path = Path::new("application.toml");
    if target_path.exists() {
        let content = fs::read_to_string("application.toml").await.unwrap();
        APP_CONFIG.set(toml::from_str(&content).unwrap()).unwrap();
    } else {
        error!("Configuration application.toml not found.");
        process::exit(1);
    }
}

pub fn get_config() -> Config {
    APP_CONFIG.get().unwrap().clone()
}
