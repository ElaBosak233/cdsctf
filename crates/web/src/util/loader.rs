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
pub fn ensure_game_ongoing(game: &GameDetail, now: time::OffsetDateTime) -> Result<(), WebError> {
    if !(game.started_at..=game.ended_at).contains(&now) {
        return Err(WebError::Forbidden(json!("game_not_ongoing")));
    }

    Ok(())
}

/// Rejects write-up access when the game does not require write-ups.
pub fn ensure_writeup_required(game: &GameDetail) -> Result<(), WebError> {
    if !game.writeup_required {
        return Err(WebError::BadRequest(json!("writeup_not_required")));
    }

    Ok(())
}

/// Rejects write-up submission unless the game requires write-ups and is
/// ongoing.
pub fn ensure_writeup_available(
    game: &GameDetail,
    now: time::OffsetDateTime,
) -> Result<(), WebError> {
    ensure_writeup_required(game)?;
    ensure_game_ongoing(game, now)
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

    use super::{
        ensure_game_not_paused, ensure_game_ongoing, ensure_writeup_available,
        ensure_writeup_required,
    };
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
            started_at: time::OffsetDateTime::from_unix_timestamp(100).unwrap(),
            frozen_at: time::OffsetDateTime::from_unix_timestamp(150).unwrap(),
            ended_at: time::OffsetDateTime::from_unix_timestamp(200).unwrap(),
            icon_hash: None,
            poster_hash: None,
            created_at: time::OffsetDateTime::from_unix_timestamp(1).unwrap(),
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

        let start = time::OffsetDateTime::from_unix_timestamp(100).unwrap();
        let end = time::OffsetDateTime::from_unix_timestamp(200).unwrap();
        assert!(ensure_game_ongoing(&game, start).is_ok());
        assert!(ensure_game_ongoing(&game, end).is_ok());
        assert!(matches!(
            ensure_game_ongoing(
                &game,
                time::OffsetDateTime::from_unix_timestamp(99).unwrap()
            ),
            Err(WebError::Forbidden(_))
        ));
        assert!(matches!(
            ensure_game_ongoing(
                &game,
                time::OffsetDateTime::from_unix_timestamp(201).unwrap()
            ),
            Err(WebError::Forbidden(_))
        ));
    }

    #[test]
    fn writeup_requires_an_enabled_ongoing_game() {
        let game = game();
        let now = time::OffsetDateTime::from_unix_timestamp(150).unwrap();

        assert!(matches!(
            ensure_writeup_required(&game),
            Err(WebError::BadRequest(_))
        ));
        assert!(matches!(
            ensure_writeup_available(&game, now),
            Err(WebError::BadRequest(_))
        ));

        let mut game = game;
        game.writeup_required = true;
        assert!(ensure_writeup_available(&game, now).is_ok());
        assert!(matches!(
            ensure_writeup_available(
                &game,
                time::OffsetDateTime::from_unix_timestamp(99).unwrap()
            ),
            Err(WebError::Forbidden(_))
        ));
    }
}
