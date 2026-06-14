//! HTTP routing for `submission` — Axum router wiring and OpenAPI route
//! registration.

use std::sync::Arc;

use axum::{Json, Router, extract::State};
use cds_db::{
    Submission,
    submission::{FindSubmissionsOptions, Status},
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::{info, warn};
use utoipa_axum::{
    router::{OpenApiRouter, UtoipaMethodRouterExt},
    routes,
};

use crate::{
    extract::{Extension, Json as ReqJson, Path, Query},
    traits::{AppState, AuthPrincipal, EmptyJson, WebError},
};

/// Builds the Axum router fragment for this module.

pub fn router(state: Arc<AppState>) -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::from(Router::new().with_state(state.clone()))
        .routes(routes!(get_submissions).with_state(state.clone()))
        .routes(routes!(delete_submission).with_state(state.clone()))
        .routes(routes!(create_debug_submission).with_state(state.clone()))
}

#[derive(Clone, Debug, Serialize, Deserialize, utoipa::ToSchema, utoipa::IntoParams)]
#[into_params(parameter_in = Query)]
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

#[derive(Clone, Debug, Serialize, utoipa::ToSchema)]
pub struct ListSubmissionsResponse {
    pub submissions: Vec<Submission>,
    pub total: u64,
}

/// Returns submissions.
#[utoipa::path(
    get,
    path = "/",
    tag = "admin-submission",
    params(GetSubmissionsRequest),
    responses(
        (status = 200, description = "Submissions", body = ListSubmissionsResponse),
        (status = 404, description = "Not found", body = crate::traits::ErrorResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
#[tracing::instrument(skip_all, fields(handler = "get_submissions"))]
pub async fn get_submissions(
    State(s): State<Arc<AppState>>,

    Query(params): Query<GetSubmissionsRequest>,
) -> Result<Json<ListSubmissionsResponse>, WebError> {
    let page = params.page.unwrap_or(1);
    let size = params.size.unwrap_or(10).min(100);

    let (submissions, total) = cds_db::submission::find(
        &s.db.conn,
        FindSubmissionsOptions {
            id: params.id,
            user_id: params.user_id,
            team_id: Some(params.team_id),
            game_id: Some(params.game_id),
            challenge_id: params.challenge_id,
            status: params.status,
            page: Some(page),
            size: Some(size),
            ..Default::default()
        },
    )
    .await?;

    Ok(Json(ListSubmissionsResponse { submissions, total }))
}

/// Deletes submission.
#[utoipa::path(
    delete,
    path = "/{submission_id}",
    tag = "admin-submission",
    params(
        ("submission_id" = i64, Path, description = "Submission id"),
    ),
    responses(
        (status = 200, description = "Deleted", body = EmptyJson),
        (status = 404, description = "Not found", body = crate::traits::ErrorResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
#[tracing::instrument(skip_all, fields(handler = "delete_submission"))]
pub async fn delete_submission(
    State(s): State<Arc<AppState>>,

    Path(submission_id): Path<i64>,
) -> Result<Json<EmptyJson>, WebError> {
    cds_db::submission::delete(&s.db.conn, submission_id).await?;

    Ok(Json(EmptyJson::default()))
}

#[derive(Clone, Debug, Serialize, Deserialize, utoipa::ToSchema)]
pub struct CreateDebugSubmissionRequest {
    pub content: String,
    pub challenge_id: i64,
}

#[derive(Clone, Debug, Serialize, utoipa::ToSchema)]
pub struct CreateDebugSubmissionResponse {
    pub status: Status,
}

/// Debug-submits a flag for immediate feedback without recording a submission.
///
/// Runs the checker script synchronously and returns the result directly.
/// Does **not** create a submission record, affect counts, or trigger scoring.
/// Intended for admin challenge preview use.
#[utoipa::path(
    post,
    path = "/debug",
    tag = "admin-submission",
    request_body = CreateDebugSubmissionRequest,
    responses(
        (status = 200, description = "Debug check result", body = CreateDebugSubmissionResponse),
        (status = 400, description = "Bad request", body = crate::traits::ErrorResponse),
        (status = 401, description = "Unauthorized", body = crate::traits::ErrorResponse),
        (status = 404, description = "Not found", body = crate::traits::ErrorResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
#[tracing::instrument(skip_all, fields(handler = "debug_submit"))]
pub async fn create_debug_submission(
    State(s): State<Arc<AppState>>,

    Extension(ext): Extension<AuthPrincipal>,
    ReqJson(body): ReqJson<CreateDebugSubmissionRequest>,
) -> Result<Json<CreateDebugSubmissionResponse>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;

    // Validate challenge exists.
    let challenge = crate::util::loader::prepare_challenge(&s.db.conn, body.challenge_id).await?;

    // Run the Rune checker synchronously (no queue, no DB record).
    let result = s
        .checker
        .check(&challenge, operator.id, &body.content)
        .await;

    let status = match result {
        Ok(cds_checker::Status::Correct) => Status::Correct,
        Ok(cds_checker::Status::Cheat(peer_id)) => {
            warn!(
                user_id = operator.id,
                peer_team_id = peer_id,
                "cheat detected in debug submit"
            );
            Status::Cheat
        }
        _ => Status::Incorrect,
    };

    info!(
        user_id = operator.id,
        challenge_id = body.challenge_id,
        status = ?status,
        "debug submit result"
    );

    Ok(Json(CreateDebugSubmissionResponse { status }))
}
