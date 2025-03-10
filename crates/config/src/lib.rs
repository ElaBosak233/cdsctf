pub mod constant;
mod traits;
pub mod variable;

pub use constant::get_constant;
use serde::{Deserialize, Serialize};
pub use variable::get_variable;

use crate::traits::ConfigError;

pub fn get_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

pub fn get_commit() -> String {
    env!("GIT_COMMIT").to_string()
}

pub fn get_build_timestamp() -> i64 {
    env!("BUILD_AT").parse::<i64>().unwrap_or_default()
}

pub async fn init() -> Result<(), ConfigError> {
    constant::init().await?;
    variable::init().await?;

    Ok(())
}
