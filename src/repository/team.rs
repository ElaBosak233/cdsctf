use sea_orm::{ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, LoaderTrait, PaginatorTrait, QueryFilter, QuerySelect, TryIntoModel};

use crate::database::get_db;

async fn preload(mut teams: Vec<crate::model::team::Model>) -> Result<Vec<crate::model::team::Model>, DbErr> {
    let users = teams
        .load_many_to_many(crate::model::user::Entity, crate::model::user_team::Entity, &get_db().await)
        .await?;

    for i in 0..teams.len() {
        let mut team = teams[i].clone();
        team.users = users[i].clone();
        for j in 0..team.users.len() {
            let mut user = team.users[j].clone();
            user.simplify();
            if user.id == team.captain_id {
                team.captain = Some(team.users[j].clone());
            }
            team.users[j] = user;
        }
        teams[i] = team;
    }

    return Ok(teams);
}

pub async fn find(
    id: Option<i64>, name: Option<String>, email: Option<String>, page: Option<u64>, size: Option<u64>,
) -> Result<(Vec<crate::model::team::Model>, u64), DbErr> {
    let mut query = crate::model::team::Entity::find();

    if let Some(id) = id {
        query = query.filter(crate::model::team::Column::Id.eq(id));
    }

    if let Some(name) = name {
        query = query.filter(crate::model::team::Column::Name.contains(name));
    }

    if let Some(email) = email {
        query = query.filter(crate::model::team::Column::Email.eq(email));
    }

    let total = query.clone().count(&get_db().await).await?;

    if let Some(page) = page {
        if let Some(size) = size {
            let offset = (page - 1) * size;
            query = query.offset(offset).limit(size);
        }
    }

    let mut teams = query.all(&get_db().await).await?;

    teams = preload(teams).await?;

    return Ok((teams, total));
}

pub async fn find_by_ids(ids: Vec<i64>) -> Result<Vec<crate::model::team::Model>, DbErr> {
    let mut teams = crate::model::team::Entity::find()
        .filter(crate::model::team::Column::Id.is_in(ids))
        .all(&get_db().await)
        .await?;

    teams = preload(teams).await?;

    return Ok(teams);
}

pub async fn create(team: crate::model::team::ActiveModel) -> Result<crate::model::team::Model, DbErr> {
    return team.insert(&get_db().await).await?.try_into_model();
}

pub async fn update(team: crate::model::team::ActiveModel) -> Result<crate::model::team::Model, DbErr> {
    return team.update(&get_db().await).await?.try_into_model();
}

pub async fn delete(id: i64) -> Result<(), DbErr> {
    let result = crate::model::team::Entity::delete_by_id(id).exec(&get_db().await).await?;
    return Ok(if result.rows_affected == 0 {
        return Err(DbErr::RecordNotFound(format!("Team with id {} not found", id)));
    });
}
