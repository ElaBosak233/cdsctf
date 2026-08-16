//! Resource-level handlers for `/api/admin/submissions/{submission_id}`.

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

fn status_scores(previous: &Status, next: &Status) -> (ActiveValue<i64>, ActiveValue<i64>) {
    if previous == &Status::Correct && next == &Status::Correct {
        (NotSet, NotSet)
    } else {
        (Set(0), Set(0))
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
    let initial = cds_db::submission::find_by_id(&transaction, submission_id)
        .await?
        .ok_or_else(|| WebError::NotFound(json!("")))?;
    cds_db::submission::lock_solve_owner(&transaction, &initial).await?;
    let previous = cds_db::submission::find_by_id(&transaction, submission_id)
        .await?
        .ok_or_else(|| WebError::NotFound(json!("")))?;

    if body.status == Status::Correct
        && previous.status != Status::Correct
        && cds_db::submission::has_other_correct_in_scope(&transaction, &previous).await?
    {
        transaction
            .rollback()
            .await
            .map_err(cds_db::DbError::from)?;
        return Err(WebError::Conflict(json!(
            "correct_submission_already_exists"
        )));
    }

    let now = time::OffsetDateTime::now_utc().unix_timestamp();
    let (processing_at, checked_at) = status_timestamps(&previous.status, &body.status, now);
    let (pts, rank) = status_scores(&previous.status, &body.status);
    let score_game_id =
        score_recalculation_game_id(previous.game_id, &previous.status, &body.status);
    if let Some(game_id) = score_game_id {
        // The calculator may already have read this Correct row. Lock before
        // changing it so an old score plan cannot write points back afterward.
        cds_db::game::lock_score_recalculation(&transaction, game_id).await?;
    }

    let submission = cds_db::submission::update(
        &transaction,
        ActiveModel {
            id: Unchanged(submission_id),
            status: Set(body.status),
            processing_at,
            checked_at,
            pts,
            rank,
            ..Default::default()
        },
    )
    .await?;

    if let Some(game_id) = score_game_id {
        cds_db::game::request_score_recalculation(&transaction, game_id).await?;
    }
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
    let transaction =
        s.db.conn
            .begin_with_config(
                Some(IsolationLevel::ReadCommitted),
                Some(AccessMode::ReadWrite),
            )
            .await
            .map_err(cds_db::DbError::from)?;
    let initial = cds_db::submission::find_by_id(&transaction, submission_id)
        .await?
        .ok_or_else(|| WebError::NotFound(json!("")))?;
    cds_db::submission::lock_solve_owner(&transaction, &initial).await?;
    let submission = cds_db::submission::find_by_id(&transaction, submission_id)
        .await?
        .ok_or_else(|| WebError::NotFound(json!("")))?;
    let score_game_id = match (submission.game_id, &submission.status) {
        (Some(game_id), Status::Correct) => {
            cds_db::game::lock_score_recalculation(&transaction, game_id).await?;
            Some(game_id)
        }
        _ => None,
    };
    cds_db::submission::delete(&transaction, submission_id).await?;
    if let Some(game_id) = score_game_id {
        cds_db::game::request_score_recalculation(&transaction, game_id).await?;
    }
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

    #[test]
    fn status_change_clears_persisted_score_fields() {
        assert_eq!(
            status_scores(&Status::Correct, &Status::Incorrect),
            (Set(0), Set(0))
        );
        assert_eq!(
            status_scores(&Status::Correct, &Status::Correct),
            (NotSet, NotSet)
        );
        assert_eq!(
            status_scores(&Status::Incorrect, &Status::Incorrect),
            (Set(0), Set(0))
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
