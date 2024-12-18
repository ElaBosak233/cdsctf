use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct Config {
    pub enabled: bool,
    pub domains: Vec<String>,
}
