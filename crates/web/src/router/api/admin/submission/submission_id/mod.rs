//! Resource-level handlers for `/api/admin/submissions/{submission_id}`.

use std::sync::Arc;

use axum::{Json, Router, extract::State};
use cds_db::{
    Submission,
    sea_orm::ActiveValue::{Set, Unchanged},
    submission::ActiveModel,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::info;
use utoipa_axum::{
    router::{OpenApiRouter, UtoipaMethodRouterExt},
    routes,
};

use crate::{
    extract::{Json as ReqJson, Path},
    traits::{AppState, EmptyJson, WebError},
};

/// Builds the Axum router fragment for this module.

pub fn router(state: Arc<AppState>) -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::from(Router::new().with_state(state.clone()))
        .routes(routes!(update_submission_status).with_state(state.clone()))
        .routes(routes!(delete_submission).with_state(state.clone()))
}

#[derive(Clone, Debug, Serialize, Deserialize, utoipa::ToSchema)]
pub struct UpdateSubmissionStatusRequest {
    pub status: cds_db::submission::Status,
}

/// Updates the status of a submission.
#[utoipa::path(
    put,
    path = "/status",
    tag = "admin-submission",
    params(
        ("submission_id" = i64, Path, description = "Submission id"),
    ),
    request_body = UpdateSubmissionStatusRequest,
    responses(
        (status = 200, description = "Updated submission", body = Submission),
        (status = 404, description = "Not found", body = crate::traits::ErrorResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
#[tracing::instrument(skip_all, fields(handler = "update_submission_status"))]
pub async fn update_submission_status(
    State(s): State<Arc<AppState>>,

    Path(submission_id): Path<i64>,
    ReqJson(body): ReqJson<UpdateSubmissionStatusRequest>,
) -> Result<Json<Submission>, WebError> {
    let _submission = cds_db::submission::find_by_id::<Submission>(&s.db.conn, submission_id)
        .await?
        .ok_or_else(|| WebError::NotFound(json!("")))?;

    let submission = cds_db::submission::update::<Submission>(
        &s.db.conn,
        ActiveModel {
            id: Unchanged(submission_id),
            status: Set(body.status),
            ..Default::default()
        },
    )
    .await?;

    info!(
        submission_id = submission.id,
        status = ?submission.status,
        "submission status updated by admin"
    );

    Ok(Json(submission))
}

/// Deletes submission.
#[utoipa::path(
    delete,
    path = "/",
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
