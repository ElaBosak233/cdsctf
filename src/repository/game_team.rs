use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, PaginatorTrait, QueryFilter, TryIntoModel,
};

use crate::database::get_db;

async fn preload(
    mut game_teams: Vec<crate::model::game_team::Model>,
) -> Result<Vec<crate::model::game_team::Model>, DbErr> {
    let team_ids: Vec<i64> = game_teams
        .iter()
        .map(|game_team| game_team.team_id)
        .collect();

    let teams = super::team::find_by_ids(team_ids).await?;

    for game_team in game_teams.iter_mut() {
        game_team.team = teams
            .iter()
            .find(|team| team.id == game_team.team_id)
            .cloned();
    }

    return Ok(game_teams);
}

pub async fn find(
    game_id: Option<i64>,
    team_id: Option<i64>,
) -> Result<(Vec<crate::model::game_team::Model>, u64), DbErr> {
    let mut query = crate::model::game_team::Entity::find();

    if let Some(game_id) = game_id {
        query = query.filter(crate::model::game_team::Column::GameId.eq(game_id));
    }

    if let Some(team_id) = team_id {
        query = query.filter(crate::model::game_team::Column::TeamId.eq(team_id));
    }

    let total = query.clone().count(&get_db().await).await?;

    let mut game_teams = query.all(&get_db().await).await?;

    game_teams = preload(game_teams).await?;

    Ok((game_teams, total))
}

pub async fn create(
    user: crate::model::game_team::ActiveModel,
) -> Result<crate::model::game_team::Model, DbErr> {
    user.insert(&get_db().await).await?.try_into_model()
}

pub async fn update(
    user: crate::model::game_team::ActiveModel,
) -> Result<crate::model::game_team::Model, DbErr> {
    user.update(&get_db().await).await?.try_into_model()
}

pub async fn delete(game_id: i64, team_id: i64) -> Result<(), DbErr> {
    let _result = crate::model::game_team::Entity::delete_many()
        .filter(crate::model::game_team::Column::GameId.eq(game_id))
        .filter(crate::model::game_team::Column::TeamId.eq(team_id))
        .exec(&get_db().await)
        .await?;

    return Ok(());
}
