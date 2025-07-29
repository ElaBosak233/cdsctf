mod attachment;

use axum::{Router, http::StatusCode};
use cds_checker::traits::CheckerError;
use cds_db::{
    Challenge,
    sea_orm::{
        ActiveValue::{Set, Unchanged},
        NotSet,
    },
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use validator::Validate;

use crate::{
    extract::{Path, VJson},
    traits::{WebError, WebResponse},
};

pub fn router() -> Router {
    Router::new()
        .route("/", axum::routing::get(get_challenge))
        .route("/", axum::routing::put(update_challenge))
        .route("/", axum::routing::delete(delete_challenge))
        .route("/env", axum::routing::put(update_challenge_env))
        .route("/checker", axum::routing::put(update_challenge_checker))
        .nest("/attachments", attachment::router())
}

pub async fn get_challenge(
    Path(challenge_id): Path<uuid::Uuid>,
) -> Result<WebResponse<Challenge>, WebError> {
    let challenge = crate::util::loader::prepare_challenge(challenge_id).await?;

    Ok(WebResponse {
        data: Some(challenge),
        ..Default::default()
    })
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateChallengeRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub category: Option<i32>,
    pub tags: Option<Vec<String>>,
    pub is_public: Option<bool>,
    pub is_dynamic: Option<bool>,
    pub has_attachment: Option<bool>,
}

pub async fn update_challenge(
    Path(challenge_id): Path<uuid::Uuid>,
    VJson(body): VJson<UpdateChallengeRequest>,
) -> Result<WebResponse<Challenge>, WebError> {
    let challenge = crate::util::loader::prepare_challenge(challenge_id).await?;

    let challenge = cds_db::challenge::update(cds_db::challenge::ActiveModel {
        id: Unchanged(challenge.id),
        title: body.title.map_or(NotSet, Set),
        description: body.description.map_or(NotSet, Set),
        tags: body.tags.map_or(NotSet, Set),
        category: body.category.map_or(NotSet, Set),
        is_public: body.is_public.map_or(NotSet, Set),
        is_dynamic: body.is_dynamic.map_or(NotSet, Set),
        has_attachment: body.has_attachment.map_or(NotSet, Set),
        ..Default::default()
    })
    .await?;

    Ok(WebResponse {
        code: StatusCode::OK,
        data: challenge,
        ..Default::default()
    })
}

pub async fn delete_challenge(
    Path(challenge_id): Path<uuid::Uuid>,
) -> Result<WebResponse<()>, WebError> {
    let challenge = crate::util::loader::prepare_challenge(challenge_id).await?;

    cds_db::challenge::delete(challenge.id).await?;

    Ok(WebResponse {
        code: StatusCode::OK,
        ..Default::default()
    })
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateChallengeEnvRequest {
    pub env: Option<cds_db::challenge::Env>,
}

pub async fn update_challenge_env(
    Path(challenge_id): Path<uuid::Uuid>,
    VJson(body): VJson<UpdateChallengeEnvRequest>,
) -> Result<WebResponse<()>, WebError> {
    let _ = crate::util::loader::prepare_challenge(challenge_id).await?;

    let _ = cds_db::challenge::update::<Challenge>(cds_db::challenge::ActiveModel {
        id: Unchanged(challenge_id),
        env: body.env.map_or(NotSet, |v| Set(Some(v))),
        ..Default::default()
    })
    .await?;

    Ok(WebResponse {
        code: StatusCode::OK,
        ..Default::default()
    })
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateChallengeCheckerRequest {
    pub checker: Option<String>,
}

pub async fn update_challenge_checker(
    Path(challenge_id): Path<uuid::Uuid>,
    VJson(body): VJson<UpdateChallengeCheckerRequest>,
) -> Result<WebResponse<()>, WebError> {
    let _ = crate::util::loader::prepare_challenge(challenge_id).await?;

    let challenge = cds_db::challenge::update::<Challenge>(cds_db::challenge::ActiveModel {
        id: Unchanged(challenge_id),
        checker: body.checker.map_or(NotSet, |v| Set(Some(v))),
        ..Default::default()
    })
    .await?;

    let lint = cds_checker::lint(&challenge).await;
    let msg = if let Err(lint) = lint {
        match lint {
            CheckerError::CompileError(diagnostics) => Some(diagnostics),
            err => Some(err.to_string()),
        }
    } else {
        None
    };

    Ok(WebResponse {
        code: StatusCode::OK,
        msg: msg.map(|msg| json!(msg)),
        ..Default::default()
    })
}
