use serde::{Deserialize, Serialize};

use crate::{
    config::{auth, cluster, site},
    db::entity,
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Config {
    pub id: i64,
    pub auth: auth::Config,
    pub cluster: cluster::Config,
    pub site: site::Config,
}

impl From<entity::config::Model> for Config {
    fn from(model: entity::config::Model) -> Self {
        Self {
            id: model.id,
            auth: model.auth,
            cluster: model.cluster,
            site: model.site,
        }
    }
}
