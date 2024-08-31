pub mod auth;
pub mod axum;
pub mod cache;
pub mod captcha;
pub mod consts;
pub mod container;
pub mod db;
pub mod queue;
pub mod site;

use serde::{Deserialize, Serialize};
use std::{path::Path, process, sync::OnceLock};
use tokio::fs::{self};
use tracing::error;

static APP_CONFIG: OnceLock<Config> = OnceLock::new();

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub site: site::Config,
    pub auth: auth::Config,
    pub axum: axum::Config,
    pub container: container::Config,
    pub captcha: captcha::Config,
    pub db: db::Config,
    pub queue: queue::Config,
    pub cache: cache::Config,
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

pub fn get_config() -> &'static Config {
    return APP_CONFIG.get().unwrap();
}
