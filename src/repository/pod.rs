use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, LoaderTrait, PaginatorTrait, QueryFilter,
    TryIntoModel,
};

use crate::database::get_db;

async fn preload(
    mut pods: Vec<crate::model::pod::Model>,
) -> Result<Vec<crate::model::pod::Model>, DbErr> {
    let users = pods
        .load_one(crate::model::user::Entity, &get_db().await)
        .await?;
    let teams = pods
        .load_one(crate::model::team::Entity, &get_db().await)
        .await?;
    let challenges = pods
        .load_one(crate::model::challenge::Entity, &get_db().await)
        .await?;

    for i in 0..pods.len() {
        let mut pod = pods[i].clone();
        pod.user = users[i].clone();
        pod.team = teams[i].clone();
        pod.challenge = challenges[i].clone();
        pods[i] = pod;
    }

    return Ok(pods);
}

pub async fn find(
    id: Option<i64>,
    name: Option<String>,
    user_id: Option<i64>,
    team_id: Option<i64>,
    game_id: Option<i64>,
    challenge_id: Option<i64>,
    is_available: Option<bool>,
) -> Result<(Vec<crate::model::pod::Model>, u64), DbErr> {
    let mut query = crate::model::pod::Entity::find();
    if let Some(id) = id {
        query = query.filter(crate::model::pod::Column::Id.eq(id));
    }

    if let Some(name) = name {
        query = query.filter(crate::model::pod::Column::Name.eq(name));
    }

    if let Some(user_id) = user_id {
        query = query.filter(crate::model::pod::Column::UserId.eq(user_id));
    }

    if let Some(team_id) = team_id {
        query = query.filter(crate::model::pod::Column::TeamId.eq(team_id));
    }

    if let Some(game_id) = game_id {
        query = query.filter(crate::model::pod::Column::GameId.eq(game_id));
    }

    if let Some(challenge_id) = challenge_id {
        query = query.filter(crate::model::pod::Column::ChallengeId.eq(challenge_id));
    }

    if let Some(is_available) = is_available {
        match is_available {
            true => {
                query = query.filter(
                    crate::model::pod::Column::RemovedAt.gte(chrono::Utc::now().timestamp()),
                )
            }
            false => {
                query = query.filter(
                    crate::model::pod::Column::RemovedAt.lte(chrono::Utc::now().timestamp()),
                )
            }
        }
    }

    let total = query.clone().count(&get_db().await).await?;

    let mut pods = query.all(&get_db().await).await?;

    pods = preload(pods).await?;

    return Ok((pods, total));
}

pub async fn create(
    pod: crate::model::pod::ActiveModel,
) -> Result<crate::model::pod::Model, DbErr> {
    return pod.insert(&get_db().await).await?.try_into_model();
}

pub async fn update(
    pod: crate::model::pod::ActiveModel,
) -> Result<crate::model::pod::Model, DbErr> {
    return pod.update(&get_db().await).await?.try_into_model();
}

pub async fn delete(id: i64) -> Result<(), DbErr> {
    let result = crate::model::pod::Entity::delete_by_id(id)
        .exec(&get_db().await)
        .await?;
    return Ok(if result.rows_affected == 0 {
        return Err(DbErr::RecordNotFound(format!(
            "Pod with id {} not found",
            id
        )));
    });
}
