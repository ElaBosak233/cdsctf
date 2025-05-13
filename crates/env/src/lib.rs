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

use crate::traits::EnvError;

const CONFIG_PREDEFINED_PATH: [&str; 4] = [
    "/etc/cdsctf/",
    "~/.config/cdsctf/",
    "./config/",
    "./data/config/",
];

const CONFIG_PREDEFINED_FILE_NAME: &str = "config.toml";

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

pub fn get_config() -> &'static Constant {
    CONSTANT.get().unwrap()
}

pub async fn init() -> Result<(), EnvError> {
    for raw_path in CONFIG_PREDEFINED_PATH.into_iter() {
        let file_path = Path::new(raw_path).join(CONFIG_PREDEFINED_FILE_NAME);
        match tokio::fs::read_to_string(file_path).await {
            Ok(content) => {
                let content = content.replace("%nanoid%", &nanoid!(24));
                CONSTANT
                    .set(toml::from_str(&content)?)
                    .map_err(|_| anyhow!("Failed to set constant env into OnceCell."))?;

                return Ok(());
            }
            _ => continue,
        }
    }

    Err(EnvError::NotFound)
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
