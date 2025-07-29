use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct GameChallengeEvent {
    #[serde(rename = "type")]
    pub type_: GameChallengeEventType,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GameChallengeEventType {
    Up,
    Down,
}
