//! Configuration section — `mod` (loaded via Figment / `CDSCTF_*`).

/// Defines the `rate_limit` submodule (see sibling `*.rs` files).
mod rate_limit;

use serde::{Deserialize, Serialize};

/// Process runtime role for horizontal scaling deployments.
///
/// - `all` (default): serves HTTP APIs and runs background workers in one
///   binary.
/// - `api`: serves HTTP APIs and CQRS score queries only (no checker/mailbox).
/// - `worker`: runs background checker and mailbox workers only (no HTTP
///   listener).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    #[default]
    All,
    Api,
    Worker,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub frontend: String,
    pub rate_limit: rate_limit::Config,
    pub cors_origins: String,
    pub role: Role,
}

impl Default for Config {
    /// Returns the default value for this type.
    fn default() -> Self {
        Config {
            host: "0.0.0.0".to_owned(),
            port: 8888,
            frontend: "./dist".to_owned(),
            rate_limit: rate_limit::Config::default(),
            cors_origins: "*".to_owned(),
            role: Role::All,
        }
    }
}
