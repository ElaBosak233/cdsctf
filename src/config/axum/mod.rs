pub mod cors;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub cors: cors::Config,
    pub host: String,
    pub port: u16,
}
