pub mod auth;
pub mod cluster;
pub mod site;
pub mod traits;

use futures_util::StreamExt;
use sea_orm::EntityTrait;
use serde::{Deserialize, Serialize};

use crate::db::get_db;

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
    let config = crate::cache::get::<Config>("config").await.unwrap();
    if config.is_none() {
        let model = crate::db::entity::config::Entity::find()
            .one(get_db())
            .await
            .unwrap();
        if let Some(model) = model {
            let _ = crate::cache::set("config", Config::from(model.clone())).await;
        }
    }
}

pub async fn get_config() -> Config {
    let config = crate::cache::get::<Config>("config").await.unwrap();
    config.clone().unwrap()
}
