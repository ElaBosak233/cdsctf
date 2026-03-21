//! HTTP routing for `submission` — Axum router wiring and OpenAPI route
//! registration.

use std::sync::Arc;

use axum::{Json, Router, extract::State};
use cds_db::{
    Submission,
    submission::{FindSubmissionsOptions, Status},
};
use serde::{Deserialize, Serialize};
use utoipa_axum::{
    router::{OpenApiRouter, UtoipaMethodRouterExt},
    routes,
};

use crate::{
    extract::{Path, Query},
    traits::{AppState, EmptyJson, WebError},
};

/// Builds the Axum router fragment for this module.

pub fn router(state: Arc<AppState>) -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::from(Router::new().with_state(state.clone()))
        .routes(routes!(get_submissions).with_state(state.clone()))
        .routes(routes!(delete_submission).with_state(state.clone()))
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
    pub items: Vec<Submission>,
    pub total: u64,
}

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

/// Returns submissions.
pub async fn get_submissions(
    State(s): State<Arc<AppState>>,

    Query(params): Query<GetSubmissionsRequest>,
) -> Result<Json<ListSubmissionsResponse>, WebError> {
    let page = params.page.unwrap_or(1);
    let size = params.size.unwrap_or(10).max(100);

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

    Ok(Json(ListSubmissionsResponse {
        items: submissions,
        total,
    }))
}

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

/// Deletes submission.
pub async fn delete_submission(
    State(s): State<Arc<AppState>>,

    Path(submission_id): Path<i64>,
) -> Result<Json<EmptyJson>, WebError> {
    cds_db::submission::delete(&s.db.conn, submission_id).await?;

    Ok(Json(EmptyJson::default()))
}
