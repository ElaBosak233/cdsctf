use cds_db::{
    get_db,
    sea_orm::{ColumnTrait, EntityTrait, JoinType, QueryFilter, QuerySelect, RelationTrait},
};
use serde_json::json;
use uuid::Uuid;

use crate::{
    model::{game::Game, game_challenge::GameChallenge, team::Team, user::User},
    traits::WebError,
};

pub async fn prepare_challenge(
    challenge_id: Uuid,
) -> Result<cds_db::entity::challenge::Model, WebError> {
    let challenge = cds_db::entity::challenge::Entity::find()
        .filter(cds_db::entity::challenge::Column::Id.eq(challenge_id))
        .filter(cds_db::entity::challenge::Column::DeletedAt.is_null())
        .one(get_db())
        .await?
        .ok_or(WebError::NotFound(json!("challenge_not_found")))?;

    Ok(challenge)
}

pub async fn prepare_game(game_id: i64) -> Result<Game, WebError> {
    let game = cds_db::entity::game::Entity::find()
        .filter(cds_db::entity::game::Column::Id.eq(game_id))
        .into_model::<Game>()
        .one(get_db())
        .await?
        .ok_or(WebError::NotFound(json!("game_not_found")))?;

    Ok(game)
}

pub async fn prepare_game_challenge(
    game_id: i64,
    challenge_id: Uuid,
) -> Result<GameChallenge, WebError> {
    let game_challenge = cds_db::entity::game_challenge::Entity::base_find()
        .filter(cds_db::entity::game_challenge::Column::GameId.eq(game_id))
        .filter(cds_db::entity::game_challenge::Column::ChallengeId.eq(challenge_id))
        .into_model::<GameChallenge>()
        .one(get_db())
        .await?
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
        .into_model::<Team>()
        .all(get_db())
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
        .into_model::<Team>()
        .all(get_db())
        .await?;

    teams
        .into_iter()
        .next()
        .ok_or(WebError::NotFound(json!("team_not_found")))
}

pub async fn prepare_user(user_id: i64) -> Result<User, WebError> {
    let user = cds_db::entity::user::Entity::find()
        .filter(cds_db::entity::user::Column::Id.eq(user_id))
        .into_model::<User>()
        .one(get_db())
        .await?
        .ok_or(WebError::NotFound(json!("user_not_found")))?;

    Ok(user)
}
