//! HTTP routing for `challenge` — Axum router wiring and OpenAPI route
//! registration.

/// Defines the `challenge_id` submodule (see sibling `*.rs` files).
mod challenge_id;

use std::{collections::HashMap, sync::Arc};

use axum::{Json, Router, extract::State};
use cds_db::{
    ChallengeMini, GameChallenge, Submission, challenge::FindChallengeOptions,
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
        .routes(routes!(get_challenge).with_state(state.clone()))
        .routes(routes!(get_challenge_status).with_state(state.clone()))
        .nest("/{challenge_id}", challenge_id::router(state.clone()))
}

#[derive(Clone, Debug, Serialize, Deserialize, utoipa::ToSchema, utoipa::IntoParams)]
#[into_params(parameter_in = Query)]
pub struct GetChallengeRequest {
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
    pub challenges: Vec<ChallengeMini>,
    pub total: u64,
}

#[utoipa::path(
    get,
    path = "/playground",
    tag = "challenge",
    params(GetChallengeRequest),
    responses(
        (status = 200, description = "Challenges", body = ChallengesListResponse),
        (status = 401, description = "Unauthorized", body = crate::traits::ErrorResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]

/// Returns challenge.
pub async fn get_challenge(
    State(s): State<Arc<AppState>>,
    Extension(ext): Extension<AuthPrincipal>,
    Query(params): Query<GetChallengeRequest>,
) -> Result<Json<ChallengesListResponse>, WebError> {
    let _ = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;

    let page = params.page.unwrap_or(1);
    let size = params.size.unwrap_or(10).min(100);

    let (challenges, total) = cds_db::challenge::find::<ChallengeMini>(
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

    Ok(Json(ChallengesListResponse {
        challenges,
        total,
    }))
}

#[derive(Clone, Debug, Serialize, Deserialize, utoipa::ToSchema)]
pub struct GetChallengeStatusRequest {
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
    pub bloods: Vec<Submission>,
}

#[derive(Clone, Debug, Serialize, utoipa::ToSchema)]
pub struct ChallengeStatusesResponse {
    pub statuses: HashMap<i64, ChallengeStatusResponse>,
}

#[utoipa::path(
    post,
    path = "/status",
    tag = "challenge",
    request_body = GetChallengeStatusRequest,
    responses(
        (status = 200, description = "Per-challenge status", body = ChallengeStatusesResponse),
        (status = 400, description = "Bad request", body = crate::traits::ErrorResponse),
        (status = 401, description = "Unauthorized", body = crate::traits::ErrorResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]

/// Returns challenge status.
pub async fn get_challenge_status(
    State(s): State<Arc<AppState>>,
    Extension(ext): Extension<AuthPrincipal>,
    ReqJson(body): ReqJson<GetChallengeStatusRequest>,
) -> Result<Json<ChallengeStatusesResponse>, WebError> {
    let _ = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;

    if body.user_id.is_some() && (body.team_id.is_some() || body.game_id.is_some()) {
        return Err(WebError::BadRequest(json!("either_user_or_team")));
    }

    let mut submissions =
        cds_db::submission::find_correct_by_challenge_ids_and_optional_team_game::<Submission>(
            &s.db.conn,
            body.challenge_ids.clone(),
            body.team_id,
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
            },
        );
    }

    for submission in submissions.iter_mut() {
        *submission = submission.desensitize();

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
                status_response.bloods.push(submission.clone());
            }
        }
    }

    if let Some(game_id) = body.game_id {
        let (game_challenges, _) = cds_db::game_challenge::find::<GameChallenge>(
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
