use axum::{Router, http::StatusCode};
use cds_db::{
    entity::{submission::Status, team::State, user::Group},
    get_db,
    transfer::Submission,
};
use sea_orm::{
    ActiveModelTrait, ActiveValue::NotSet, ColumnTrait, Condition, EntityTrait, PaginatorTrait,
    QueryFilter, QuerySelect, Set,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    extract::{Extension, Json, Path, Query},
    traits::{Ext, WebError, WebResponse},
};

pub fn router() -> Router {
    Router::new()
        .route("/", axum::routing::get(get_submission))
        .route("/{submission_id}", axum::routing::delete(delete_submission))
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetSubmissionRequest {
    pub id: Option<i64>,
    pub user_id: Option<i64>,
    pub team_id: Option<i64>,
    pub game_id: Option<i64>,
    pub challenge_id: Option<i64>,
    pub status: Option<Status>,
    pub page: Option<u64>,
    pub size: Option<u64>,
}

pub async fn get_submission(
    Extension(ext): Extension<Ext>, Query(params): Query<GetSubmissionRequest>,
) -> Result<WebResponse<Vec<Submission>>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;

    if operator.group != Group::Admin {
        return Err(WebError::Forbidden(json!("")));
    }

    let mut sql = cds_db::entity::submission::Entity::find();

    if let Some(id) = params.id {
        sql = sql.filter(cds_db::entity::submission::Column::Id.eq(id));
    }

    if let Some(user_id) = params.user_id {
        sql = sql.filter(cds_db::entity::submission::Column::UserId.eq(user_id));
    }

    if let Some(team_id) = params.team_id {
        sql = sql.filter(cds_db::entity::submission::Column::TeamId.eq(team_id));
    }

    if let Some(game_id) = params.game_id {
        sql = sql.filter(cds_db::entity::submission::Column::GameId.eq(game_id));
    }

    if let Some(challenge_id) = params.challenge_id {
        sql = sql.filter(cds_db::entity::submission::Column::ChallengeId.eq(challenge_id));
    }

    if let Some(status) = params.status {
        sql = sql.filter(cds_db::entity::submission::Column::Status.eq(status));
    }

    let total = sql.clone().count(get_db()).await?;

    if let (Some(page), Some(size)) = (params.page, params.size) {
        let offset = (page - 1) * size;
        sql = sql.offset(offset).limit(size);
    }

    let submissions = sql.all(get_db()).await?;
    let mut submissions = submissions
        .into_iter()
        .map(Submission::from)
        .collect::<Vec<Submission>>();

    submissions = cds_db::transfer::submission::preload(submissions).await?;

    Ok(WebResponse {
        code: StatusCode::OK,
        data: Some(submissions),
        total: Some(total),
        ..Default::default()
    })
}

pub async fn delete_submission(
    Extension(ext): Extension<Ext>, Path(submission_id): Path<i64>,
) -> Result<WebResponse<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    if operator.group != Group::Admin {
        return Err(WebError::Forbidden(json!("")));
    }

    let _ = cds_db::entity::submission::Entity::delete_by_id(submission_id)
        .exec(get_db())
        .await?;

    Ok(WebResponse {
        code: StatusCode::OK,
        ..Default::default()
    })
}
