use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct Config {
    pub parallel_limit: u64,
    pub request_limit: u64,
}
