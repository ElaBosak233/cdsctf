pub mod auth;
pub mod cluster;
pub mod site;
pub mod traits;

use futures_util::StreamExt;
use once_cell::sync::Lazy;
use sea_orm::EntityTrait;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use tracing::info;

use crate::db::get_db;

pub static APP_CONFIG: Lazy<RwLock<Config>> = Lazy::new(|| RwLock::new(Config::default()));

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Config {
    pub site: site::Config,
    pub auth: auth::Config,
    pub cluster: cluster::Config,
}

impl From<crate::db::entity::config::Model> for Config {
    fn from(config: crate::db::entity::config::Model) -> Self {
        Config {
            auth: config.auth,
            cluster: config.cluster,
            site: config.site,
        }
    }
}

pub async fn init() {
    tokio::spawn(async move {
        let mut messages = crate::queue::subscribe("config").await.unwrap();
        while let Some(result) = messages.next().await {
            if result.is_err() {
                continue;
            }
            let message = result.unwrap();
            let _ = String::from_utf8(message.payload.to_vec()).unwrap();
            sync().await;
            message.ack().await.unwrap();
        }
    });
    sync().await;
    info!("Configuration synchronizer initialized successfully.");
}

pub async fn sync() {
    let config = crate::db::entity::config::Entity::find()
        .one(get_db())
        .await
        .unwrap();
    if let Some(config) = config {
        *APP_CONFIG.write().await = config.into();
    }
}

pub async fn get_config() -> Config {
    let config = APP_CONFIG.read().await;
    config.clone()
}
