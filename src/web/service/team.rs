use std::error::Error;

use sea_orm::{ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter, Set};

use crate::database::get_db;

pub async fn find(
    req: crate::model::team::request::FindRequest,
) -> Result<(Vec<crate::model::team::Model>, u64), ()> {
    let (teams, total) = crate::model::team::find(req.id, req.name, req.email, req.page, req.size)
        .await
        .unwrap();
    return Ok((teams, total));
}

pub async fn find_by_user_id(id: i64) -> Result<Vec<crate::model::team::Model>, Box<dyn Error>> {
    let mut teams = crate::model::team::find_by_user_id(id).await.unwrap();
    for team in teams.iter_mut() {
        team.simplify();
    }
    return Ok(teams);
}

pub async fn create(req: crate::model::team::request::CreateRequest) -> Result<(), Box<dyn Error>> {
    match crate::model::team::create(req.clone().into()).await {
        Ok(team) => {
            match crate::model::user_team::create(crate::model::user_team::ActiveModel {
                team_id: Set(team.id),
                user_id: Set(req.captain_id),
            })
            .await
            {
                Ok(_) => {
                    return Ok(());
                }
                Err(err) => return Err(Box::new(err)),
            }
        }
        Err(err) => return Err(Box::new(err)),
    }
}

pub async fn update(req: crate::model::team::request::UpdateRequest) -> Result<(), Box<dyn Error>> {
    match crate::model::team::update(req.into()).await {
        Ok(_) => return Ok(()),
        Err(err) => return Err(Box::new(err)),
    }
}

pub async fn delete(id: i64) -> Result<(), Box<dyn Error>> {
    match crate::model::team::delete(id).await {
        Ok(()) => return Ok(()),
        Err(err) => return Err(Box::new(err)),
    }
}

pub async fn get_invite_token(id: i64) -> Result<String, Box<dyn Error>> {
    let teams = crate::model::team::Entity::find()
        .filter(crate::model::team::Column::Id.eq(id))
        .all(&get_db())
        .await
        .unwrap();

    if teams.is_empty() {
        return Err("team_not_found".into());
    }

    let team = teams.get(0).unwrap();

    return Ok(team.invite_token.clone().unwrap_or("".to_string()));
}

pub async fn update_invite_token(id: i64) -> Result<String, Box<dyn Error>> {
    let teams = crate::model::team::Entity::find()
        .filter(crate::model::team::Column::Id.eq(id))
        .all(&get_db())
        .await
        .unwrap();

    if teams.is_empty() {
        return Err("team_not_found".into());
    }

    let mut team = teams.get(0).unwrap().clone().into_active_model();
    let token = uuid::Uuid::new_v4().simple().to_string();
    team.invite_token = Set(Some(token.clone()));

    match crate::model::team::update(team).await {
        Ok(_) => return Ok(token),
        Err(err) => return Err(Box::new(err)),
    }
}
