pub mod redis;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub provider: String,
    pub redis: redis::Config,
}
