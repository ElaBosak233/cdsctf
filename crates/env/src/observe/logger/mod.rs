//! Configuration section — `mod` (loaded via Figment / `CDSCTF_*`).

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub level: String,
}

impl Default for Config {
    /// Returns the default value for this type.
    fn default() -> Self {
        Self {
            level: "info".to_string(),
        }
    }
}
