use axum::{Router, http::StatusCode};
use cds_db::{
    entity::{submission::Status, user::Group},
    get_db,
    sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QuerySelect},
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    extract::{Extension, Path, Query},
    model::submission::Submission,
    traits::{AuthPrincipal, WebError, WebResponse},
};

pub fn router() -> Router {
    Router::new()
        .route("/", axum::routing::get(get_submissions))
        .route("/{submission_id}", axum::routing::delete(delete_submission))
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetSubmissionsRequest {
    pub id: Option<i64>,
    pub user_id: Option<i64>,
    pub team_id: Option<i64>,
    pub game_id: Option<i64>,
    pub challenge_id: Option<i64>,
    pub status: Option<Status>,
    pub page: Option<u64>,
    pub size: Option<u64>,
}

pub async fn get_submissions(
    Query(params): Query<GetSubmissionsRequest>,
) -> Result<WebResponse<Vec<Submission>>, WebError> {
    let page = params.page.unwrap_or(1);
    let size = params.size.unwrap_or(10).max(100);

    let mut sql = cds_db::entity::submission::Entity::base_find();

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

    let offset = (page - 1) * size;
    sql = sql.offset(offset).limit(size);

    let submissions = sql.into_model::<Submission>().all(get_db()).await?;

    Ok(WebResponse {
        code: StatusCode::OK,
        data: Some(submissions),
        total: Some(total),
        ..Default::default()
    })
}

pub async fn delete_submission(
    Extension(ext): Extension<AuthPrincipal>,
    Path(submission_id): Path<i64>,
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
