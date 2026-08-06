use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};

use crate::entity::team::State;

#[derive(
    Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromQueryResult, utoipa::ToSchema,
)]
pub struct TeamView {
    pub id: i64,
    pub game_id: i64,
    pub name: String,
    pub email: Option<String>,
    pub slogan: Option<String>,
    pub avatar_hash: Option<String>,
    pub has_writeup: bool,
    pub state: State,
    pub pts: i64,
    pub rank: i64,
}

impl From<&TeamView> for super::scoreboard::ScoreboardTeam {
    fn from(team: &TeamView) -> Self {
        Self {
            id: team.id,
            name: team.name.clone(),
            slogan: team.slogan.clone(),
            avatar_hash: team.avatar_hash.clone(),
            pts: team.pts,
            rank: team.rank,
        }
    }
}
