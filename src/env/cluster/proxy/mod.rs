use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Env {
    pub enabled: bool,
    pub traffic_capture: bool,
}
