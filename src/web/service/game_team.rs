use std::error::Error;

pub async fn find(
    req: crate::model::game_team::request::FindRequest,
) -> Result<(Vec<crate::model::game_team::Model>, u64), Box<dyn Error>> {
    let (mut game_teams, total) = crate::model::game_team::find(req.game_id, req.team_id)
        .await
        .unwrap();

    for game_team in game_teams.iter_mut() {
        if let Some(team) = game_team.team.as_mut() {
            team.simplify();
        }
    }

    return Ok((game_teams, total));
}

pub async fn create(
    req: crate::model::game_team::request::CreateRequest,
) -> Result<(), Box<dyn Error>> {
    match crate::model::game_team::create(req.into()).await {
        Ok(_) => {
            return Ok(());
        }
        Err(e) => {
            return Err(Box::new(e));
        }
    };
}

pub async fn update(
    req: crate::model::game_team::request::UpdateRequest,
) -> Result<(), Box<dyn Error>> {
    match crate::model::game_team::update(req.into()).await {
        Ok(_) => {
            return Ok(());
        }
        Err(e) => {
            return Err(Box::new(e));
        }
    }
}

pub async fn delete(id: i64, challenge_id: i64) -> Result<(), Box<dyn Error>> {
    match crate::model::game_team::delete(id, challenge_id).await {
        Ok(_) => {
            return Ok(());
        }
        Err(e) => {
            return Err(Box::new(e));
        }
    }
}
