use sea_orm::{ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, LoaderTrait, PaginatorTrait, QueryFilter, TryIntoModel};

use crate::database::get_db;

async fn preload(mut game_challenges: Vec<crate::model::game_challenge::Model>) -> Result<Vec<crate::model::game_challenge::Model>, DbErr> {
    let challenges = game_challenges.load_one(crate::model::challenge::Entity, &get_db().await).await?;

    for i in 0..game_challenges.len() {
        game_challenges[i].challenge = challenges[i].clone();
    }

    return Ok(game_challenges);
}

pub async fn find(game_id: Option<i64>, challenge_id: Option<i64>) -> Result<(Vec<crate::model::game_challenge::Model>, u64), DbErr> {
    let mut query = crate::model::game_challenge::Entity::find();

    if let Some(game_id) = game_id {
        query = query.filter(crate::model::game_challenge::Column::GameId.eq(game_id));
    }

    if let Some(challenge_id) = challenge_id {
        query = query.filter(crate::model::game_challenge::Column::ChallengeId.eq(challenge_id));
    }

    let total = query.clone().count(&get_db().await).await?;

    let mut game_challenges = query.all(&get_db().await).await?;

    game_challenges = preload(game_challenges).await?;

    Ok((game_challenges, total))
}

pub async fn create(game_challenge: crate::model::game_challenge::ActiveModel) -> Result<crate::model::game_challenge::Model, DbErr> {
    game_challenge.insert(&get_db().await).await?.try_into_model()
}

pub async fn update(game_challenge: crate::model::game_challenge::ActiveModel) -> Result<crate::model::game_challenge::Model, DbErr> {
    game_challenge.update(&get_db().await).await?.try_into_model()
}

pub async fn delete(game_id: i64, challenge_id: i64) -> Result<(), DbErr> {
    let _result = crate::model::game_challenge::Entity::delete_many()
        .filter(crate::model::game_challenge::Column::GameId.eq(game_id))
        .filter(crate::model::game_challenge::Column::ChallengeId.eq(challenge_id))
        .exec(&get_db().await)
        .await?;

    return Ok(());
}
