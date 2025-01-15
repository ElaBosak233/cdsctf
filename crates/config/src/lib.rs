pub mod auth;
pub mod cluster;
pub mod site;
pub mod traits;

use cds_db::get_db;
use sea_orm::EntityTrait;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Config {
    pub site: site::Config,
    pub auth: auth::Config,
    pub cluster: cluster::Config,
}

impl From<cds_db::entity::config::Model> for Config {
    fn from(config: cds_db::entity::config::Model) -> Self {
        serde_json::from_value::<Self>(config.value).unwrap()
    }
}

impl Config {
    pub fn desensitize(&mut self) {
        self.auth.jwt.secret_key.clear();
    }
}

pub async fn init() {
    let config = cds_cache::get::<Config>("config").await.unwrap();
    if config.is_none() {
        let model = cds_db::entity::config::Entity::find()
            .one(get_db())
            .await
            .unwrap();
        if let Some(model) = model {
            let _ = cds_cache::set("config", Config::from(model.clone())).await;
        }
    }
}

pub async fn get_config() -> Config {
    let config = cds_cache::get::<Config>("config").await.unwrap();
    config.clone().unwrap()
}
