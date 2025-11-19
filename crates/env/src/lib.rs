pub mod cache;
pub mod cluster;
pub mod db;
pub mod logger;
pub mod media;
pub mod observe;
pub mod queue;
pub mod server;
pub mod traits;

use std::path::Path;

use anyhow::anyhow;
use nanoid::nanoid;
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use shadow_rs::shadow;

use crate::traits::EnvError;

shadow!(build);

const CONFIG_PREDEFINED_PATH: [&str; 4] = [
    "/etc/cdsctf/",
    "~/.config/cdsctf/",
    "./config/",
    "./data/config/",
];

const CONFIG_PREDEFINED_FILE_NAME: &str = "config.toml";

static CONSTANT: OnceCell<Constant> = OnceCell::new();

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Constant {
    pub server: server::Config,
    pub db: db::Config,
    pub queue: queue::Config,
    pub cache: cache::Config,
    pub cluster: cluster::Config,
    pub media: media::Config,
    pub logger: logger::Config,
    pub observe: observe::Config,
}

pub fn get_config() -> &'static Constant {
    CONSTANT
        .get()
        .expect("No runtime config instance, forget to init?")
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

pub fn get_version() -> &'static str {
    build::PKG_VERSION
}

pub fn get_commit_hash() -> &'static str {
    build::COMMIT_HASH
}

pub fn get_build_time() -> &'static str {
    build::BUILD_TIME
}
