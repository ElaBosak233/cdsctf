use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct Config {
    pub enabled: bool,
    pub traffic_capture: bool,
}
