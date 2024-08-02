use std::error::Error;

use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use crate::database::get_db;

pub async fn join(
    req: crate::model::user_team::request::JoinRequest,
) -> Result<(), Box<dyn Error>> {
    let users = crate::model::user::Entity::find()
        .filter(crate::model::user::Column::Id.eq(req.user_id))
        .all(&get_db())
        .await
        .unwrap();
    let teams = crate::model::team::Entity::find()
        .filter(crate::model::team::Column::Id.eq(req.team_id))
        .all(&get_db())
        .await
        .unwrap();

    if users.is_empty() || teams.is_empty() {
        return Err("invalid_user_or_team".into());
    }

    let team = teams.get(0).unwrap().clone();

    if Some(req.invite_token.clone()) != team.invite_token {
        return Err("invalid_invite_token".into());
    }

    crate::model::user_team::create(req.into()).await.unwrap();

    return Ok(());
}

pub async fn create(
    req: crate::model::user_team::request::CreateRequest,
) -> Result<(), Box<dyn Error>> {
    match crate::model::user_team::create(req.into()).await {
        Ok(_) => {
            return Ok(());
        }
        Err(e) => {
            return Err(e.into());
        }
    }
}

pub async fn delete(user_id: i64, team_id: i64) -> Result<(), Box<dyn Error>> {
    match crate::model::user_team::delete(user_id, team_id).await {
        Ok(_) => {
            return Ok(());
        }
        Err(e) => {
            return Err(e.into());
        }
    }
}
