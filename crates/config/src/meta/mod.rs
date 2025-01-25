use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub title: String,
    pub description: String,
    pub logo_path: String,
}
