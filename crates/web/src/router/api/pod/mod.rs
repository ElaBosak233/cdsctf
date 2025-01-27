pub mod daemon;

use axum::{Router, http::StatusCode};
use cds_db::{entity::user::Group, get_db, transfer::Pod};
use regex::Regex;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, EntityTrait, IntoActiveModel, PaginatorTrait,
    QueryFilter, Set,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

use crate::{
    extract::{Extension, Json, Path, Query},
    traits::{Ext, WebError, WebResponse},
};

pub async fn router() -> Router {
    daemon::init().await;

    Router::new()
        .route("/", axum::routing::get(get))
        .route("/", axum::routing::post(create))
        .route("/{id}/renew", axum::routing::post(renew))
        .route("/{id}/stop", axum::routing::post(stop))
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetRequest {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub user_id: Option<i64>,
    pub team_id: Option<i64>,
    pub game_id: Option<i64>,
    pub challenge_id: Option<Uuid>,
    pub is_available: Option<bool>,
    pub is_detailed: Option<bool>,
    pub page: Option<u64>,
    pub size: Option<u64>,
}

pub async fn get(
    Extension(ext): Extension<Ext>, Query(params): Query<GetRequest>,
) -> Result<WebResponse<Vec<Pod>>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;

    let (mut pods, total) = cds_db::transfer::pod::find(
        params.id,
        params.name,
        params.user_id,
        params.team_id,
        params.game_id,
        params.challenge_id,
        params.is_available,
    )
    .await?;

    match params.is_detailed {
        Some(true) => {
            if operator.group != Group::Admin {
                return Err(WebError::Forbidden(json!("")));
            }
        }
        _ => {
            for pod in pods.iter_mut() {
                pod.flag = None;
                pod.user = None;
                pod.challenge = None;
            }
        }
    }

    Ok(WebResponse {
        code: StatusCode::OK.as_u16(),
        data: Some(pods),
        total: Some(total),
        ..WebResponse::default()
    })
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateRequest {
    pub challenge_id: Uuid,
    pub team_id: Option<i64>,
    pub user_id: Option<i64>,
    pub game_id: Option<i64>,
}

pub async fn create(
    Extension(ext): Extension<Ext>, Json(mut body): Json<CreateRequest>,
) -> Result<WebResponse<Pod>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;

    let challenge = cds_db::entity::challenge::Entity::find_by_id(body.challenge_id)
        .one(get_db())
        .await?
        .ok_or(WebError::BadRequest(json!("challenge_not_found")))?;

    let ctn_name = format!("cds-{}", Uuid::new_v4().simple());

    if challenge.flags.clone().into_iter().next().is_none() {
        return Err(WebError::BadRequest(json!("no_flag")));
    }

    let mut injected_flag = challenge.flags.clone().into_iter().next().unwrap();

    let re = Regex::new(r"\[([Uu][Uu][Ii][Dd])]").unwrap();
    if injected_flag.type_ == cds_db::entity::challenge::FlagType::Dynamic {
        injected_flag.value = re
            .replace_all(&injected_flag.value, Uuid::new_v4().simple().to_string())
            .to_string();
    }

    let env = challenge
        .env
        .ok_or(WebError::BadRequest(json!("challenge_env_invalid")))?;

    if body.game_id.is_some() != body.team_id.is_some() {
        return Err(WebError::BadRequest(json!("invalid")));
    }

    if let (Some(game_id), Some(team_id)) = (body.game_id, body.team_id) {
        let _ = cds_db::entity::game_team::Entity::find()
            .filter(
                Condition::all()
                    .add(cds_db::entity::game_team::Column::GameId.eq(game_id))
                    .add(cds_db::entity::game_team::Column::TeamId.eq(team_id)),
            )
            .one(get_db())
            .await?
            .ok_or(WebError::BadRequest(json!("game_team_not_found")))?;

        let _ = cds_db::entity::game_challenge::Entity::find()
            .filter(
                Condition::all()
                    .add(cds_db::entity::game_challenge::Column::GameId.eq(game_id))
                    .add(cds_db::entity::game_challenge::Column::ChallengeId.eq(challenge.id)),
            )
            .one(get_db())
            .await?
            .ok_or(WebError::BadRequest(json!("game_challenge_not_found")))?;

        let member_count = cds_db::entity::user_team::Entity::find()
            .filter(Condition::all().add(cds_db::entity::user_team::Column::TeamId.eq(team_id)))
            .count(get_db())
            .await?;

        let existing_pod_count = cds_db::entity::pod::Entity::find()
            .filter(
                Condition::all()
                    .add(cds_db::entity::pod::Column::TeamId.eq(team_id))
                    .add(cds_db::entity::pod::Column::GameId.eq(game_id)),
            )
            .count(get_db())
            .await?;

        if member_count == existing_pod_count {
            return Err(WebError::TooManyRequests(json!("too_many_team_pods")));
        }
    } else {
        let existing_pod_count = cds_db::entity::pod::Entity::find()
            .filter(Condition::all().add(cds_db::entity::pod::Column::UserId.eq(operator.id)))
            .count(get_db())
            .await?;

        if existing_pod_count == 1 {
            return Err(WebError::TooManyRequests(json!("too_many_user_pods")));
        }
    }

    let nats = cds_cluster::create(ctn_name.clone(), env.clone(), injected_flag.clone()).await?;

    let pod = cds_db::entity::pod::ActiveModel {
        name: Set(ctn_name),
        user_id: Set(operator.id),
        team_id: Set(body.team_id),
        game_id: Set(body.game_id),
        challenge_id: Set(body.challenge_id),
        flag: Set(Some(injected_flag.value)),
        removed_at: Set(chrono::Utc::now().timestamp() + env.duration),
        nats: Set(nats),
        ..Default::default()
    }
    .insert(get_db())
    .await?;
    let mut pod = cds_db::transfer::Pod::from(pod);

    pod.desensitize();

    Ok(WebResponse {
        code: StatusCode::OK.as_u16(),
        data: Some(pod),
        ..WebResponse::default()
    })
}

