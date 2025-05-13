use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub frontend: String,
    pub burst_restore_rate: Option<u64>,
    pub burst_limit: Option<u32>,
}
