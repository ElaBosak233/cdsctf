//! Message body for the `calculator` subject (game score / rank recomputation).

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Payload {
    pub game_id: Option<i64>,
}
