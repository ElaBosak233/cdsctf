//! Event system — `mod` (types and traits for NATS-backed events).

use serde::{Deserialize, Serialize};

use crate::types::game_challenge::GameChallengeEvent;

/// Defines the `game_challenge` submodule (see sibling `*.rs` files).
pub mod game_challenge;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload", rename_all = "snake_case")]
pub enum Event {
    GameChallenge(GameChallengeEvent),
}
