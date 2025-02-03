use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub level: String,
    pub path: String,
    pub files_kept: usize,
}
