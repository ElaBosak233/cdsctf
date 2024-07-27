use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, PaginatorTrait, QueryFilter, TryIntoModel,
};

use crate::database::get_db;

pub async fn find(
    user_id: Option<i64>,
    team_id: Option<i64>,
) -> Result<(Vec<crate::model::user_team::Model>, u64), DbErr> {
    let mut query = crate::model::user_team::Entity::find();

    if let Some(user_id) = user_id {
        query = query.filter(crate::model::user_team::Column::UserId.eq(user_id));
    }

    if let Some(team_id) = team_id {
        query = query.filter(crate::model::user_team::Column::TeamId.eq(team_id));
    }

    let total = query.clone().count(&get_db().await).await?;

    let user_teams = query.all(&get_db().await).await?;

    Ok((user_teams, total))
}

pub async fn create(
    user_team: crate::model::user_team::ActiveModel,
) -> Result<crate::model::user_team::Model, DbErr> {
    user_team.insert(&get_db().await).await?.try_into_model()
}

pub async fn update(
    user_team: crate::model::user_team::ActiveModel,
) -> Result<crate::model::user_team::Model, DbErr> {
    user_team.update(&get_db().await).await?.try_into_model()
}

pub async fn delete(user_id: i64, team_id: i64) -> Result<(), DbErr> {
    let _result: sea_orm::DeleteResult = crate::model::user_team::Entity::delete_many()
        .filter(crate::model::user_team::Column::UserId.eq(user_id))
        .filter(crate::model::user_team::Column::TeamId.eq(team_id))
        .exec(&get_db().await)
        .await?;

    return Ok(());
}
