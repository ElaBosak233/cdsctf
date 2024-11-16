pub mod email;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct Config {
    pub enabled: bool,
    pub captcha: bool,
    pub email: email::Config,
}
