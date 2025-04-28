use cds_db::{
    get_db,
    sea_orm::{ColumnTrait, EntityTrait, JoinType, QueryFilter, QuerySelect, RelationTrait},
    transfer::{Challenge, Game, GameChallenge, Team, User},
};
use serde_json::json;
use uuid::Uuid;
use cds_db::traits::EagerLoading;
use crate::traits::WebError;

pub async fn prepare_challenge(challenge_id: Uuid) -> Result<Challenge, WebError> {
    let challenge = cds_db::entity::challenge::Entity::find()
        .filter(cds_db::entity::challenge::Column::Id.eq(challenge_id))
        .one(get_db())
        .await?
        .map(cds_db::transfer::Challenge::from)
        .ok_or(WebError::NotFound(json!("challenge_not_found")))?;

    Ok(challenge)
}

pub async fn prepare_game(game_id: i64) -> Result<Game, WebError> {
    let game = cds_db::entity::game::Entity::find()
        .filter(cds_db::entity::game::Column::Id.eq(game_id))
        .one(get_db())
        .await?
        .map(cds_db::transfer::Game::from)
        .ok_or(WebError::NotFound(json!("game_not_found")))?;

    Ok(game)
}

pub async fn prepare_game_challenge(
    game_id: i64,
    challenge_id: Uuid,
) -> Result<GameChallenge, WebError> {
    let game_challenge = cds_db::entity::game_challenge::Entity::find()
        .filter(cds_db::entity::game_challenge::Column::GameId.eq(game_id))
        .filter(cds_db::entity::game_challenge::Column::ChallengeId.eq(challenge_id))
        .one(get_db())
        .await?
        .map(cds_db::transfer::GameChallenge::from)
        .ok_or(WebError::NotFound(json!("game_challenge_not_found")))?;

    Ok(game_challenge)
}

pub async fn prepare_self_team(game_id: i64, user_id: i64) -> Result<Team, WebError> {
    let teams = cds_db::entity::team::Entity::find()
        .filter(cds_db::entity::team::Column::GameId.eq(game_id))
        .join(
            JoinType::InnerJoin,
            cds_db::entity::team_user::Relation::Team.def().rev(),
        )
        .filter(cds_db::entity::team_user::Column::UserId.eq(user_id))
        .all(get_db())
        .await?
        .eager_load(get_db())
        .await?;

    teams
        .into_iter()
        .next()
        .ok_or(WebError::NotFound(json!("team_not_found")))
}

pub async fn prepare_team(game_id: i64, team_id: i64) -> Result<Team, WebError> {
    let teams = cds_db::entity::team::Entity::find()
        .filter(cds_db::entity::team::Column::GameId.eq(game_id))
        .filter(cds_db::entity::team::Column::Id.eq(team_id))
        .all(get_db())
        .await?
        .eager_load(get_db())
        .await?;

    teams
        .into_iter()
        .next()
        .ok_or(WebError::NotFound(json!("team_not_found")))
}

pub async fn prepare_user(user_id: i64) -> Result<User, WebError> {
    let user = cds_db::entity::user::Entity::find()
        .filter(cds_db::entity::user::Column::Id.eq(user_id))
        .one(get_db())
        .await?
        .map(cds_db::transfer::User::from)
        .ok_or(WebError::NotFound(json!("user_not_found")))?;

    Ok(user)
}
