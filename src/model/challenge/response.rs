use serde::{Deserialize, Serialize};

use super::submission;

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusResponse {
    pub is_solved: bool,
    pub solved_times: i64,
    pub pts: i64,
    pub bloods: Vec<submission::Model>,
}
