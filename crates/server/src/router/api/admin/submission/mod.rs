use axum::{http::StatusCode, Router};
use cds_db::{
    submission::{FindSubmissionsOptions, Status},
    Submission,
};
use serde::{Deserialize, Serialize};

use crate::{
    extract::{Path, Query},
    traits::{WebError, WebResponse},
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

    let (submissions, total) = cds_db::submission::find::<Submission>(FindSubmissionsOptions {
        id: params.id,
        user_id: params.user_id,
        team_id: Some(params.team_id),
        game_id: Some(params.game_id),
        challenge_id: params.challenge_id,
        status: params.status,
        page: Some(page),
        size: Some(size),
        ..Default::default()
    })
    .await?;

    Ok(WebResponse {
        code: StatusCode::OK,
        data: Some(submissions),
        total: Some(total),
        ..Default::default()
    })
}

pub async fn delete_submission(
    Path(submission_id): Path<i64>,
) -> Result<WebResponse<()>, WebError> {
    cds_db::submission::delete(submission_id).await?;

    Ok(WebResponse {
        code: StatusCode::OK,
        ..Default::default()
    })
}
