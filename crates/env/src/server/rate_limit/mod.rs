//! Configuration section — `mod` (loaded via Figment / `CDSCTF_*`).

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub enabled: bool,
    pub burst_restore_rate: u64,
    pub burst_size: u32,
}

impl Default for Config {
    /// Returns the default value for this type.
    fn default() -> Self {
        Config {
            enabled: true,
            burst_restore_rate: 100,
            burst_size: 512,
        }
    }
}
