//! HTTP routing for `challenge` — Axum router wiring and OpenAPI route
//! registration.

/// Defines the `challenge_id` submodule (see sibling `*.rs` files).
mod challenge_id;

use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use axum::{Json, Router, extract::State};
use cds_db::{ChallengeSummary, SubmissionSummary, challenge::FindChallengeOptions};
use serde::{Deserialize, Serialize};
use serde_json::json;
use utoipa_axum::{
    router::{OpenApiRouter, UtoipaMethodRouterExt},
    routes,
};

use crate::{
    extract::{Extension, Json as ReqJson, Query},
    middleware::auth::basic_operator,
    traits::{AppState, AuthPrincipal, WebError},
};

/// Builds the Axum router fragment for this module.

pub fn router(state: Arc<AppState>) -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::from(Router::new().with_state(state.clone()))
        .routes(routes!(query_challenge_status).with_state(state.clone()))
        .routes(routes!(list_challenges).with_state(state.clone()))
        .nest("/{challenge_id}", challenge_id::router(state.clone()))
}

#[derive(Clone, Debug, Serialize, Deserialize, utoipa::ToSchema, utoipa::IntoParams)]
#[into_params(parameter_in = Query)]
pub struct ListChallengesRequest {
    pub id: Option<i64>,
    pub title: Option<String>,
    pub category: Option<i32>,
    pub tag: Option<String>,
    pub page: Option<u64>,
    pub size: Option<u64>,
    pub sorts: Option<String>,
}

#[derive(Clone, Debug, Serialize, utoipa::ToSchema)]
pub struct ChallengesListResponse {
    pub challenges: Vec<ChallengeSummary>,
    pub total: u64,
}

