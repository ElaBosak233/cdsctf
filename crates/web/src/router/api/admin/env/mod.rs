mod env_id;

use std::collections::BTreeMap;

use axum::{Router, response::IntoResponse};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    extract::Query,
    traits::{WebError, WebResponse},
    util::cluster::Env,
};

pub fn router() -> Router {
    Router::new()
        .route("/", axum::routing::get(get_env))
        .nest("/{env_id}", env_id::router())
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetEnvRequest {
    pub id: Option<String>,
    pub user_id: Option<i64>,
    pub team_id: Option<i64>,
    pub game_id: Option<i64>,
    pub challenge_id: Option<Uuid>,
}

pub async fn get_env(
    Query(params): Query<GetEnvRequest>,
) -> Result<WebResponse<Vec<Env>>, WebError> {
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
        .map(|pod| Env::from(pod))
        .collect::<Vec<Env>>();

    Ok(WebResponse {
        data: Some(pods),
        ..Default::default()
    })
}
