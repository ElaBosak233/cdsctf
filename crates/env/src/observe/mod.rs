//! Configuration section — `mod` (loaded via Figment / `CDSCTF_*`).

/// Defines the `exporter` submodule (see sibling `*.rs` files).
pub mod exporter;

/// Defines the `logger` submodule (see sibling `*.rs` files).
pub mod logger;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub service_name: String,
    pub logger: logger::Config,
    pub exporter: exporter::Config,
}

impl Default for Config {
    /// Returns the default value for this type.
    fn default() -> Self {
        Self {
            service_name: "cdsctf".to_string(),
            logger: logger::Config::default(),
            exporter: exporter::Config::default(),
        }
    }
}
