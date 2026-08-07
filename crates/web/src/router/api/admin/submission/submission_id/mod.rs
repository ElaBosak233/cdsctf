//! Resource-level handlers for `/api/admin/submissions/{submission_id}`.

use std::sync::Arc;

use axum::{Json, Router, extract::State};
use cds_db::{
    SubmissionView,
    sea_orm::{
        ActiveValue::{self, NotSet, Set, Unchanged},
        TransactionTrait,
    },
    submission::{ActiveModel, Status},
};
use cds_worker::calculator;
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

fn status_timestamps(
    previous: &Status,
    next: &Status,
    now: i64,
) -> (ActiveValue<Option<i64>>, ActiveValue<Option<i64>>) {
    match next {
        Status::Queued => (Set(None), Set(None)),
        Status::Processing => (Set(Some(now)), Set(None)),
        _ if previous == &Status::Processing => (NotSet, Set(Some(now))),
        _ => (NotSet, NotSet),
    }
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
        (status = 200, description = "Updated submission", body = SubmissionView),
        (status = 404, description = "Not found", body = crate::traits::ErrorResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
#[tracing::instrument(skip_all, fields(handler = "update_submission_status"))]
pub async fn update_submission_status(
    State(s): State<Arc<AppState>>,

    Path(submission_id): Path<i64>,
    ReqJson(body): ReqJson<UpdateSubmissionStatusRequest>,
) -> Result<Json<SubmissionView>, WebError> {
    let previous = cds_db::submission::find_by_id(&s.db.conn, submission_id)
        .await?
        .ok_or_else(|| WebError::NotFound(json!("")))?;

    let now = time::OffsetDateTime::now_utc().unix_timestamp();
    let (processing_at, checked_at) = status_timestamps(&previous.status, &body.status, now);

    let transaction = s.db.conn.begin().await.map_err(cds_db::DbError::from)?;
    let submission = cds_db::submission::update(
        &transaction,
        ActiveModel {
            id: Unchanged(submission_id),
            status: Set(body.status),
            processing_at,
            checked_at,
            ..Default::default()
        },
    )
    .await?;

    let score_game_id = if let Some(game_id) = submission.game_id
        && previous.status != submission.status
        && (previous.status == Status::Correct || submission.status == Status::Correct)
    {
        cds_db::game::request_score_recalculation(&transaction, game_id).await?;
        Some(game_id)
    } else {
        None
    };
    transaction.commit().await.map_err(cds_db::DbError::from)?;

    if let Some(game_id) = score_game_id {
        calculator::notify(&s.queue, game_id).await;
    }

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
    let submission = cds_db::submission::find_by_id(&s.db.conn, submission_id)
        .await?
        .ok_or_else(|| WebError::NotFound(json!("")))?;
    let transaction = s.db.conn.begin().await.map_err(cds_db::DbError::from)?;
    cds_db::submission::delete(&transaction, submission_id).await?;
    let score_game_id =
        if let (Some(game_id), Status::Correct) = (submission.game_id, submission.status) {
            cds_db::game::request_score_recalculation(&transaction, game_id).await?;
            Some(game_id)
        } else {
            None
        };
    transaction.commit().await.map_err(cds_db::DbError::from)?;

    if let Some(game_id) = score_game_id {
        calculator::notify(&s.queue, game_id).await;
    }

    Ok(Json(EmptyJson::default()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn queued_clears_processing_timestamps() {
        assert_eq!(
            status_timestamps(&Status::Processing, &Status::Queued, 42),
            (Set(None), Set(None))
        );
    }

    #[test]
    fn processing_starts_a_new_attempt() {
        assert_eq!(
            status_timestamps(&Status::Queued, &Status::Processing, 42),
            (Set(Some(42)), Set(None))
        );
    }

    #[test]
    fn finishing_processing_records_completion() {
        assert_eq!(
            status_timestamps(&Status::Processing, &Status::Incorrect, 42),
            (NotSet, Set(Some(42)))
        );
    }

    #[test]
    fn terminal_status_correction_preserves_checker_timestamps() {
        assert_eq!(
            status_timestamps(&Status::Correct, &Status::Incorrect, 42),
            (NotSet, NotSet)
        );
    }
}
