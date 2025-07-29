mod challenge_id;

use std::collections::HashMap;

use axum::Router;
use cds_db::{
    ChallengeMini, GameChallenge, Submission, challenge::FindChallengeOptions,
    game_challenge::FindGameChallengeOptions,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    extract::{Extension, Json, Query},
    traits::{AuthPrincipal, WebError, WebResponse},
};

pub fn router() -> Router {
    Router::new()
        .route("/playground", axum::routing::get(get_challenge))
        .route("/status", axum::routing::post(get_challenge_status))
        .nest("/{challenge_id}", challenge_id::router())
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetChallengeRequest {
    pub id: Option<uuid::Uuid>,
    pub title: Option<String>,
    pub category: Option<i32>,
    pub tag: Option<String>,
    pub page: Option<u64>,
    pub size: Option<u64>,
    pub sorts: Option<String>,
}

pub async fn get_challenge(
    Extension(ext): Extension<AuthPrincipal>,
    Query(params): Query<GetChallengeRequest>,
) -> Result<WebResponse<Vec<ChallengeMini>>, WebError> {
    let _ = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;

    let page = params.page.unwrap_or(1);
    let size = params.size.unwrap_or(10).min(100);

    let (challenges, total) = cds_db::challenge::find::<ChallengeMini>(FindChallengeOptions {
        id: params.id,
        title: params.title,
        category: params.category,
        tag: params.tag,
        is_public: Some(true),
        sorts: params.sorts,
        page: Some(page),
        size: Some(size),
        ..Default::default()
    })
    .await?;

    Ok(WebResponse {
        data: Some(challenges),
        total: Some(total),
        ..Default::default()
    })
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetChallengeStatusRequest {
    pub challenge_ids: Vec<uuid::Uuid>,
    pub user_id: Option<i64>,
    pub team_id: Option<i64>,
    pub game_id: Option<i64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChallengeStatusResponse {
    pub is_solved: bool,
    pub solved_times: i64,
    pub pts: i64,
    pub bloods: Vec<Submission>,
}

pub async fn get_challenge_status(
    Extension(ext): Extension<AuthPrincipal>,
    Json(body): Json<GetChallengeStatusRequest>,
) -> Result<WebResponse<HashMap<uuid::Uuid, ChallengeStatusResponse>>, WebError> {
    let _ = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;

    if body.user_id.is_some() && (body.team_id.is_some() || body.game_id.is_some()) {
        return Err(WebError::BadRequest(json!("either_user_or_team")));
    }

    let mut submissions =
        cds_db::submission::find_correct_by_challenge_ids_and_optional_team_game::<Submission>(
            body.challenge_ids.clone(),
            body.team_id,
            body.game_id,
        )
        .await?;

    let mut result: HashMap<uuid::Uuid, ChallengeStatusResponse> = HashMap::new();

    for challenge_id in body.challenge_ids.iter() {
        result.insert(
            *challenge_id,
            ChallengeStatusResponse {
                is_solved: false,
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
                status_response.is_solved = true;
            }

            status_response.solved_times += 1;

            if status_response.bloods.len() < 3 {
                status_response.bloods.push(submission.clone());
            }
        }
    }

    if let Some(game_id) = body.game_id {
        let (game_challenges, _) =
            cds_db::game_challenge::find::<GameChallenge>(FindGameChallengeOptions {
                game_id: Some(game_id),
                ..Default::default()
            })
            .await?;

        for game_challenge in game_challenges {
            if let Some(status_response) = result.get_mut(&game_challenge.challenge_id) {
                status_response.pts = game_challenge.pts;
            }
        }
    }

    Ok(WebResponse {
        data: Some(result),
        ..Default::default()
    })
}
