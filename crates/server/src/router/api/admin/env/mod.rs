mod env_id;

use std::{collections::BTreeMap, sync::Arc};

use axum::{Router, extract::State};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    extract::{Extension, Json, Query},
    traits::{AppState, AuthPrincipal, WebError, WebResponse},
    util::cluster::DynamicEnvironment,
};

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", axum::routing::get(get_env))
        .route("/", axum::routing::post(create_env))
        .nest("/{env_id}", env_id::router())
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetEnvRequest {
    pub id: Option<String>,
    pub user_id: Option<i64>,
    pub team_id: Option<i64>,
    pub game_id: Option<i64>,
    pub challenge_id: Option<i64>,
}

pub async fn get_env(
    State(s): State<Arc<AppState>>,

    Query(params): Query<GetEnvRequest>,
) -> Result<WebResponse<Vec<DynamicEnvironment>>, WebError> {
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

    let pods = s.cluster.get_pods_by_label(&labels).await?;

    let envs = pods
        .into_iter()
        .map(|pod| DynamicEnvironment::from(pod).with_env(&s.env))
        .collect::<Vec<DynamicEnvironment>>();

    Ok(WebResponse {
        data: Some(envs),
        ..Default::default()
    })
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateEnvRequest {
    pub challenge_id: i64,
}

pub async fn create_env(
    State(s): State<Arc<AppState>>,

    Extension(ext): Extension<AuthPrincipal>,
    Json(body): Json<CreateEnvRequest>,
) -> Result<WebResponse<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;

    let challenge = crate::util::loader::prepare_challenge(&s.db.conn, body.challenge_id).await?;

    let _ = challenge
        .clone()
        .env
        .ok_or(WebError::BadRequest(json!("challenge_env_invalid")))?;

    let existing_pods = s
        .cluster
        .get_pods_by_label(
            &BTreeMap::from([("cds/user_id", format!("{}", operator.id))])
                .iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect::<Vec<String>>()
                .join(","),
        )
        .await?;

    if existing_pods.len() >= 1 {
        return Err(WebError::TooManyRequests(json!("too_many_user_pods")));
    }

    s.cluster
        .create_challenge_env(operator, None, None, challenge)
        .await?;

    Ok(WebResponse::default())
}
