//! JSON message body published to the [`super::SUBJECT`] stream.

use serde::{Deserialize, Serialize};

/// Optional scope for a recalculation job.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Payload {
    /// When `Some`, only that competition is recomputed; when `None`, every
    /// game is processed.
    pub game_id: Option<i64>,
}
