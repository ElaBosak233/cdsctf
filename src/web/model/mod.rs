pub mod category;
pub mod challenge;
pub mod game;
pub mod pod;
pub mod proxy;
pub mod submission;
pub mod team;
pub mod user;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Metadata {
    pub filename: String,
    pub size: u64,
}
