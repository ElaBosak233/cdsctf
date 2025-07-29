use serde::{Deserialize, Serialize};

use crate::types::game_challenge::GameChallengeEvent;

pub mod game_challenge;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload", rename_all = "snake_case")]
pub enum Event {
    GameChallenge(GameChallengeEvent),
}
