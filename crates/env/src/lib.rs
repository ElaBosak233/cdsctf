//! Configuration loading: layered TOML + environment variables into a typed
//! [`Env`].
//!
//! Predefined directories are searched for `config.toml`; then `CDSCTF_`
//! prefixed variables override (nested keys use `__`, e.g.
//! `CDSCTF_SERVER__PORT`). Build metadata comes from `shadow-rs`.

/// Defines the `cache` submodule (see sibling `*.rs` files).
pub mod cache;

/// Defines the `cluster` submodule (see sibling `*.rs` files).
pub mod cluster;

/// Defines the `db` submodule (see sibling `*.rs` files).
pub mod db;

/// Defines the `media` submodule (see sibling `*.rs` files).
pub mod media;

/// Defines the `observe` submodule (see sibling `*.rs` files).
pub mod observe;

/// Defines the `queue` submodule (see sibling `*.rs` files).
pub mod queue;

/// Defines the `server` submodule (see sibling `*.rs` files).
pub mod server;

/// Defines the `traits` submodule (see sibling `*.rs` files).
pub mod traits;

use figment::{
    Figment,
    providers::{Env as FEnv, Format, Toml},
};
use serde::{Deserialize, Serialize};
use shadow_rs::shadow;

use crate::traits::EnvError;

shadow!(build);

/// Candidate directories for `config.toml` (first match wins).
const CONFIG_PREDEFINED_PATH: [&str; 4] = [
    "/etc/cdsctf/",
    "~/.config/cdsctf/",
    "./config/",
    "./data/config/",
];

/// Fixed config file basename inside each search directory.
const CONFIG_PREDEFINED_FILE_NAME: &str = "config.toml";

/// Root configuration struct deserialized from Figment (TOML + env).
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Env {
    pub server: server::Config,
    pub db: db::Config,
    pub queue: queue::Config,
    pub cache: cache::Config,
    pub cluster: cluster::Config,
    pub media: media::Config,
    pub observe: observe::Config,
}

/// Expands a leading `~/` using `$HOME`; otherwise returns the path as-is.
fn expand_tilde(path: &str) -> std::path::PathBuf {
    if let Some(stripped) = path.strip_prefix("~/") {
        if let Some(home) = std::env::var_os("HOME") {
            return std::path::PathBuf::from(home).join(stripped);
        }
    }
    std::path::PathBuf::from(path)
}

/// Returns the first existing `config.toml` path from
/// [`CONFIG_PREDEFINED_PATH`].
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

/// Merges optional TOML file with `CDSCTF_*` environment and deserializes into
/// [`Env`].
pub async fn init() -> Result<Env, EnvError> {
    let mut figment = Figment::new();
    if let Some(path) = find_first_config_file() {
        figment = figment.merge(Toml::file(path));
    }
    figment = figment.merge(FEnv::prefixed("CDSCTF_").split("__"));
    let global_env = figment.extract::<Env>()?;

    Ok(global_env)
}

/// Cargo package version from build-time `shadow` metadata.
pub fn get_version() -> &'static str {
    build::PKG_VERSION
}

/// Git commit hash captured at build time.
pub fn get_commit_hash() -> &'static str {
    build::COMMIT_HASH
}

/// ISO-ish build timestamp string from `shadow-rs`.
pub fn get_build_time() -> &'static str {
    build::BUILD_TIME
}
