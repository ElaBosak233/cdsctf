use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Metadata {
    pub filename: String,
    pub size: u64,
}
