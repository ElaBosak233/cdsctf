pub mod axum;
pub mod cache;
pub mod cluster;
pub mod consts;
pub mod db;
pub mod metric;
pub mod queue;

use std::{path::Path, process, sync::OnceLock};

use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use tokio::fs::{self};
use tracing::error;

static APP_ENV: OnceCell<Env> = OnceCell::new();

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Env {
    pub axum: axum::Env,
    pub db: db::Env,
    pub queue: queue::Env,
    pub cache: cache::Env,
    pub metric: metric::Env,
    pub cluster: cluster::Env,
}

pub async fn init() {
    let target_path = Path::new("application.toml");
    if target_path.exists() {
        let content = fs::read_to_string("application.toml").await.unwrap();
        APP_ENV.set(toml::from_str(&content).unwrap()).unwrap();
    } else {
        error!("Environment configuration application.toml not found.");
        process::exit(1);
    }
}

pub fn get_env() -> &'static Env {
    APP_ENV.get().unwrap()
}
