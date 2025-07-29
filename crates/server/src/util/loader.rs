use cds_db::{
    Challenge, Game, GameChallenge, User,
    team::{FindTeamOptions, Team},
};
use serde_json::json;
use uuid::Uuid;

use crate::traits::WebError;

pub async fn prepare_challenge(challenge_id: Uuid) -> Result<Challenge, WebError> {
    let challenge = cds_db::challenge::find_by_id(challenge_id)
        .await?
        .ok_or(WebError::NotFound(json!("challenge_not_found")))?;

    Ok(challenge)
}

pub async fn prepare_game(game_id: i64) -> Result<Game, WebError> {
    let game = cds_db::game::find_by_id(game_id)
        .await?
        .ok_or(WebError::NotFound(json!("challenge_not_found")))?;

    Ok(game)
}

pub async fn prepare_game_challenge(
    game_id: i64,
    challenge_id: Uuid,
) -> Result<GameChallenge, WebError> {
    let game_challenge = cds_db::game_challenge::find_by_id::<GameChallenge>(game_id, challenge_id)
        .await?
        .unwrap();

    Ok(game_challenge)
}

pub async fn prepare_self_team(game_id: i64, user_id: i64) -> Result<Team, WebError> {
    let (teams, _) = cds_db::team::find::<Team>(FindTeamOptions {
        game_id: Some(game_id),
        user_id: Some(user_id),
        ..Default::default()
    })
    .await?;

    teams
        .into_iter()
        .next()
        .ok_or(WebError::NotFound(json!("team_not_found")))
}

pub async fn prepare_team(game_id: i64, team_id: i64) -> Result<Team, WebError> {
    cds_db::team::find_by_id::<Team>(team_id, game_id)
        .await?
        .ok_or(WebError::NotFound(json!("team_not_found")))
}

pub async fn prepare_user(user_id: i64) -> Result<User, WebError> {
    let user = cds_db::user::find_by_id(user_id)
        .await?
        .ok_or(WebError::NotFound(json!("user_not_found")))?;

    Ok(user)
}
