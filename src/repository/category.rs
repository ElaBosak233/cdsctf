use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, PaginatorTrait, QueryFilter, TryIntoModel,
};

use crate::database::get_db;

pub async fn find(
    id: Option<i64>,
    name: Option<String>,
) -> Result<(Vec<crate::model::category::Model>, u64), DbErr> {
    let mut query = crate::model::category::Entity::find();
    if let Some(id) = id {
        query = query.filter(crate::model::category::Column::Id.eq(id));
    }
    if let Some(name) = name {
        query = query.filter(crate::model::category::Column::Name.eq(name));
    }
    let total = query.clone().count(&get_db().await).await?;
    let categories = query.all(&get_db().await).await?;
    Ok((categories, total))
}

pub async fn create(
    category: crate::model::category::ActiveModel,
) -> Result<crate::model::category::Model, DbErr> {
    category.insert(&get_db().await).await?.try_into_model()
}

pub async fn update(
    category: crate::model::category::ActiveModel,
) -> Result<crate::model::category::Model, DbErr> {
    category.update(&get_db().await).await?.try_into_model()
}

pub async fn delete(id: i64) -> Result<(), DbErr> {
    let result = crate::model::category::Entity::delete_by_id(id)
        .exec(&get_db().await)
        .await?;
    Ok(if result.rows_affected == 0 {
        return Err(DbErr::RecordNotFound(format!(
            "Category with id {} not found",
            id
        )));
    })
}