macro_rules! check_permission {
    ($operator:expr, $pod:expr) => {
        if !($operator.group == Group::Admin
            || $operator.id == $pod.user_id
            || $operator
                .teams
                .iter()
                .any(|team| Some(team.id) == $pod.team_id))
        {
            return Err(WebError::Forbidden(json!("")));
        }
    };
}

pub async fn renew(
    Extension(ext): Extension<Ext>, Path(id): Path<i64>,
) -> Result<WebResponse<Pod>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;

    let pod = cds_db::entity::pod::Entity::find_by_id(id)
        .one(get_db())
        .await?
        .ok_or(WebError::NotFound(json!("pod_not_found")))?;

    check_permission!(operator, pod);

    let challenge = cds_db::entity::challenge::Entity::find_by_id(pod.challenge_id)
        .one(get_db())
        .await?;
    let challenge = challenge.unwrap();

    let mut pod = pod.clone().into_active_model();
    pod.removed_at = Set(chrono::Utc::now().timestamp() + challenge.env.unwrap().duration);
    let pod = pod.clone().update(get_db()).await?;

    let mut pod = cds_db::transfer::Pod::from(pod);
    pod.desensitize();

    Ok(WebResponse {
        code: StatusCode::OK.as_u16(),
        data: Some(pod),
        ..WebResponse::default()
    })
}

pub async fn stop(
    Extension(ext): Extension<Ext>, Path(id): Path<i64>,
) -> Result<WebResponse<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;

    let pod = cds_db::entity::pod::Entity::find_by_id(id)
        .one(get_db())
        .await?
        .ok_or(WebError::NotFound(json!("pod_not_found")))?;

    check_permission!(operator, pod);

    let pod_name = pod.name.clone();

    tokio::spawn(async move {
        cds_cluster::delete(pod_name).await;
    });

    let mut pod = pod.clone().into_active_model();
    pod.removed_at = Set(chrono::Utc::now().timestamp());

    let _ = pod.update(get_db()).await?;

    Ok(WebResponse {
        code: StatusCode::OK.as_u16(),
        ..WebResponse::default()
    })
}
