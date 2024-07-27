use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub title: String,
    pub description: String,
    pub color: String,
    pub favicon: String,
}
