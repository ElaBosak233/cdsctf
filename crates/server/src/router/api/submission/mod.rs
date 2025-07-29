use axum::{Router, http::StatusCode};
use cds_db::{
    Submission,
    sea_orm::{ActiveValue::NotSet, Set},
    submission::{FindSubmissionsOptions, Status},
    team::{FindTeamOptions, State, Team},
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

use crate::{
    extract::{Extension, Json, Query},
    traits::{AuthPrincipal, WebError, WebResponse},
};

pub fn router() -> Router {
    Router::new()
        .route("/", axum::routing::get(get_submission))
        .route("/", axum::routing::post(create_submission))
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetSubmissionRequest {
    pub id: Option<i64>,
    pub user_id: Option<i64>,
    pub team_id: Option<i64>,
    pub game_id: Option<i64>,
    pub challenge_id: Option<Uuid>,
    pub status: Option<Status>,
    pub page: Option<u64>,
    pub size: Option<u64>,
    pub sorts: Option<String>,
}

pub async fn get_submission(
    Extension(ext): Extension<AuthPrincipal>,
    Query(params): Query<GetSubmissionRequest>,
) -> Result<WebResponse<Vec<Submission>>, WebError> {
    let _ = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;

    let page = params.page.unwrap_or(1);
    let size = params.size.unwrap_or(10).min(100);

    let (submissions, total) = cds_db::submission::find::<Submission>(FindSubmissionsOptions {
        id: params.id,
        user_id: params.user_id,
        team_id: Some(params.team_id),
        game_id: Some(params.game_id),
        challenge_id: params.challenge_id,
        status: params.status,
        page: Some(page),
        size: Some(size),
        sorts: params.sorts,
    })
    .await?;

    let submissions = submissions
        .into_iter()
        .map(|submission| submission.desensitize())
        .collect::<Vec<Submission>>();

    Ok(WebResponse {
        code: StatusCode::OK,
        data: Some(submissions),
        total: Some(total),
        ..Default::default()
    })
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateSubmissionRequest {
    pub content: String,
    pub user_id: Option<i64>,
    pub team_id: Option<i64>,
    pub game_id: Option<i64>,
    pub challenge_id: uuid::Uuid,
}

pub async fn create_submission(
    Extension(ext): Extension<AuthPrincipal>,
    Json(mut body): Json<CreateSubmissionRequest>,
) -> Result<WebResponse<Submission>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;

    body.user_id = Some(operator.id);

    let token = format!("submission:user:{}", operator.id);
    if let Some(limit) = cds_cache::get::<i32>(&token).await? {
        if limit > 10 {
            return Err(WebError::TooManyRequests(json!("submission")));
        } else {
            cds_cache::set_ex(&token, limit + 1, 60).await?;
        }
    } else {
        cds_cache::set_ex(&token, 1, 60).await?;
    }

    let challenge = crate::util::loader::prepare_challenge(body.challenge_id).await?;

    if body.game_id.is_some() != body.team_id.is_some() {
        return Err(WebError::BadRequest(json!("invalid")));
    }

    // If the submission is not in game mode, challenge must be public.
    if !challenge.is_public && (body.game_id.is_none() || body.team_id.is_none()) {
        return Err(WebError::BadRequest(json!("challenge_not_found")));
    }

    if let (Some(game_id), Some(team_id)) = (body.game_id, body.team_id) {
        let game = crate::util::loader::prepare_game(game_id).await?;

        let _ = crate::util::loader::prepare_game_challenge(game_id, challenge.id).await?;

        if cds_db::team::find::<Team>(FindTeamOptions {
            id: Some(team_id),
            game_id: Some(game.id),
            state: Some(State::Passed),
            user_id: Some(operator.id),
            ..Default::default()
        })
        .await?
        .1 == 0
        {
            return Err(WebError::BadRequest(json!("team_not_found")));
        };
    }

    let submission = cds_db::submission::create::<Submission>(cds_db::submission::ActiveModel {
        content: Set(body.content),
        user_id: body.user_id.map_or(NotSet, Set),
        team_id: body.team_id.map_or(NotSet, |v| Set(Some(v))),
        game_id: body.game_id.map_or(NotSet, |v| Set(Some(v))),
        challenge_id: Set(body.challenge_id),
        status: Set(Status::Pending),
        ..Default::default()
    })
    .await?;

    cds_queue::publish("checker", submission.id).await?;

    let submission = cds_db::submission::find_by_id::<Submission>(submission.id).await?;

    Ok(WebResponse {
        code: StatusCode::OK,
        data: submission,
        ..Default::default()
    })
}
