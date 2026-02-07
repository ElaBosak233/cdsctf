mod challenge_id;

use std::sync::Arc;

use axum::{Router, extract::State, http::StatusCode};
use cds_db::{Challenge, challenge::FindChallengeOptions, sea_orm::ActiveValue::Set};
use serde::{Deserialize, Serialize};

use crate::{
    extract::{Json, Query},
    traits::{AppState, WebError, WebResponse},
};

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", axum::routing::get(get_challenges))
        .route("/", axum::routing::post(create_challenge))
        .nest("/{challenge_id}", challenge_id::router())
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetChallengeRequest {
    pub id: Option<i64>,
    pub title: Option<String>,
    pub category: Option<i32>,
    pub tag: Option<String>,
    pub public: Option<bool>,
    pub has_instance: Option<bool>,
    pub page: Option<u64>,
    pub size: Option<u64>,
    pub sorts: Option<String>,
}

pub async fn get_challenges(
    State(s): State<Arc<AppState>>,

    Query(params): Query<GetChallengeRequest>,
) -> Result<WebResponse<Vec<Challenge>>, WebError> {
    let page = params.page.unwrap_or(1);
    let size = params.size.unwrap_or(10).min(100);

    let (challenges, total) = cds_db::challenge::find(
        &s.db.conn,
        FindChallengeOptions {
            id: params.id,
            title: params.title,
            category: params.category,
            tag: params.tag,
            public: params.public,
            has_instance: params.has_instance,
            sorts: params.sorts,
            page: Some(page),
            size: Some(size),
        },
    )
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
    pub public: Option<bool>,
    pub has_instance: Option<bool>,
    pub has_attachment: Option<bool>,
    pub instance: Option<cds_db::challenge::Instance>,
    pub checker: Option<String>,
}

pub async fn create_challenge(
    State(s): State<Arc<AppState>>,

    Json(body): Json<CreateChallengeRequest>,
) -> Result<WebResponse<Challenge>, WebError> {
    let challenge = cds_db::challenge::create(
        &s.db.conn,
        cds_db::challenge::ActiveModel {
            title: Set(body.title),
            description: Set(body.description),
            category: Set(body.category),
            tags: Set(body.tags.unwrap_or(vec![])),
            public: Set(body.public.unwrap_or(false)),
            has_instance: Set(body.has_instance.unwrap_or(false)),
            has_attachment: Set(body.has_attachment.unwrap_or(false)),
            has_writeup: Set(false),
            instance: Set(body.instance),
            checker: Set(body.checker),
            ..Default::default()
        },
    )
    .await?;

    Ok(WebResponse {
        data: Some(challenge),
        ..Default::default()
    })
}
