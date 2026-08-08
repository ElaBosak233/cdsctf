//! Configuration section — `mod` (loaded via Figment / `CDSCTF_*`).

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub url: String,
    /// Prefix applied to every application-owned key.
    pub key_prefix: String,
    /// Timeout for opening a connection to Valkey.
    pub connection_timeout_ms: u64,
    /// Timeout for an individual cache command.
    pub response_timeout_ms: u64,
    /// Maximum number of commands concurrently awaiting a response.
    pub max_in_flight: usize,
}

impl Default for Config {
    /// Returns the default value for this type.
    fn default() -> Self {
        Self {
            url: "redis://cache:6379".to_string(),
            key_prefix: "cdsctf".to_string(),
            connection_timeout_ms: 5_000,
            response_timeout_ms: 3_000,
            max_in_flight: 1_024,
        }
    }
}
