//! Resource-level handlers for `/api/management/submissions/{submission_id}`.

use std::sync::Arc;

use axum::{Json, Router, extract::State};
use cds_db::{
    SubmissionView,
    sea_orm::{
        AccessMode,
        ActiveValue::{self, NotSet, Set, Unchanged},
        IsolationLevel, TransactionTrait,
    },
    submission::{ActiveModel, Status},
};
use cds_worker::checker;
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
    now: time::OffsetDateTime,
) -> (
    ActiveValue<Option<time::OffsetDateTime>>,
    ActiveValue<Option<time::OffsetDateTime>>,
) {
    match next {
        Status::Queued => (Set(None), Set(None)),
        Status::Processing => (Set(Some(now)), Set(None)),
        _ if previous == &Status::Processing => (NotSet, Set(Some(now))),
        _ => (NotSet, NotSet),
    }
}

fn score_recalculation_game_id(
    game_id: Option<i64>,
    previous: &Status,
    next: &Status,
) -> Option<i64> {
    game_id
        .filter(|_| previous != next && (previous == &Status::Correct || next == &Status::Correct))
}

/// Updates the status of a submission.
///
/// Uniqueness of `Correct` submissions per solve scope is enforced directly by
/// PostgreSQL's partial unique indexes (`uq_submissions_correct_*`), which map
/// constraint violations to `409 Conflict`
/// (`correct_submission_already_exists`).
#[utoipa::path(
    patch,
    path = "/status",
    tag = "management-submission",
    params(
        ("submission_id" = i64, Path, description = "Submission id"),
    ),
    request_body = UpdateSubmissionStatusRequest,
    responses(
        (status = 200, description = "Updated submission", body = SubmissionView),
        (status = 409, description = "Correct submission already exists", body = crate::traits::ErrorResponse),
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
    let transaction =
        s.db.conn
            .begin_with_config(
                Some(IsolationLevel::ReadCommitted),
                Some(AccessMode::ReadWrite),
            )
            .await
            .map_err(cds_db::DbError::from)?;
    let previous = cds_db::submission::find_by_id(&transaction, submission_id)
        .await?
        .ok_or_else(|| WebError::NotFound(json!("not_found")))?;

    let now = time::OffsetDateTime::now_utc();
    let next_status = body.status.clone();
    let (processing_at, checked_at) = status_timestamps(&previous.status, &body.status, now);
    let score_game_id =
        score_recalculation_game_id(previous.game_id, &previous.status, &body.status);

    let mut submission = cds_db::submission::update(
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

    if next_status == Status::Processing {
        cds_db::submission::advance_claim_generation(&transaction, submission_id).await?;
    }
    transaction.commit().await.map_err(cds_db::DbError::from)?;

    if next_status == Status::Queued {
        checker::notify(&s.queue).await;
    }
    if let Some(game_id) = score_game_id {
        s.score.invalidate(game_id).await;
    }
    if submission.status == Status::Correct
        && let Some(game_id) = submission.game_id
    {
        let snap = s.score.snapshot(game_id).await?;
        let (pts, rank) = snap.submission_score(submission.id);
        submission.pts = pts;
        submission.rank = rank;
    }

    tracing::info!(
        resource = "submission",
        action = "override_status",
        submission_id = submission.id,
        before = ?previous.status,
        after = ?submission.status,
        "resource step completed",
    );
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
    tag = "management-submission",
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
        .ok_or_else(|| WebError::NotFound(json!("not_found")))?;
    let score_game_id = match (submission.game_id, &submission.status) {
        (Some(game_id), Status::Correct) => Some(game_id),
        _ => None,
    };

    cds_db::submission::delete(&s.db.conn, submission_id).await?;

    if let Some(game_id) = score_game_id {
        s.score.invalidate(game_id).await;
    }

    Ok(Json(EmptyJson::default()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn queued_clears_processing_timestamps() {
        let now = time::OffsetDateTime::from_unix_timestamp(42).unwrap();
        assert_eq!(
            status_timestamps(&Status::Processing, &Status::Queued, now),
            (Set(None), Set(None))
        );
    }

    #[test]
    fn processing_starts_a_new_attempt() {
        let now = time::OffsetDateTime::from_unix_timestamp(42).unwrap();
        assert_eq!(
            status_timestamps(&Status::Queued, &Status::Processing, now),
            (Set(Some(now)), Set(None))
        );
    }

    #[test]
    fn finishing_processing_records_completion() {
        let now = time::OffsetDateTime::from_unix_timestamp(42).unwrap();
        assert_eq!(
            status_timestamps(&Status::Processing, &Status::Incorrect, now),
            (NotSet, Set(Some(now)))
        );
    }

    #[test]
    fn terminal_status_correction_preserves_checker_timestamps() {
        let now = time::OffsetDateTime::from_unix_timestamp(42).unwrap();
        assert_eq!(
            status_timestamps(&Status::Correct, &Status::Incorrect, now),
            (NotSet, NotSet)
        );
    }

    #[test]
    fn score_recalculation_tracks_only_game_correct_transitions() {
        assert_eq!(
            score_recalculation_game_id(Some(7), &Status::Correct, &Status::Incorrect),
            Some(7)
        );
        assert_eq!(
            score_recalculation_game_id(Some(7), &Status::Incorrect, &Status::Correct),
            Some(7)
        );
        assert_eq!(
            score_recalculation_game_id(Some(7), &Status::Correct, &Status::Correct),
            None
        );
        assert_eq!(
            score_recalculation_game_id(None, &Status::Correct, &Status::Incorrect),
            None
        );
    }
}
