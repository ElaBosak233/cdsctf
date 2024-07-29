use sea_orm::{ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, PaginatorTrait, QueryFilter, QuerySelect, TryIntoModel};

use crate::database::get_db;

pub async fn find(
    id: Option<i64>, title: Option<String>, category_id: Option<i64>, is_practicable: Option<bool>, is_dynamic: Option<bool>, page: Option<u64>,
    size: Option<u64>,
) -> Result<(Vec<crate::model::challenge::Model>, u64), DbErr> {
    let mut query = crate::model::challenge::Entity::find();

    if let Some(id) = id {
        query = query.filter(crate::model::challenge::Column::Id.eq(id));
    }

    if let Some(title) = title {
        query = query.filter(crate::model::challenge::Column::Title.contains(title));
    }

    if let Some(category_id) = category_id {
        query = query.filter(crate::model::challenge::Column::CategoryId.eq(category_id));
    }

    if let Some(is_practicable) = is_practicable {
        query = query.filter(crate::model::challenge::Column::IsPracticable.eq(is_practicable));
    }

    if let Some(is_dynamic) = is_dynamic {
        query = query.filter(crate::model::challenge::Column::IsDynamic.eq(is_dynamic));
    }

    let total = query.clone().count(&get_db().await).await?;

    if let Some(page) = page {
        if let Some(size) = size {
            let offset = (page - 1) * size;
            query = query.offset(offset).limit(size);
        }
    }

    let challenges = query.all(&get_db().await).await?;

    return Ok((challenges, total));
}

pub async fn find_by_ids(ids: Vec<i64>) -> Result<Vec<crate::model::challenge::Model>, DbErr> {
    let challenges = crate::model::challenge::Entity::find()
        .filter(crate::model::challenge::Column::Id.is_in(ids))
        .all(&get_db().await)
        .await?;

    return Ok(challenges);
}

pub async fn create(challenge: crate::model::challenge::ActiveModel) -> Result<crate::model::challenge::Model, DbErr> {
    challenge.insert(&get_db().await).await?.try_into_model()
}

pub async fn update(challenge: crate::model::challenge::ActiveModel) -> Result<crate::model::challenge::Model, DbErr> {
    challenge.update(&get_db().await).await?.try_into_model()
}

pub async fn delete(id: i64) -> Result<(), DbErr> {
    let result = crate::model::challenge::Entity::delete_by_id(id).exec(&get_db().await).await?;
    Ok(if result.rows_affected == 0 {
        return Err(DbErr::RecordNotFound(format!("Challenge with id {} not found", id)));
    })
}
