use cds_db::{
    Challenge, Game, GameChallenge, User,
    sea_orm::DatabaseConnection,
    team::{FindTeamOptions, Team},
};
use serde_json::json;

use crate::traits::WebError;

pub async fn prepare_challenge(
    db: &DatabaseConnection,
    challenge_id: i64,
) -> Result<Challenge, WebError> {
    let challenge = cds_db::challenge::find_by_id(db, challenge_id)
        .await?
        .ok_or(WebError::NotFound(json!("challenge_not_found")))?;

    Ok(challenge)
}

pub async fn prepare_game(db: &DatabaseConnection, game_id: i64) -> Result<Game, WebError> {
    let game = cds_db::game::find_by_id(db, game_id)
        .await?
        .ok_or(WebError::NotFound(json!("challenge_not_found")))?;

    Ok(game)
}

pub async fn prepare_game_challenge(
    db: &DatabaseConnection,
    game_id: i64,
    challenge_id: i64,
) -> Result<GameChallenge, WebError> {
    let game_challenge = cds_db::game_challenge::find_by_id(db, game_id, challenge_id)
        .await?
        .ok_or(WebError::NotFound(json!("game_challenge_not_found")))?;

    Ok(game_challenge)
}

pub async fn prepare_self_team(
    db: &DatabaseConnection,
    game_id: i64,
    user_id: i64,
) -> Result<Team, WebError> {
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

pub async fn prepare_team(
    db: &DatabaseConnection,
    game_id: i64,
    team_id: i64,
) -> Result<Team, WebError> {
    cds_db::team::find_by_id(db, team_id, game_id)
        .await?
        .ok_or(WebError::NotFound(json!("team_not_found")))
}

pub async fn prepare_user(db: &DatabaseConnection, user_id: i64) -> Result<User, WebError> {
    let user = cds_db::user::find_by_id(db, user_id)
        .await?
        .ok_or(WebError::NotFound(json!("user_not_found")))?;

    Ok(user)
}
