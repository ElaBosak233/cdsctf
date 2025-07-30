mod challenge_id;

use axum::{Router, http::StatusCode};
use cds_db::{Challenge, challenge::FindChallengeOptions, sea_orm::ActiveValue::Set};
use serde::{Deserialize, Serialize};

use crate::{
    extract::{Json, Query},
    traits::{WebError, WebResponse},
};

pub fn router() -> Router {
    Router::new()
        .route("/", axum::routing::get(get_challenges))
        .route("/", axum::routing::post(create_challenge))
        .nest("/{challenge_id}", challenge_id::router())
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetChallengeRequest {
    pub id: Option<uuid::Uuid>,
    pub title: Option<String>,
    pub category: Option<i32>,
    pub tag: Option<String>,
    pub is_public: Option<bool>,
    pub is_dynamic: Option<bool>,
    pub page: Option<u64>,
    pub size: Option<u64>,
    pub sorts: Option<String>,
}

pub async fn get_challenges(
    Query(params): Query<GetChallengeRequest>,
) -> Result<WebResponse<Vec<Challenge>>, WebError> {
    let page = params.page.unwrap_or(1);
    let size = params.size.unwrap_or(10).min(100);

    let (challenges, total) = cds_db::challenge::find::<Challenge>(FindChallengeOptions {
        id: params.id,
        title: params.title,
        category: params.category,
        tag: params.tag,
        is_public: params.is_public,
        is_dynamic: params.is_dynamic,
        sorts: params.sorts,
        page: Some(page),
        size: Some(size),
    })
    .await?;

    Ok(WebResponse {
        code: StatusCode::OK,
        data: Some(challenges),
        total: Some(total),
        ..Default::default()
    })
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateChallengeRequest {
    pub title: String,
    pub description: String,
    pub category: i32,
    pub tags: Option<Vec<String>>,
    pub is_public: Option<bool>,
    pub is_dynamic: Option<bool>,
    pub has_attachment: Option<bool>,
    pub image_name: Option<String>,
    pub env: Option<cds_db::challenge::Env>,
    pub checker: Option<String>,
}

pub async fn create_challenge(
    Json(body): Json<CreateChallengeRequest>,
) -> Result<WebResponse<Challenge>, WebError> {
    let challenge = cds_db::challenge::create::<Challenge>(cds_db::challenge::ActiveModel {
        title: Set(body.title),
        description: Set(body.description),
        category: Set(body.category),
        tags: Set(body.tags.unwrap_or(vec![])),
        is_public: Set(body.is_public.unwrap_or(false)),
        is_dynamic: Set(body.is_dynamic.unwrap_or(false)),
        has_attachment: Set(body.has_attachment.unwrap_or(false)),
        env: Set(body.env),
        checker: Set(body.checker),
        ..Default::default()
    })
    .await?;

    Ok(WebResponse {
        code: StatusCode::OK,
        data: Some(challenge),
        ..Default::default()
    })
}
