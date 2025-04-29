mod env_id;

use std::collections::BTreeMap;

use axum::{Router, http::StatusCode};
use cds_db::{
    get_db,
    sea_orm::{ActiveModelTrait, ColumnTrait, Condition, EntityTrait, PaginatorTrait, QueryFilter},
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

use crate::{
    extract::{Extension, Json, Query},
    traits::{Ext, WebError, WebResponse},
    util::cluster::Env,
};

pub fn router() -> Router {
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
    pub challenge_id: Option<Uuid>,
}

pub async fn get_env(
    Extension(ext): Extension<Ext>,
    Query(params): Query<GetEnvRequest>,
) -> Result<WebResponse<Vec<Env>>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    let mut map: BTreeMap<String, String> = BTreeMap::new();

    match (params.user_id, params.team_id, params.game_id) {
        (Some(user_id), None, None) => {
            if operator.id != user_id {
                return Err(WebError::Forbidden(json!("")));
            }
            map.insert("cds/user_id".to_owned(), format!("{}", user_id));
        }
        (_, Some(team_id), Some(game_id)) => {
            let team = crate::util::loader::prepare_self_team(game_id, operator.id).await?;
            if team.id != team_id {
                return Err(WebError::Forbidden(json!("")));
            }
            map.insert("cds/team_id".to_owned(), format!("{}", team_id));
            map.insert("cds/game_id".to_owned(), format!("{}", game_id));
        }
        _ => {
            return Err(WebError::BadRequest(json!("")));
        }
    }

    if let Some(id) = params.id {
        map.insert("cds/env_id".to_owned(), id);
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

    let envs = pods
        .into_iter()
        .map(crate::util::cluster::Env::from)
        .collect::<Vec<Env>>();

    Ok(WebResponse {
        code: StatusCode::OK,
        data: Some(envs),
        ..Default::default()
    })
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateEnvRequest {
    pub challenge_id: Uuid,
    pub team_id: Option<i64>,
    pub user_id: Option<i64>,
    pub game_id: Option<i64>,
}

pub async fn create_env(
    Extension(ext): Extension<Ext>,
    Json(body): Json<CreateEnvRequest>,
) -> Result<WebResponse<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;

    let challenge = crate::util::loader::prepare_challenge(body.challenge_id).await?;

    let _ = challenge
        .clone()
        .env
        .ok_or(WebError::BadRequest(json!("challenge_env_invalid")))?;

    if body.game_id.is_some() != body.team_id.is_some() {
        return Err(WebError::BadRequest(json!("invalid")));
    }

    if let (Some(game_id), Some(team_id)) = (body.game_id, body.team_id) {
        let _ = crate::util::loader::prepare_team(game_id, team_id).await?;
        let _ = crate::util::loader::prepare_game_challenge(game_id, challenge.id).await?;

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

    let (team, game) = match (body.team_id, body.game_id) {
        (Some(team_id), Some(game_id)) => (
            cds_db::entity::team::Entity::find()
                .filter(cds_db::entity::team::Column::GameId.eq(game_id))
                .filter(cds_db::entity::team::Column::Id.eq(team_id))
                .one(get_db())
                .await?,
            cds_db::entity::game::Entity::find()
                .filter(cds_db::entity::game::Column::Id.eq(game_id))
                .one(get_db())
                .await?,
        ),
        _ => (None, None),
    };

    cds_cluster::create_challenge_env(operator, team, game, challenge).await?;

    Ok(WebResponse {
        code: StatusCode::OK,
        ..Default::default()
    })
}
