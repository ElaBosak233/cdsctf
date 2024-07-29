use std::error::Error;

use sea_orm::TryIntoModel;

pub async fn find(req: crate::model::game::request::FindRequest) -> Result<(Vec<crate::model::game::Model>, u64), Box<dyn Error>> {
    let (games, total) = crate::repository::game::find(req.id, req.title, req.is_enabled, req.page, req.size)
        .await
        .unwrap();

    return Ok((games, total));
}

pub async fn create(req: crate::model::game::request::CreateRequest) -> Result<crate::model::game::Model, Box<dyn Error>> {
    match crate::repository::game::create(req.into()).await {
        Ok(game) => return Ok(game.try_into_model().unwrap()),
        Err(err) => return Err(Box::new(err)),
    }
}

pub async fn update(req: crate::model::game::request::UpdateRequest) -> Result<(), Box<dyn Error>> {
    match crate::repository::game::update(req.into()).await {
        Ok(_game) => return Ok(()),
        Err(err) => return Err(Box::new(err)),
    }
}

pub async fn delete(id: i64) -> Result<(), Box<dyn Error>> {
    match crate::repository::game::delete(id).await {
        Ok(_) => return Ok(()),
        Err(err) => return Err(Box::new(err)),
    }
}
