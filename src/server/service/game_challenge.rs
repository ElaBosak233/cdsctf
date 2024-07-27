use std::error::Error;

pub async fn find(
    req: crate::model::game_challenge::request::FindRequest,
) -> Result<(Vec<crate::model::game_challenge::Model>, u64), Box<dyn Error>> {
    let (game_challenges, total) =
        crate::repository::game_challenge::find(req.game_id, req.challenge_id)
            .await
            .unwrap();

    return Ok((game_challenges, total));
}

pub async fn create(
    req: crate::model::game_challenge::request::CreateRequest,
) -> Result<(), Box<dyn Error>> {
    match crate::repository::game_challenge::create(req.into()).await {
        Ok(_) => {
            return Ok(());
        }
        Err(e) => {
            return Err(Box::new(e));
        }
    };
}

pub async fn update(
    req: crate::model::game_challenge::request::UpdateRequest,
) -> Result<(), Box<dyn Error>> {
    match crate::repository::game_challenge::update(req.into()).await {
        Ok(_) => {
            return Ok(());
        }
        Err(e) => {
            return Err(Box::new(e));
        }
    };
}

pub async fn delete(id: i64, challenge_id: i64) -> Result<(), Box<dyn Error>> {
    match crate::repository::game_challenge::delete(id, challenge_id).await {
        Ok(_) => {
            return Ok(());
        }
        Err(e) => {
            return Err(Box::new(e));
        }
    }
}
