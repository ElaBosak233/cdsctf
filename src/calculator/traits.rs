use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CalculatorPayload {
    pub team_id: Option<i64>,
    pub game_id: Option<i64>,
}
