use std::collections::BTreeMap;

use axum::{Router, http::StatusCode};
use cds_db::{entity::user::Group, get_db};
use sea_orm::{ActiveModelTrait, ColumnTrait, Condition, EntityTrait, PaginatorTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::info;
use uuid::Uuid;

use crate::{
    extract::{Extension, Json, Path, Query},
    traits::{Ext, WebError, WebResponse},
};

pub async fn router() -> Router {
    Router::new()
        .route("/", axum::routing::get(get))
        .route("/", axum::routing::post(create))
        .route("/{id}/renew", axum::routing::post(renew))
        .route("/{id}/stop", axum::routing::post(stop))
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetRequest {
    pub id: Option<String>,
    pub user_id: Option<i64>,
    pub team_id: Option<i64>,
    pub game_id: Option<i64>,
    pub challenge_id: Option<Uuid>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Pod {
    pub id: String,
    pub user_id: String,
    pub team_id: String,
    pub game_id: String,
    pub challenge_id: String,

    pub ports: String,
    pub nats: String,

    pub status: String,
    pub reason: String,
}

pub async fn get(
    Extension(ext): Extension<Ext>, Query(params): Query<GetRequest>,
) -> Result<WebResponse<Vec<Pod>>, WebError> {
    let _ = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;

    let mut map: BTreeMap<String, String> = BTreeMap::new();

    if let Some(id) = params.id {
        map.insert("cds/resource_id".to_owned(), id);
    }

    if let Some(user_id) = params.user_id {
        map.insert("cds/user_id".to_owned(), format!("{}", user_id));
    }

    if let Some(team_id) = params.team_id {
        map.insert("cds/team_id".to_owned(), format!("{}", team_id));
    }

    if let Some(game_id) = params.game_id {
        map.insert("cds/game_id".to_owned(), format!("{}", game_id));
    }

    if let Some(challenge_id) = params.challenge_id {
        map.insert("cds/challenge_id".to_owned(), format!("{}", challenge_id));
    }

    let labels = map
        .iter()
        .map(|(k, v)| format!("{}={}", k, v))
        .collect::<Vec<String>>()
        .join(",");

    let pods = cds_cluster::get_pods_by_label(&labels).await?;

    let pods = pods
        .into_iter()
        .map(|pod| {
            let labels = pod.metadata.labels.unwrap_or_default();

            let id = labels
                .get("cds/resource_id")
                .map(|s| s.to_owned())
                .unwrap_or_default()
                .to_owned();
            let user_id = labels
                .get("cds/user_id")
                .map(|s| s.to_owned())
                .unwrap_or_default()
                .to_owned();
            let team_id = labels
                .get("cds/team_id")
                .map(|s| s.to_owned())
                .unwrap_or_default()
                .to_owned();
            let game_id = labels
                .get("cds/game_id")
                .map(|s| s.to_owned())
                .unwrap_or_default()
                .to_owned();
            let challenge_id = labels
                .get("cds/challenge_id")
                .map(|s| s.to_owned())
                .unwrap_or_default()
                .to_owned();

            let annotations = pod.metadata.annotations.unwrap_or_default();

            let ports = annotations
                .get("cds/ports")
                .map(|s| s.to_owned())
                .unwrap_or_default()
                .to_owned();
            let nats = annotations
                .get("cds/nats")
                .map(|s| s.to_owned())
                .unwrap_or_default()
                .to_owned();

            let mut status = "".to_owned();
            let mut reason = "".to_owned();

            let _ = pod
                .status
                .unwrap()
                .container_statuses
                .unwrap()
                .iter()
                .for_each(|s| {
                    let container_state = s.to_owned().state.unwrap();
                    if let Some(waiting) = container_state.waiting {
                        status = "waiting".to_owned();
                        if let Some(r) = waiting.reason {
                            reason = r.clone();
                        }
                    }
                    if let Some(_) = container_state.running {
                        status = "running".to_owned();
                    }
                    if let Some(terminated) = container_state.terminated {
                        status = "terminated".to_owned();
                        if let Some(r) = terminated.reason {
                            reason = r.clone();
                        }
                    }
                });

            Pod {
                id,
                user_id,
                team_id,
                game_id,
                challenge_id,
                ports,
                nats,
                status,
                reason,
            }
        })
        .collect::<Vec<Pod>>();

    Ok(WebResponse {
        code: StatusCode::OK.as_u16(),
        data: Some(pods),
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
) -> Result<WebResponse<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;

    let challenge = cds_db::entity::challenge::Entity::find_by_id(body.challenge_id)
        .one(get_db())
        .await?
        .map(|challenge| cds_db::transfer::Challenge::from(challenge))
        .ok_or(WebError::BadRequest(json!("challenge_not_found")))?;

    if challenge.clone().flags.into_iter().next().is_none() {
        return Err(WebError::BadRequest(json!("no_flag")));
    }

    let _ = challenge
        .clone()
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
                    .add(
                        cds_db::entity::game_challenge::Column::ChallengeId
                            .eq(challenge.clone().id),
                    ),
            )
            .one(get_db())
            .await?
            .ok_or(WebError::BadRequest(json!("game_challenge_not_found")))?;

        let member_count = cds_db::entity::user_team::Entity::find()
            .filter(Condition::all().add(cds_db::entity::user_team::Column::TeamId.eq(team_id)))
            .count(get_db())
            .await?;

        let existing_pods = cds_cluster::get_pods_by_label(
            &BTreeMap::from([
                ("cds/game_id", format!("{}", game_id)),
                ("cds/team_id", format!("{}", team_id)),
            ])
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<String>>()
            .join(","),
        )
        .await?;

        if member_count == existing_pods.len() as u64 {
            return Err(WebError::TooManyRequests(json!("too_many_team_pods")));
        }
    } else {
        let existing_pods = cds_cluster::get_pods_by_label(
            &BTreeMap::from([("cds/user_id", format!("{}", operator.id))])
                .iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect::<Vec<String>>()
                .join(","),
        )
        .await?;

        if existing_pods.len() == 1 {
            return Err(WebError::TooManyRequests(json!("too_many_user_pods")));
        }
    }

    let team = match body.team_id {
        Some(team_id) => cds_db::entity::team::Entity::find_by_id(team_id)
            .one(get_db())
            .await?
            .map(|team| cds_db::transfer::Team::from(team)),
        _ => None,
    };

    let game = match body.game_id {
        Some(game_id) => cds_db::entity::game::Entity::find_by_id(game_id)
            .one(get_db())
            .await?
            .map(|game| cds_db::transfer::Game::from(game)),
        _ => None,
    };

    let _ = cds_cluster::create_challenge_env(operator, team, game, challenge).await;

    Ok(WebResponse {
        code: StatusCode::OK.as_u16(),
        ..WebResponse::default()
    })
}

pub async fn renew(
    Extension(ext): Extension<Ext>, Path(id): Path<Uuid>,
) -> Result<WebResponse<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;

    let pod = cds_cluster::get_pod(&id.to_string()).await?;

    let labels = pod.metadata.labels.unwrap_or_default();
    let id = labels
        .get("cds/resource_id")
        .map(|s| s.to_string())
        .unwrap_or_default();
    let team_id = labels
        .get("cds/team_id")
        .map(|s| s.to_string())
        .unwrap_or_default();
    let user_id = labels
        .get("cds/user_id")
        .map(|s| s.to_string())
        .unwrap_or_default();

    if !(operator.group == Group::Admin
        || operator.id.to_string() == user_id
        || operator
            .teams
            .iter()
            .any(|team| team.id.to_string() == team_id))
    {
        return Err(WebError::Forbidden(json!("")));
    }

    let started_at = pod.metadata.creation_timestamp.unwrap().0.timestamp();

    let annotations = pod.metadata.annotations.unwrap_or_default();

    let renew = annotations
        .get("cds/renew")
        .map(|s| s.to_owned())
        .unwrap_or_default()
        .parse::<i64>()
        .unwrap();
    let duration = annotations
        .get("cds/duration")
        .map(|s| s.to_owned())
        .unwrap_or_default()
        .parse::<i64>()
        .unwrap();

    let now = chrono::Utc::now().timestamp();

    if now - started_at + (renew + 1) * duration > chrono::Duration::minutes(10).num_seconds() {
        return Err(WebError::BadRequest(json!("renewal_within_10_minutes")));
    }

    cds_cluster::renew_challenge_env(&id).await?;

    Ok(WebResponse {
        code: StatusCode::OK.as_u16(),
        ..WebResponse::default()
    })
}

pub async fn stop(
    Extension(ext): Extension<Ext>, Path(id): Path<Uuid>,
) -> Result<WebResponse<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;

    let pod = cds_cluster::get_pod(&id.to_string()).await?;

    let labels = pod.metadata.labels.unwrap_or_default();
    let id = labels
        .get("cds/resource_id")
        .map(|s| s.to_string())
        .unwrap_or_default();
    let team_id = labels
        .get("cds/team_id")
        .map(|s| s.to_string())
        .unwrap_or_default();
    let user_id = labels
        .get("cds/user_id")
        .map(|s| s.to_string())
        .unwrap_or_default();

    if !(operator.group == Group::Admin
        || operator.id.to_string() == user_id
        || operator
            .teams
            .iter()
            .any(|team| team.id.to_string() == team_id))
    {
        return Err(WebError::Forbidden(json!("")));
    }

    cds_cluster::delete_challenge_env(&id).await?;

    Ok(WebResponse {
        code: StatusCode::OK.as_u16(),
        ..WebResponse::default()
    })
}
