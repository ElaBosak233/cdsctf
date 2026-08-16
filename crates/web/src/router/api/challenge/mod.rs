//! HTTP routing for `challenge` — Axum router wiring and OpenAPI route
//! registration.

/// Defines the `challenge_id` submodule (see sibling `*.rs` files).
mod challenge_id;

use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use axum::{Json, Router, extract::State};
use cds_db::{
    ChallengeSummary, GameChallengeView, SubmissionSummary, challenge::FindChallengeOptions,
    game_challenge::FindGameChallengeOptions,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
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
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
#[tracing::instrument(skip_all, fields(handler = "list_challenges"))]
pub async fn list_challenges(
    State(s): State<Arc<AppState>>,
    Extension(ext): Extension<AuthPrincipal>,
    Query(params): Query<ListChallengesRequest>,
) -> Result<Json<ChallengesListResponse>, WebError> {
    let _ = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;

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
    let _ = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;

    if !valid_status_scope(body.user_id, body.team_id, body.game_id) {
        return Err(WebError::BadRequest(json!("either_user_or_team")));
    }

    if let Some(game_id) = body.game_id {
        let game = crate::util::loader::prepare_game(&s.db.conn, game_id).await?;
        crate::util::loader::ensure_game_not_paused(&game)?;
    }

    let submissions = cds_db::submission::find_correct_by_challenge_ids_and_game_id(
        &s.db.conn,
        body.challenge_ids.clone(),
        body.game_id,
    )
    .await?;

    let mut result: HashMap<i64, ChallengeStatusResponse> = HashMap::new();

    for challenge_id in body.challenge_ids.iter() {
        result.insert(
            *challenge_id,
            ChallengeStatusResponse {
                solved: false,
                solved_times: 0,
                pts: 0,
                bloods: Vec::new(),
                cheated: false,
            },
        );
    }

    for submission in &submissions {
        if let Some(status_response) = result.get_mut(&submission.challenge_id) {
            if Some(submission.user_id) == body.user_id
                || submission
                    .team_id
                    .is_some_and(|team_id| Some(team_id) == body.team_id)
            {
                status_response.solved = true;
            }

            status_response.solved_times += 1;

            if status_response.bloods.len() < 3 {
                status_response
                    .bloods
                    .push(SubmissionSummary::from(submission));
            }
        }
    }

    // Check for Cheat submissions in the requested game scope.
    if let (Some(team_id), Some(game_id)) = (body.team_id, body.game_id) {
        let cheated_ids = cds_db::submission::find_cheat_challenge_ids(
            &s.db.conn,
            body.challenge_ids.clone(),
            team_id,
            game_id,
        )
        .await?;
        let cheated_set: HashSet<i64> = cheated_ids.into_iter().collect();
        for challenge_id in body.challenge_ids.iter() {
            if cheated_set.contains(challenge_id) {
                if let Some(status_response) = result.get_mut(challenge_id) {
                    status_response.cheated = true;
                }
            }
        }
    }

    if let Some(game_id) = body.game_id {
        let (game_challenges, _) = cds_db::game_challenge::find::<GameChallengeView>(
            &s.db.conn,
            FindGameChallengeOptions {
                game_id: Some(game_id),
                ..Default::default()
            },
        )
        .await?;

        for game_challenge in game_challenges {
            if let Some(status_response) = result.get_mut(&game_challenge.challenge_id) {
                status_response.pts = game_challenge.pts;
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
