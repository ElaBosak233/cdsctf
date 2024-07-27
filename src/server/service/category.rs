use std::error::Error;

use sea_orm::TryIntoModel;

pub async fn find(
    req: crate::model::category::request::FindRequest,
) -> Result<(Vec<crate::model::category::Model>, u64), Box<dyn Error>> {
    let (categories, total) = crate::repository::category::find(req.id, req.name)
        .await
        .unwrap();
    return Ok((categories, total));
}

pub async fn create(
    req: crate::model::category::request::CreateRequest,
) -> Result<crate::model::category::Model, Box<dyn Error>> {
    match crate::repository::category::create(req.into()).await {
        Ok(port) => return Ok(port.try_into_model().unwrap()),
        Err(err) => return Err(Box::new(err)),
    }
}

pub async fn update(
    req: crate::model::category::request::UpdateRequest,
) -> Result<(), Box<dyn Error>> {
    match crate::repository::category::update(req.into()).await {
        Ok(_) => return Ok(()),
        Err(err) => return Err(Box::new(err)),
    }
}

pub async fn delete(id: i64) -> Result<(), Box<dyn Error>> {
    match crate::repository::category::delete(id).await {
        Ok(_) => return Ok(()),
        Err(err) => return Err(Box::new(err)),
    }
}
