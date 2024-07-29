use sea_orm::{ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, PaginatorTrait, QueryFilter, QuerySelect, TryIntoModel};

use crate::database::get_db;

pub async fn find(
    id: Option<i64>, title: Option<String>, is_enabled: Option<bool>, page: Option<u64>, size: Option<u64>,
) -> Result<(Vec<crate::model::game::Model>, u64), DbErr> {
    let mut query = crate::model::game::Entity::find();

    if let Some(id) = id {
        query = query.filter(crate::model::game::Column::Id.eq(id));
    }

    if let Some(title) = title {
        query = query.filter(crate::model::game::Column::Title.contains(title));
    }

    if let Some(is_enabled) = is_enabled {
        query = query.filter(crate::model::game::Column::IsEnabled.eq(is_enabled));
    }

    let total = query.clone().count(&get_db().await).await?;

    if let Some(page) = page {
        if let Some(size) = size {
            let offset = (page - 1) * size;
            query = query.offset(offset).limit(size);
        }
    }

    let games = query.all(&get_db().await).await?;

    return Ok((games, total));
}

pub async fn create(game: crate::model::game::ActiveModel) -> Result<crate::model::game::Model, DbErr> {
    game.insert(&get_db().await).await?.try_into_model()
}

pub async fn update(game: crate::model::game::ActiveModel) -> Result<crate::model::game::Model, DbErr> {
    game.update(&get_db().await).await?.try_into_model()
}

pub async fn delete(id: i64) -> Result<(), DbErr> {
    let result = crate::model::game::Entity::delete_by_id(id).exec(&get_db().await).await?;
    Ok(if result.rows_affected == 0 {
        return Err(DbErr::RecordNotFound(format!("Game with id {} not found", id)));
    })
}
