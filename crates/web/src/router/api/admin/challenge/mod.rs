//! HTTP routing for `challenge` — Axum router wiring and OpenAPI route
//! registration.

/// Defines the `challenge_id` submodule (see sibling `*.rs` files).
mod challenge_id;

use std::sync::Arc;

use axum::{Json, Router, extract::State};
use cds_db::{Challenge, challenge::FindChallengeOptions, sea_orm::ActiveValue::Set};
use serde::{Deserialize, Serialize};
use utoipa_axum::{
    router::{OpenApiRouter, UtoipaMethodRouterExt},
    routes,
};

use crate::{
    extract::{Json as ReqJson, Query},
    traits::{AppState, WebError},
};

/// Builds the Axum router fragment for this module.

pub fn router(state: Arc<AppState>) -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::from(Router::new().with_state(state.clone()))
        .routes(routes!(get_challenges).with_state(state.clone()))
        .routes(routes!(create_challenge).with_state(state.clone()))
        .nest("/{challenge_id}", challenge_id::router(state.clone()))
}

#[derive(Clone, Debug, Serialize, Deserialize, utoipa::ToSchema, utoipa::IntoParams)]
#[into_params(parameter_in = Query)]
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

#[derive(Clone, Debug, Serialize, utoipa::ToSchema)]
pub struct AdminChallengesListResponse {
    pub challenges: Vec<Challenge>,
    pub total: u64,
}

#[utoipa::path(
    get,
    path = "/",
    tag = "admin-challenge",
    params(GetChallengeRequest),
    responses(
        (status = 200, description = "Challenges", body = AdminChallengesListResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]

/// Returns challenges.
pub async fn get_challenges(
    State(s): State<Arc<AppState>>,
    Query(params): Query<GetChallengeRequest>,
) -> Result<Json<AdminChallengesListResponse>, WebError> {
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

    Ok(Json(AdminChallengesListResponse {
        challenges,
        total,
    }))
}

#[derive(Debug, Serialize, Deserialize, utoipa::ToSchema)]
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

#[derive(Clone, Debug, Serialize, utoipa::ToSchema)]
pub struct AdminChallengeResponse {
    pub challenge: Challenge,
}

#[utoipa::path(
    post,
    path = "/",
    tag = "admin-challenge",
    request_body = CreateChallengeRequest,
    responses(
        (status = 200, description = "Created", body = AdminChallengeResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]

/// Creates challenge.
pub async fn create_challenge(
    State(s): State<Arc<AppState>>,
    ReqJson(body): ReqJson<CreateChallengeRequest>,
) -> Result<Json<AdminChallengeResponse>, WebError> {
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

    Ok(Json(AdminChallengeResponse { challenge }))
}
