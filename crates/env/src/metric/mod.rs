use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Env {
    pub is_enabled: bool,
    pub namespace: String,
}
