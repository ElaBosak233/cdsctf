use std::error::Error;

use regex::Regex;
use sea_orm::{ActiveModelTrait, IntoActiveModel, Set};
use uuid::Uuid;

use crate::{database::get_db, model::challenge::flag};

pub async fn create(
    req: crate::model::pod::request::CreateRequest,
) -> Result<crate::model::pod::Model, Box<dyn Error>> {
    let (challenges, _) = crate::model::challenge::find(
        Some(req.challenge_id.clone()),
        None,
        None,
        None,
        None,
        None,
        None,
    )
    .await
    .unwrap();

    let challenge = challenges.get(0).unwrap();

    let ctn_name = format!("cds-{}", Uuid::new_v4().simple().to_string());

    if challenge.flags.clone().into_iter().next().is_none() {
        return Err("No flags found".into());
    }

    let mut injected_flag = challenge.flags.clone().into_iter().next().unwrap();

    let re = Regex::new(r"\[([Uu][Uu][Ii][Dd])\]").unwrap();
    if injected_flag.type_ == flag::Type::Dynamic {
        injected_flag.value = re
            .replace_all(
                &injected_flag.value,
                uuid::Uuid::new_v4().simple().to_string(),
            )
            .to_string();
    }

    let nats = crate::container::get_container()
        .await
        .create(ctn_name.clone(), challenge.clone(), injected_flag.clone())
        .await?;

    let mut pod = crate::model::pod::ActiveModel {
        name: Set(ctn_name),
        user_id: Set(req.user_id.clone().unwrap()),
        team_id: Set(req.team_id.clone()),
        game_id: Set(req.game_id.clone()),
        challenge_id: Set(req.challenge_id.clone()),
        flag: Set(Some(injected_flag.value)),
        removed_at: Set(chrono::Utc::now().timestamp() + challenge.duration),
        nats: Set(nats),
        ..Default::default()
    }
    .insert(&get_db())
    .await?;

    pod.flag = None;
    pod.simplify();

    return Ok(pod);
}

pub async fn update(id: i64) -> Result<(), Box<dyn Error>> {
    let (pods, total) =
        crate::model::pod::find(Some(id), None, None, None, None, None, None).await?;
    if total == 0 {
        return Err("No pod found".into());
    }
    let pod = pods.get(0).unwrap();
    let (challenges, _) = crate::model::challenge::find(
        Some(pod.challenge_id.clone()),
        None,
        None,
        None,
        None,
        None,
        None,
    )
    .await?;
    let challenge = challenges.get(0).unwrap();

    let mut pod = pod.clone().into_active_model();
    pod.removed_at = Set(chrono::Utc::now().timestamp() + challenge.duration);
    let _ = pod.update(&get_db()).await;
    return Ok(());
}

pub async fn delete(id: i64) -> Result<(), Box<dyn Error>> {
    let (pods, total) =
        crate::model::pod::find(Some(id), None, None, None, None, None, None).await?;
    if total == 0 {
        return Err("No pod found".into());
    }
    let pod = pods.get(0).unwrap();
    crate::container::get_container()
        .await
        .delete(pod.name.clone())
        .await;

    let mut pod = pod.clone().into_active_model();
    pod.removed_at = Set(chrono::Utc::now().timestamp());

    let _ = pod.update(&get_db()).await?;
    return Ok(());
}
