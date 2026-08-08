//! HTTP routing for `challenge` — Axum router wiring and OpenAPI route
//! registration.

/// Defines the `challenge_id` submodule (see sibling `*.rs` files).
mod challenge_id;

use std::sync::Arc;

use axum::{Json, Router, extract::State, http::StatusCode};
use cds_db::{
    ChallengeDetail,
    challenge::FindChallengeOptions,
    sea_orm::{ActiveValue::Set, TransactionTrait},
};
use cds_media::{Media, SaveIfAbsent};
use ring::rand::{SecureRandom, SystemRandom};
use serde::{Deserialize, Serialize};
use tracing::info;
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
    pub challenges: Vec<ChallengeDetail>,
    pub total: u64,
}

/// Returns challenges.
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
#[tracing::instrument(skip_all, fields(handler = "get_challenges"))]
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

    Ok(Json(AdminChallengesListResponse { challenges, total }))
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
    pub challenge: ChallengeDetail,
}

/// Creates challenge.
#[utoipa::path(
    post,
    path = "/",
    tag = "admin-challenge",
    request_body = CreateChallengeRequest,
    responses(
        (status = 201, description = "Created", body = AdminChallengeResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
#[tracing::instrument(skip_all, fields(handler = "create_challenge"))]
pub async fn create_challenge(
    State(s): State<Arc<AppState>>,
    ReqJson(body): ReqJson<CreateChallengeRequest>,
) -> Result<(StatusCode, Json<AdminChallengeResponse>), WebError> {
    let challenge = create_challenge_with_key(
        &s.db.conn,
        &s.media,
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

    info!(
        challenge_id = challenge.id,
        title = %challenge.title,
        category = challenge.category,
        public = challenge.public,
        has_instance = challenge.has_instance,
        "admin created challenge"
    );

    Ok((
        StatusCode::CREATED,
        Json(AdminChallengeResponse { challenge }),
    ))
}

async fn create_challenge_with_key(
    conn: &cds_db::sea_orm::DatabaseConnection,
    media: &Media,
    model: cds_db::challenge::ActiveModel,
) -> Result<ChallengeDetail, WebError> {
    let transaction = conn.begin().await.map_err(cds_db::DbError::from)?;
    let challenge = cds_db::challenge::create::<ChallengeDetail>(&transaction, model).await?;

    let mut key = [0_u8; 64];
    SystemRandom::new().fill(&mut key).map_err(|_| {
        WebError::InternalServerError(serde_json::json!("checker_key_generation_failed"))
    })?;
    let key = hex::encode(key);
    let path = format!("challenges/{}", challenge.id);
    let key_result = media
        .save_if_absent(path.clone(), ".key".to_owned(), key.into_bytes())
        .await;
    match key_result {
        Ok(SaveIfAbsent::Created) => {}
        Ok(SaveIfAbsent::AlreadyExists) => {
            return Err(WebError::InternalServerError(serde_json::json!(
                "checker_key_already_exists"
            )));
        }
        Err(error) => {
            let _ = media.delete(path.clone(), ".key".to_owned()).await;
            return Err(error.into());
        }
    }
    if let Err(error) = transaction.commit().await {
        // A commit error has an unknown outcome. Keep the object because the
        // transaction may have committed and made the challenge visible.
        return Err(cds_db::DbError::from(error).into());
    }

    Ok(challenge)
}
