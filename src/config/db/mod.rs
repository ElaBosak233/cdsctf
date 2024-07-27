pub mod mysql;
pub mod postgres;
pub mod sqlite;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub provider: String,
    pub sqlite: sqlite::Config,
    pub postgres: postgres::Config,
    pub mysql: mysql::Config,
}
