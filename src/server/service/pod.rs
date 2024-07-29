use std::error::Error;

use regex::Regex;
use sea_orm::{IntoActiveModel, Set};
use uuid::Uuid;

use crate::container::traits::Container;

pub async fn find(
    req: crate::model::pod::request::FindRequest,
) -> Result<(Vec<crate::model::pod::Model>, u64), ()> {
    let (mut pods, total) = crate::repository::pod::find(
        req.id,
        req.name,
        req.user_id,
        req.team_id,
        req.game_id,
        req.challenge_id,
        req.is_available,
    )
    .await
    .unwrap();

    if let Some(is_detailed) = req.is_detailed {
        if !is_detailed {
            for pod in pods.iter_mut() {
                pod.flag = None;
            }
        }
    }
    return Ok((pods, total));
}

pub async fn create(
    req: crate::model::pod::request::CreateRequest,
) -> Result<crate::model::pod::Model, Box<dyn Error>> {
    let (challenges, _) = crate::repository::challenge::find(
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

    let re = Regex::new(r"\[([Uu][Ii][Dd])\]").unwrap();
    if injected_flag.type_.to_ascii_lowercase() == "dynamic" {
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

    let mut pod = crate::repository::pod::create(crate::model::pod::ActiveModel {
        name: Set(ctn_name),
        user_id: Set(req.user_id.clone().unwrap()),
        team_id: Set(req.team_id.clone()),
        game_id: Set(req.game_id.clone()),
        challenge_id: Set(req.challenge_id.clone()),
        flag: Set(Some(injected_flag.value)),
        removed_at: Set(chrono::Utc::now().timestamp() + challenge.duration),
        nats: Set(nats),
        ..Default::default()
    })
    .await?;

    pod.flag = None;

    return Ok(pod);
}

pub async fn update(id: i64) -> Result<(), Box<dyn Error>> {
    let (pods, total) =
        crate::repository::pod::find(Some(id), None, None, None, None, None, None).await?;
    if total == 0 {
        return Err("No pod found".into());
    }
    let pod = pods.get(0).unwrap();
    let (challenges, _) = crate::repository::challenge::find(
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
    let _ = crate::repository::pod::update(pod).await;
    return Ok(());
}

pub async fn delete(id: i64) -> Result<(), Box<dyn Error>> {
    let (pods, total) =
        crate::repository::pod::find(Some(id), None, None, None, None, None, None).await?;
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

    let _ = crate::repository::pod::update(pod).await;
    return Ok(());
}