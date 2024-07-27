use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub parallel_limit: u64,
    pub request_limit: u64,
}
