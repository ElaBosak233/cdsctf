mod env_id;

use std::collections::BTreeMap;

use axum::{Router, http::StatusCode, response::IntoResponse};
use cds_db::get_db;
use sea_orm::{ActiveModelTrait, ColumnTrait, Condition, EntityTrait, PaginatorTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

use crate::{
    extract::{Extension, Json, Query},
    traits::{Ext, WebError, WebResponse},
};

pub async fn router() -> Router {
    Router::new()
        .route("/", axum::routing::get(get_env))
        .route("/", axum::routing::post(create_env))
        .nest("/{env_id}", env_id::router())
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Pod {
    pub id: String,
    pub user_id: i64,
    pub game_team_id: i64,
    pub game_id: i64,
    pub challenge_id: String,

    pub public_entry: String,
    pub ports: Vec<i32>,
    pub nats: String,

    pub status: String,
    pub reason: String,

    pub renew: i64,
    pub duration: i64,
    pub started_at: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetPodRequest {
    pub id: Option<String>,
    pub user_id: Option<i64>,
    pub team_id: Option<i64>,
    pub game_id: Option<i64>,
    pub challenge_id: Option<Uuid>,
}

pub async fn get_env(
    Extension(ext): Extension<Ext>, Query(params): Query<GetPodRequest>,
) -> Result<WebResponse<Vec<Pod>>, WebError> {
    let _ = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;

    let mut map: BTreeMap<String, String> = BTreeMap::new();

    if let Some(id) = params.id {
        map.insert("cds/env_id".to_owned(), id);
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
                .get("cds/env_id")
                .map(|s| s.to_owned())
                .unwrap_or_default()
                .to_owned();
            let user_id = labels
                .get("cds/user_id")
                .map(|s| s.to_owned())
                .unwrap_or_default()
                .to_owned()
                .parse::<i64>()
                .unwrap_or(0);
            let team_id = labels
                .get("cds/team_id")
                .map(|s| s.to_owned())
                .unwrap_or_default()
                .to_owned()
                .parse::<i64>()
                .unwrap_or(0);
            let game_id = labels
                .get("cds/game_id")
                .map(|s| s.to_owned())
                .unwrap_or_default()
                .to_owned()
                .parse::<i64>()
                .unwrap_or(0);
            let challenge_id = labels
                .get("cds/challenge_id")
                .map(|s| s.to_owned())
                .unwrap_or_default()
                .to_owned();

            let annotations = pod.metadata.annotations.unwrap_or_default();

            let ports = serde_json::from_str::<Vec<i32>>(
                &annotations
                    .get("cds/ports")
                    .map(|s| s.to_owned())
                    .unwrap_or_default(),
            )
            .unwrap_or_default();
            let nats = annotations
                .get("cds/nats")
                .map(|s| s.to_owned())
                .unwrap_or_default()
                .to_owned();
            let duration = annotations
                .get("cds/duration")
                .map(|s| s.to_owned())
                .unwrap_or_default()
                .to_owned()
                .parse::<i64>()
                .unwrap_or(0);
            let renew = annotations
                .get("cds/renew")
                .map(|s| s.to_owned())
                .unwrap_or_default()
                .to_owned()
                .parse::<i64>()
                .unwrap_or(0);

            let mut status = "".to_owned();
            let mut reason = "".to_owned();

            let _ = pod
                .status
                .unwrap_or_default()
                .container_statuses
                .unwrap_or_default()
                .iter()
                .for_each(|s| {
                    let container_state = s.to_owned().state.unwrap_or_default();
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

            let started_at = pod.metadata.creation_timestamp.unwrap().0.timestamp();

            let node_name = pod.spec.unwrap_or_default().node_name.unwrap_or_default();

            let public_entry = cds_config::get_constant()
                .cluster
                .public_entries
                .get(&node_name)
                .cloned()
                .unwrap_or("unknown".to_owned());

            Pod {
                id,
                user_id,
                game_team_id: team_id,
                game_id,
                challenge_id,
                public_entry,
                ports,
                nats,
                status,
                reason,
                renew,
                duration,
                started_at,
            }
        })
        .collect::<Vec<Pod>>();

    Ok(WebResponse {
        code: StatusCode::OK,
        data: Some(pods),
        ..Default::default()
    })
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreatePodRequest {
    pub challenge_id: Uuid,
    pub game_team_id: Option<i64>,
    pub user_id: Option<i64>,
    pub game_id: Option<i64>,
}

pub async fn create_env(
    Extension(ext): Extension<Ext>, Json(mut body): Json<CreatePodRequest>,
) -> Result<WebResponse<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;

    let challenge = cds_db::entity::challenge::Entity::find_by_id(body.challenge_id)
        .one(get_db())
        .await?
        .map(|challenge| cds_db::transfer::Challenge::from(challenge))
        .ok_or(WebError::BadRequest(json!("challenge_not_found")))?;

    let _ = challenge
        .clone()
        .env
        .ok_or(WebError::BadRequest(json!("challenge_env_invalid")))?;

    if body.game_id.is_some() != body.game_team_id.is_some() {
        return Err(WebError::BadRequest(json!("invalid")));
    }

    if let (Some(game_id), Some(team_id)) = (body.game_id, body.game_team_id) {
        let _ = cds_db::entity::team::Entity::find()
            .filter(
                Condition::all()
                    .add(cds_db::entity::team::Column::GameId.eq(game_id))
                    .add(cds_db::entity::team::Column::Id.eq(team_id)),
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

        let member_count = cds_db::entity::team_user::Entity::find()
            .filter(Condition::all().add(cds_db::entity::team_user::Column::TeamId.eq(team_id)))
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

    let team = match body.game_team_id {
        Some(game_team_id) => cds_db::entity::team::Entity::find_by_id(game_team_id)
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

    let _ = cds_cluster::create_challenge_env(operator, team, game, challenge).await?;

    Ok(WebResponse {
        code: StatusCode::OK,
        ..Default::default()
    })
}
