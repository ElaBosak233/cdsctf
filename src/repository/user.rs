use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DbErr, EntityTrait, LoaderTrait, PaginatorTrait,
    QueryFilter, QuerySelect, TryIntoModel,
};

use crate::database::get_db;

async fn preload(
    mut users: Vec<crate::model::user::Model>,
) -> Result<Vec<crate::model::user::Model>, DbErr> {
    let teams = users
        .load_many_to_many(
            crate::model::team::Entity,
            crate::model::user_team::Entity,
            &get_db().await,
        )
        .await?;

    for i in 0..users.len() {
        let mut user = users[i].clone();
        user.teams = teams[i].clone();
        users[i] = user;
    }

    return Ok(users);
}

pub async fn find(
    id: Option<i64>,
    name: Option<String>,
    username: Option<String>,
    group: Option<String>,
    email: Option<String>,
    page: Option<u64>,
    size: Option<u64>,
) -> Result<(Vec<crate::model::user::Model>, u64), DbErr> {
    let mut query = crate::model::user::Entity::find();

    if let Some(id) = id {
        query = query.filter(crate::model::user::Column::Id.eq(id));
    }

    if let Some(name) = name {
        let pattern = format!("%{}%", name);
        let condition = Condition::any()
            .add(crate::model::user::Column::Username.like(&pattern))
            .add(crate::model::user::Column::Nickname.like(&pattern));
        query = query.filter(condition);
    }

    if let Some(username) = username {
        query = query.filter(crate::model::user::Column::Username.eq(username));
    }

    if let Some(group) = group {
        query = query.filter(crate::model::user::Column::Group.eq(group));
    }

    if let Some(email) = email {
        query = query.filter(crate::model::user::Column::Email.eq(email));
    }

    let total = query.clone().count(&get_db().await).await?;

    if let Some(page) = page {
        if let Some(size) = size {
            let offset = (page - 1) * size;
            query = query.offset(offset).limit(size);
        }
    }

    let mut users = query.all(&get_db().await).await?;

    users = preload(users).await?;

    Ok((users, total))
}

pub async fn create(
    user: crate::model::user::ActiveModel,
) -> Result<crate::model::user::Model, DbErr> {
    user.insert(&get_db().await).await?.try_into_model()
}

pub async fn update(
    user: crate::model::user::ActiveModel,
) -> Result<crate::model::user::Model, DbErr> {
    user.update(&get_db().await).await?.try_into_model()
}

pub async fn delete(id: i64) -> Result<(), DbErr> {
    let result = crate::model::user::Entity::delete_by_id(id)
        .exec(&get_db().await)
        .await?;
    Ok(if result.rows_affected == 0 {
        return Err(DbErr::RecordNotFound(format!(
            "User with id {} not found",
            id
        )));
    })
}
