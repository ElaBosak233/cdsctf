//! Web utility — `loader` (shared HTTP helpers).

use cds_db::{
    ChallengeDetail, GameChallengeView, GameDetail, UserAccountView,
    sea_orm::DatabaseConnection,
    team::{FindTeamOptions, TeamView},
};
use serde_json::json;

use crate::traits::WebError;

/// Rejects player actions while a game is administratively paused.
pub fn ensure_game_not_paused(game: &GameDetail) -> Result<(), WebError> {
    if game.paused {
        return Err(WebError::Locked(json!("game_paused")));
    }

    Ok(())
}

/// Rejects actions outside the configured competition window.
pub fn ensure_game_ongoing(game: &GameDetail, now: i64) -> Result<(), WebError> {
    if !(game.started_at..=game.ended_at).contains(&now) {
        return Err(WebError::Forbidden(json!("game_not_ongoing")));
    }

    Ok(())
}

/// Loads a challenge model for downstream handlers.
pub async fn prepare_challenge(
    db: &DatabaseConnection,
    challenge_id: i64,
) -> Result<ChallengeDetail, WebError> {
    let challenge = cds_db::challenge::find_by_id(db, challenge_id)
        .await?
        .ok_or(WebError::NotFound(json!("challenge_not_found")))?;

    Ok(challenge)
}

/// Loads a game model for downstream handlers.
pub async fn prepare_game(db: &DatabaseConnection, game_id: i64) -> Result<GameDetail, WebError> {
    let game = cds_db::game::find_by_id(db, game_id)
        .await?
        .ok_or(WebError::NotFound(json!("challenge_not_found")))?;

    Ok(game)
}

/// Loads the game/challenge binding row.
pub async fn prepare_game_challenge(
    db: &DatabaseConnection,
    game_id: i64,
    challenge_id: i64,
) -> Result<GameChallengeView, WebError> {
    let game_challenge = cds_db::game_challenge::find_by_id(db, game_id, challenge_id)
        .await?
        .ok_or(WebError::NotFound(json!("game_challenge_not_found")))?;

    Ok(game_challenge)
}

/// Loads the caller's team within a game.
pub async fn prepare_self_team(
    db: &DatabaseConnection,
    game_id: i64,
    user_id: i64,
) -> Result<TeamView, WebError> {
    let (teams, _) = cds_db::team::find(
        db,
        FindTeamOptions {
            game_id: Some(game_id),
            user_id: Some(user_id),
            ..Default::default()
        },
    )
    .await?;

    teams
        .into_iter()
        .next()
        .ok_or(WebError::NotFound(json!("team_not_found")))
}

/// Loads an arbitrary team by id within a game.
pub async fn prepare_team(
    db: &DatabaseConnection,
    game_id: i64,
    team_id: i64,
) -> Result<TeamView, WebError> {
    cds_db::team::find_by_id(db, team_id, game_id)
        .await?
        .ok_or(WebError::NotFound(json!("team_not_found")))
}

/// Loads a user model for permission checks.
pub async fn prepare_user(
    db: &DatabaseConnection,
    user_id: i64,
) -> Result<UserAccountView, WebError> {
    let user = cds_db::user::find_by_id(db, user_id)
        .await?
        .ok_or(WebError::NotFound(json!("user_not_found")))?;

    Ok(user)
}

#[cfg(test)]
mod tests {
    use cds_db::GameDetail;

    use super::{ensure_game_not_paused, ensure_game_ongoing};
    use crate::traits::WebError;

    fn game() -> GameDetail {
        GameDetail {
            id: 1,
            title: "game".to_owned(),
            sketch: None,
            description: None,
            enabled: true,
            public: true,
            paused: false,
            blacked_out: false,
            writeup_required: false,
            member_limit_min: 1,
            member_limit_max: 3,
            timeslots: Vec::new(),
            started_at: 100,
            frozen_at: 150,
            ended_at: 200,
            icon_hash: None,
            poster_hash: None,
            created_at: 1,
        }
    }

    #[test]
    fn paused_game_is_locked() {
        let mut game = game();
        game.paused = true;

        assert!(matches!(
            ensure_game_not_paused(&game),
            Err(WebError::Locked(_))
        ));
    }

    #[test]
    fn competition_window_includes_both_boundaries() {
        let game = game();

        assert!(ensure_game_ongoing(&game, 100).is_ok());
        assert!(ensure_game_ongoing(&game, 200).is_ok());
        assert!(matches!(
            ensure_game_ongoing(&game, 99),
            Err(WebError::Forbidden(_))
        ));
        assert!(matches!(
            ensure_game_ongoing(&game, 201),
            Err(WebError::Forbidden(_))
        ));
    }
}
