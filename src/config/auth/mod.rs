pub mod jwt;
pub mod registration;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub jwt: jwt::Config,
    pub registration: registration::Config,
}
