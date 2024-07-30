use std::error::Error;

pub async fn join(
    req: crate::model::user_team::request::JoinRequest,
) -> Result<(), Box<dyn Error>> {
    let (_, user_total) =
        crate::model::user::find(Some(req.user_id), None, None, None, None, None, None)
            .await
            .unwrap();
    let (teams, team_total) = crate::model::team::find(Some(req.team_id), None, None, None, None)
        .await
        .unwrap();

    if user_total == 0 || team_total == 0 {
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
