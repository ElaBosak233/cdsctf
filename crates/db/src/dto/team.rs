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

/// Player-facing team projection. Score fields are omitted while the game is
/// blacked out, while administrators continue to use [`TeamView`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, utoipa::ToSchema)]
pub struct PlayerTeamView {
    pub id: i64,
    pub game_id: i64,
    pub name: String,
    pub email: Option<String>,
    pub slogan: Option<String>,
    pub avatar_hash: Option<String>,
    pub has_writeup: bool,
    pub state: State,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pts: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank: Option<i64>,
}

impl PlayerTeamView {
    pub fn from_team(team: TeamView, blacked_out: bool) -> Self {
        Self {
            id: team.id,
            game_id: team.game_id,
            name: team.name,
            email: team.email,
            slogan: team.slogan,
            avatar_hash: team.avatar_hash,
            has_writeup: team.has_writeup,
            state: team.state,
            pts: (!blacked_out).then_some(team.pts),
            rank: (!blacked_out).then_some(team.rank),
        }
    }
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

#[cfg(test)]
mod tests {
    use super::{PlayerTeamView, State, TeamView};

    fn team() -> TeamView {
        TeamView {
            id: 1,
            game_id: 2,
            name: "team".to_owned(),
            email: None,
            slogan: None,
            avatar_hash: None,
            has_writeup: false,
            state: State::Passed,
            pts: 500,
            rank: 3,
        }
    }

    #[test]
    fn player_team_includes_scores_when_game_is_visible() {
        let value = serde_json::to_value(PlayerTeamView::from_team(team(), false)).unwrap();

        assert_eq!(value["pts"], 500);
        assert_eq!(value["rank"], 3);
    }

    #[test]
    fn player_team_omits_scores_when_game_is_blacked_out() {
        let value = serde_json::to_value(PlayerTeamView::from_team(team(), true)).unwrap();

        assert!(value.get("pts").is_none());
        assert!(value.get("rank").is_none());
    }
}
