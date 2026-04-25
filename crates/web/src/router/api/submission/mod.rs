//! HTTP routing for `submission` — Axum router wiring and OpenAPI route
//! registration.

use std::sync::Arc;

use axum::{Json, Router, extract::State, http::StatusCode};
use cds_db::{
    Submission, Team,
    sea_orm::{ActiveValue::NotSet, Set},
    submission::{FindSubmissionsOptions, Status},
    team::{FindTeamOptions, State as TState},
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::{debug, info, warn};
use utoipa_axum::{
    router::{OpenApiRouter, UtoipaMethodRouterExt},
    routes,
};

use crate::{
    extract::{Extension, Json as ReqJson, Query},
    traits::{AppState, AuthPrincipal, WebError},
};

/// Builds the Axum router fragment for this module.

pub fn router(state: Arc<AppState>) -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::from(Router::new().with_state(state.clone()))
        .routes(routes!(list_submissions).with_state(state.clone()))
        .routes(routes!(create_submission).with_state(state.clone()))
}

#[derive(Clone, Debug, Serialize, Deserialize, utoipa::ToSchema, utoipa::IntoParams)]
#[into_params(parameter_in = Query)]
pub struct ListSubmissionsRequest {
    pub id: Option<i64>,
    pub user_id: Option<i64>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    pub team_id: Option<Option<i64>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    pub game_id: Option<Option<i64>>,
    pub challenge_id: Option<i64>,
    pub status: Option<Status>,
    pub page: Option<u64>,
    pub size: Option<u64>,
    pub sorts: Option<String>,
}

#[derive(Clone, Debug, Serialize, utoipa::ToSchema)]
pub struct ListSubmissionsResponse {
    pub submissions: Vec<Submission>,
    pub total: u64,
}

/// Lists submissions for the current user (collection).
#[utoipa::path(
    get,
    path = "/",
    tag = "submission",
    params(ListSubmissionsRequest),
    responses(
        (status = 200, description = "Submissions (content redacted)", body = ListSubmissionsResponse),
        (status = 401, description = "Unauthorized", body = crate::traits::ErrorResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
#[tracing::instrument(skip_all, fields(handler = "list_submissions"))]
pub async fn list_submissions(
    State(s): State<Arc<AppState>>,

    Extension(ext): Extension<AuthPrincipal>,
    Query(params): Query<ListSubmissionsRequest>,
) -> Result<Json<ListSubmissionsResponse>, WebError> {
    let _ = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;

    let page = params.page.unwrap_or(1);
    let size = params.size.unwrap_or(10).min(100);

    let (submissions, total) = cds_db::submission::find(
        &s.db.conn,
        FindSubmissionsOptions {
            id: params.id,
            user_id: params.user_id,
            team_id: params.team_id,
            game_id: params.game_id,
            challenge_id: params.challenge_id,
            status: params.status,
            page: Some(page),
            size: Some(size),
            sorts: params.sorts,
        },
    )
    .await?;

    let submissions = submissions
        .into_iter()
        .map(|submission: Submission| submission.desensitize())
        .collect::<Vec<Submission>>();
    debug!(
        page,
        size,
        returned = submissions.len(),
        total,
        "submissions listed"
    );

    Ok(Json(ListSubmissionsResponse { submissions, total }))
}

#[derive(Clone, Debug, Serialize, Deserialize, utoipa::ToSchema)]
pub struct CreateSubmissionRequest {
    pub content: String,
    pub team_id: Option<i64>,
    pub game_id: Option<i64>,
    pub challenge_id: i64,
}

/// Creates a submission; author is always the authenticated user.
#[utoipa::path(
    post,
    path = "/",
    tag = "submission",
    request_body = CreateSubmissionRequest,
    responses(
        (status = 201, description = "Created submission", body = Submission),
        (status = 400, description = "Bad request", body = crate::traits::ErrorResponse),
        (status = 401, description = "Unauthorized", body = crate::traits::ErrorResponse),
        (status = 404, description = "Not found", body = crate::traits::ErrorResponse),
        (status = 429, description = "Rate limited", body = crate::traits::ErrorResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
#[tracing::instrument(skip_all, fields(handler = "create_submission"))]
pub async fn create_submission(
    State(s): State<Arc<AppState>>,

    Extension(ext): Extension<AuthPrincipal>,
    ReqJson(body): ReqJson<CreateSubmissionRequest>,
) -> Result<(StatusCode, Json<Submission>), WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;

    let token = format!("submission:user:{}", operator.id);
    if let Some(limit) = s.cache.get::<i32>(&token).await? {
        if limit > 10 {
            warn!(
                user_id = operator.id,
                limit, "submission rate limit exceeded"
            );
            return Err(WebError::TooManyRequests(json!("submission")));
        } else {
            s.cache.set_ex(&token, limit + 1, 60).await?;
        }
    } else {
        s.cache.set_ex(&token, 1, 60).await?;
    }

    let challenge = crate::util::loader::prepare_challenge(&s.db.conn, body.challenge_id).await?;

    if body.game_id.is_some() != body.team_id.is_some() {
        return Err(WebError::BadRequest(json!("invalid")));
    }

    // If the submission is not in game mode, challenge must be public.
    if !challenge.public && (body.game_id.is_none() || body.team_id.is_none()) {
        return Err(WebError::BadRequest(json!("challenge_not_found")));
    }

    if let (Some(game_id), Some(team_id)) = (body.game_id, body.team_id) {
        let game = crate::util::loader::prepare_game(&s.db.conn, game_id).await?;

        let _ =
            crate::util::loader::prepare_game_challenge(&s.db.conn, game_id, challenge.id).await?;

        if cds_db::team::find::<Team>(
            &s.db.conn,
            FindTeamOptions {
                id: Some(team_id),
                game_id: Some(game.id),
                state: Some(TState::Passed),
                user_id: Some(operator.id),
                ..Default::default()
            },
        )
        .await?
        .1 == 0
        {
            return Err(WebError::BadRequest(json!("team_not_found")));
        };
    }

    let submission = cds_db::submission::create::<Submission>(
        &s.db.conn,
        cds_db::submission::ActiveModel {
            content: Set(body.content),
            user_id: Set(operator.id),
            team_id: body.team_id.map_or(NotSet, |v| Set(Some(v))),
            game_id: body.game_id.map_or(NotSet, |v| Set(Some(v))),
            challenge_id: Set(body.challenge_id),
            status: Set(Status::Pending),
            ..Default::default()
        },
    )
    .await?;

    s.queue.publish("checker", submission.id).await?;
    info!(
        submission_id = submission.id,
        user_id = operator.id,
        team_id = body.team_id,
        game_id = body.game_id,
        challenge_id = body.challenge_id,
        subject = "checker",
        "submission created and queued"
    );

    let submission = cds_db::submission::find_by_id(&s.db.conn, submission.id)
        .await?
        .ok_or_else(|| WebError::NotFound(json!("")))?;

    Ok((StatusCode::CREATED, Json(submission)))
}
