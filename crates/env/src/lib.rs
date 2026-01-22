pub mod cache;
pub mod cluster;
pub mod db;
pub mod media;
pub mod observe;
pub mod queue;
pub mod server;
pub mod traits;

use anyhow::anyhow;
use figment::{
    Figment,
    providers::{Env, Format, Toml},
};
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
    pub observe: observe::Config,
}

pub fn get_config() -> &'static Constant {
    CONSTANT
        .get()
        .expect("No runtime config instance, forget to init?")
}

fn expand_tilde(path: &str) -> std::path::PathBuf {
    if let Some(stripped) = path.strip_prefix("~/") {
        if let Some(home) = std::env::var_os("HOME") {
            return std::path::PathBuf::from(home).join(stripped);
        }
    }
    std::path::PathBuf::from(path)
}

fn find_first_config_file() -> Option<std::path::PathBuf> {
    for dir in CONFIG_PREDEFINED_PATH {
        let dir = expand_tilde(dir);
        let candidate = dir.join(CONFIG_PREDEFINED_FILE_NAME);
        if candidate.is_file() {
            return Some(candidate);
        }
    }
    None
}

pub async fn init() -> Result<(), EnvError> {
    let mut figment = Figment::new();
    if let Some(path) = find_first_config_file() {
        figment = figment.merge(Toml::file(path));
    }
    figment = figment.merge(Env::prefixed("CDSCTF_").split("_"));
    let config = figment.extract::<Constant>()?;
    CONSTANT
        .set(config)
        .map_err(|_| anyhow!("Failed to set constant env into OnceCell."))?;

    Ok(())
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
