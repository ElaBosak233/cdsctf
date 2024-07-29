use sea_orm::{ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, LoaderTrait, PaginatorTrait, QueryFilter, QuerySelect, TryIntoModel};

use crate::database::get_db;

pub async fn preload(mut submissions: Vec<crate::model::submission::Model>) -> Result<Vec<crate::model::submission::Model>, DbErr> {
    let users = submissions.load_one(crate::model::user::Entity, &get_db().await).await?;
    let challenges = submissions.load_one(crate::model::challenge::Entity, &get_db().await).await?;
    let teams = submissions.load_one(crate::model::team::Entity, &get_db().await).await?;
    let games = submissions.load_one(crate::model::game::Entity, &get_db().await).await?;

    for i in 0..submissions.len() {
        let mut submission = submissions[i].clone();
        submission.user = users[i].clone();
        submission.challenge = challenges[i].clone();
        submission.team = teams[i].clone();
        submission.game = games[i].clone();
        submissions[i] = submission;
    }
    return Ok(submissions);
}

pub async fn find(
    id: Option<i64>, user_id: Option<i64>, team_id: Option<i64>, game_id: Option<i64>, challenge_id: Option<i64>, status: Option<i64>, page: Option<u64>,
    size: Option<u64>,
) -> Result<(Vec<crate::model::submission::Model>, u64), DbErr> {
    let mut query = crate::model::submission::Entity::find();

    if let Some(id) = id {
        query = query.filter(crate::model::submission::Column::Id.eq(id));
    }

    if let Some(user_id) = user_id {
        query = query.filter(crate::model::submission::Column::UserId.eq(user_id));
    }

    if let Some(team_id) = team_id {
        query = query.filter(crate::model::submission::Column::TeamId.eq(team_id));
    }

    if let Some(game_id) = game_id {
        query = query.filter(crate::model::submission::Column::GameId.eq(game_id));
    }

    if let Some(challenge_id) = challenge_id {
        query = query.filter(crate::model::submission::Column::ChallengeId.eq(challenge_id));
    }

    if let Some(status) = status {
        query = query.filter(crate::model::submission::Column::Status.eq(status));
    }

    let total = query.clone().count(&get_db().await).await?;

    if let Some(page) = page {
        if let Some(size) = size {
            let offset = (page - 1) * size;
            query = query.offset(offset).limit(size);
        }
    }

    let mut submissions = query.all(&get_db().await).await?;

    submissions = preload(submissions).await?;

    return Ok((submissions, total));
}

pub async fn find_by_challenge_ids(challenge_ids: Vec<i64>) -> Result<Vec<crate::model::submission::Model>, DbErr> {
    let mut submissions = crate::model::submission::Entity::find()
        .filter(crate::model::submission::Column::ChallengeId.is_in(challenge_ids))
        .all(&get_db().await)
        .await?;
    submissions = preload(submissions).await?;
    return Ok(submissions);
}

pub async fn create(submission: crate::model::submission::ActiveModel) -> Result<crate::model::submission::Model, DbErr> {
    return submission.insert(&get_db().await).await?.try_into_model();
}

pub async fn update(submission: crate::model::submission::ActiveModel) -> Result<crate::model::submission::Model, DbErr> {
    return submission.update(&get_db().await).await?.try_into_model();
}

pub async fn delete(id: i64) -> Result<(), DbErr> {
    let result = crate::model::submission::Entity::delete_by_id(id).exec(&get_db().await).await?;
    return Ok(if result.rows_affected == 0 {
        return Err(DbErr::RecordNotFound(format!("Submission with id {} not found", id)));
    });
}
