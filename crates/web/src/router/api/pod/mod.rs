pub mod daemon;

use axum::{
    Router,
    extract::{Path, Query},
    http::StatusCode,
};
use cds_db::{entity::user::Group, get_db};
use regex::Regex;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

use crate::{
    extract::{Extension, Json},
    traits::{Ext, WebError, WebResponse},
};

pub async fn router() -> Router {
    daemon::init().await;

    Router::new()
        .route("/", axum::routing::get(get))
        .route("/", axum::routing::post(create))
        .route("/:id/renew", axum::routing::post(renew))
        .route("/:id/stop", axum::routing::post(stop))
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetRequest {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub user_id: Option<i64>,
    pub team_id: Option<i64>,
    pub game_id: Option<i64>,
    pub challenge_id: Option<i64>,
    pub is_available: Option<bool>,
    pub is_detailed: Option<bool>,
    pub page: Option<u64>,
    pub size: Option<u64>,
}

pub async fn get(
    Extension(ext): Extension<Ext>, Query(params): Query<GetRequest>,
) -> Result<WebResponse<Vec<cds_db::transfer::Pod>>, WebError> {
    let _ = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;

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

    if let Some(is_detailed) = params.is_detailed {
        if !is_detailed {
            for pod in pods.iter_mut() {
                pod.flag = None;
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
    pub challenge_id: i64,
    pub team_id: Option<i64>,
    pub user_id: Option<i64>,
    pub game_id: Option<i64>,
}

pub async fn create(
    Extension(ext): Extension<Ext>, Json(mut body): Json<CreateRequest>,
) -> Result<WebResponse<cds_db::transfer::Pod>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    body.user_id = Some(operator.id);

    let challenge = cds_db::entity::challenge::Entity::find_by_id(body.challenge_id)
        .one(get_db())
        .await?
        .map(cds_db::transfer::Challenge::from);

    let challenge = challenge.ok_or(WebError::BadRequest(json!("challenge_not_found")))?;

    let ctn_name = format!("cds-{}", Uuid::new_v4().simple());

    if challenge.flags.clone().into_iter().next().is_none() {
        return Err(WebError::BadRequest(json!("no_flag")));
    }

    let mut injected_flag = challenge.flags.clone().into_iter().next().unwrap();

    let re = Regex::new(r"\[([Uu][Uu][Ii][Dd])\]").unwrap();
    if injected_flag.type_ == cds_db::entity::challenge::FlagType::Dynamic {
        injected_flag.value = re
            .replace_all(&injected_flag.value, Uuid::new_v4().simple().to_string())
            .to_string();
    }

    let nats = cds_cluster::create(
        ctn_name.clone(),
        cds_db::entity::challenge::Model::from(challenge.clone()),
        injected_flag.clone(),
    )
    .await?;

    let pod = cds_db::entity::pod::ActiveModel {
        name: Set(ctn_name),
        user_id: Set(body.user_id.unwrap()),
        team_id: Set(body.team_id),
        game_id: Set(body.game_id),
        challenge_id: Set(body.challenge_id),
        flag: Set(Some(injected_flag.value)),
        removed_at: Set(chrono::Utc::now().timestamp() + challenge.duration),
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
) -> Result<WebResponse<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;

    let pod = cds_db::entity::pod::Entity::find()
        .filter(cds_db::entity::pod::Column::Id.eq(id))
        .one(get_db())
        .await?
        .ok_or_else(|| WebError::NotFound(json!("")))?;

    check_permission!(operator, pod);

    let challenge = cds_db::entity::challenge::Entity::find_by_id(pod.challenge_id)
        .one(get_db())
        .await?;
    let challenge = challenge.unwrap();

    let mut pod = pod.clone().into_active_model();
    pod.removed_at = Set(chrono::Utc::now().timestamp() + challenge.duration);
    let _ = pod.update(get_db()).await;

    Ok(WebResponse {
        code: StatusCode::OK.as_u16(),
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
        .ok_or_else(|| WebError::NotFound(json!("")))?;

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