/// Lists public challenges (collection).
#[utoipa::path(
    get,
    path = "/",
    tag = "challenge",
    params(ListChallengesRequest),
    responses(
        (status = 200, description = "Challenges", body = ChallengesListResponse),
        (status = 401, description = "Unauthorized", body = crate::traits::ErrorResponse),
        (status = 403, description = "Basic access required", body = crate::traits::ErrorResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
#[tracing::instrument(skip_all, fields(handler = "list_challenges"))]
pub async fn list_challenges(
    State(s): State<Arc<AppState>>,
    Extension(ext): Extension<AuthPrincipal>,
    Query(params): Query<ListChallengesRequest>,
) -> Result<Json<ChallengesListResponse>, WebError> {
    let _ = basic_operator(&ext)?;

    let page = params.page.unwrap_or(1);
    let size = params.size.unwrap_or(10).min(100);

    let (challenges, total) = cds_db::challenge::find::<ChallengeSummary>(
        &s.db.conn,
        FindChallengeOptions {
            id: params.id,
            title: params.title,
            category: params.category,
            tag: params.tag,
            public: Some(true),
            sorts: params.sorts,
            page: Some(page),
            size: Some(size),
            ..Default::default()
        },
    )
    .await?;

    Ok(Json(ChallengesListResponse { challenges, total }))
}

#[derive(Clone, Debug, Serialize, Deserialize, utoipa::ToSchema)]
pub struct QueryChallengeStatusRequest {
    pub challenge_ids: Vec<i64>,
    pub user_id: Option<i64>,
    pub team_id: Option<i64>,
    pub game_id: Option<i64>,
}

#[derive(Clone, Debug, Serialize, Deserialize, utoipa::ToSchema)]
pub struct ChallengeStatusResponse {
    pub solved: bool,
    pub solved_times: i64,
    pub pts: i64,
    pub bloods: Vec<SubmissionSummary>,
    pub cheated: bool,
}

#[derive(Clone, Debug, Serialize, utoipa::ToSchema)]
pub struct ChallengeStatusesResponse {
    pub statuses: HashMap<i64, ChallengeStatusResponse>,
}

fn valid_status_scope(user_id: Option<i64>, team_id: Option<i64>, game_id: Option<i64>) -> bool {
    matches!(
        (user_id, team_id, game_id),
        (Some(_), None, None) | (None, Some(_), Some(_))
    )
}

/// Batch query for solve status and score hints. Uses POST so `challenge_ids`
/// can be a JSON array.
#[utoipa::path(
    post,
    path = "/status",
    tag = "challenge",
    request_body = QueryChallengeStatusRequest,
    responses(
        (status = 200, description = "Per-challenge status", body = ChallengeStatusesResponse),
        (status = 400, description = "Bad request", body = crate::traits::ErrorResponse),
        (status = 401, description = "Unauthorized", body = crate::traits::ErrorResponse),
        (status = 403, description = "Basic access required", body = crate::traits::ErrorResponse),
        (status = 423, description = "Game paused", body = crate::traits::ErrorResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
#[tracing::instrument(skip_all, fields(handler = "query_challenge_status"))]
pub async fn query_challenge_status(
    State(s): State<Arc<AppState>>,
    Extension(ext): Extension<AuthPrincipal>,
    ReqJson(body): ReqJson<QueryChallengeStatusRequest>,
) -> Result<Json<ChallengeStatusesResponse>, WebError> {
    let _ = basic_operator(&ext)?;

    if !valid_status_scope(body.user_id, body.team_id, body.game_id) {
        return Err(WebError::BadRequest(json!("either_user_or_team")));
    }

    if let Some(game_id) = body.game_id {
        let game = crate::util::loader::prepare_game(&s.db.conn, game_id).await?;
        crate::util::loader::ensure_game_not_paused(&game)?;
    }

    let mut result: HashMap<i64, ChallengeStatusResponse> =
        HashMap::with_capacity(body.challenge_ids.len());
    for &challenge_id in &body.challenge_ids {
        result.insert(
            challenge_id,
            ChallengeStatusResponse {
                solved: false,
                solved_times: 0,
                pts: 0,
                bloods: Vec::new(),
                cheated: false,
            },
        );
    }

    if let (Some(team_id), Some(game_id)) = (body.team_id, body.game_id) {
        // Game scope: serve solve counts, points, solved flag, and top-3 blood
        // IDs directly from the CQRS score snapshot (O(challenges) instead of
        // scanning all historical solves with 4-way JOINs).
        let snap = s.score.snapshot(game_id).await?;
        let mut blood_ids = Vec::with_capacity(body.challenge_ids.len() * 3);

        for &challenge_id in &body.challenge_ids {
            if let Some(status) = result.get_mut(&challenge_id) {
                status.solved = snap.team_solved(team_id, challenge_id);
                if let Some(cs) = snap.challenges.get(&challenge_id) {
                    status.pts = cs.pts;
                    status.solved_times = cs.solve_count;
                    blood_ids.extend_from_slice(&cs.blood_submission_ids);
                }
            }
        }

        // Batch-load display metadata ONLY for the <= 3 blood submissions per
        // challenge.
        let mut blood_views = cds_db::submission::find_views_by_ids(&s.db.conn, &blood_ids).await?;
        for view in &mut blood_views {
            let (pts, rank) = snap.submission_score(view.id);
            view.pts = pts;
            view.rank = rank;
        }
        for view in &blood_views {
            if let Some(status) = result.get_mut(&view.challenge_id)
                && status.bloods.len() < 3
            {
                status.bloods.push(SubmissionSummary::from(view));
            }
        }

        // Check for Cheat submissions in the requested game scope (uses partial
        // index `idx_submissions_cheat`).
        let cheated_ids = cds_db::submission::find_cheat_challenge_ids(
            &s.db.conn,
            body.challenge_ids.clone(),
            team_id,
            game_id,
        )
        .await?;
        let cheated_set: HashSet<i64> = cheated_ids.into_iter().collect();
        for &challenge_id in &body.challenge_ids {
            if cheated_set.contains(&challenge_id)
                && let Some(status) = result.get_mut(&challenge_id)
            {
                status.cheated = true;
            }
        }
    } else if let Some(user_id) = body.user_id {
        // Standalone practice scope: SQL-aggregated solve counts + top-3 bloods
        // only, avoiding full submission history scans.
        let stats = cds_db::submission::find_standalone_challenge_stats(
            &s.db.conn,
            &body.challenge_ids,
            user_id,
        )
        .await?;
        let mut blood_ids = Vec::with_capacity(stats.len() * 3);
        for (&challenge_id, stat) in &stats {
            if let Some(status) = result.get_mut(&challenge_id) {
                status.solved = stat.solved;
                status.solved_times = stat.solved_times;
            }
            blood_ids.extend_from_slice(&stat.blood_ids);
        }
        let blood_views = cds_db::submission::find_views_by_ids(&s.db.conn, &blood_ids).await?;
        for view in &blood_views {
            if let Some(status) = result.get_mut(&view.challenge_id)
                && status.bloods.len() < 3
            {
                status.bloods.push(SubmissionSummary::from(view));
            }
        }
    }

    Ok(Json(ChallengeStatusesResponse { statuses: result }))
}

#[cfg(test)]
mod tests {
    use super::valid_status_scope;

    #[test]
    fn challenge_status_requires_one_complete_subject_scope() {
        assert!(valid_status_scope(Some(1), None, None));
        assert!(valid_status_scope(None, Some(2), Some(3)));

        assert!(!valid_status_scope(None, None, None));
        assert!(!valid_status_scope(None, Some(2), None));
        assert!(!valid_status_scope(None, None, Some(3)));
        assert!(!valid_status_scope(Some(1), Some(2), Some(3)));
    }
}
