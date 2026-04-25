//! HTTP routing for `challenge_id` — Axum router wiring and OpenAPI route
//! registration.

/// Defines the `attachment` submodule (see sibling `*.rs` files).
mod attachment;

/// Defines the `checker` submodule (see sibling `*.rs` files).
mod checker;

/// Defines the `writeup` submodule (see sibling `*.rs` files).
mod writeup;

use std::sync::Arc;

use axum::{Json, Router, extract::State};
use cds_db::{
    Challenge,
    sea_orm::{
        ActiveValue::{Set, Unchanged},
        NotSet,
    },
};
use serde::{Deserialize, Serialize};
use utoipa_axum::{
    router::{OpenApiRouter, UtoipaMethodRouterExt},
    routes,
};
use validator::Validate;

use super::AdminChallengeResponse;
use crate::{
    extract::{Path, VJson},
    traits::{AppState, EmptyJson, WebError},
};

/// Builds the Axum router fragment for this module.

pub fn router(state: Arc<AppState>) -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::from(Router::new().with_state(state.clone()))
        .routes(routes!(get_challenge).with_state(state.clone()))
        .routes(routes!(update_challenge).with_state(state.clone()))
        .routes(routes!(delete_challenge).with_state(state.clone()))
        .routes(routes!(update_challenge_instance).with_state(state.clone()))
        .nest("/checker", checker::router(state.clone()))
        .nest("/writeup", writeup::router(state.clone()))
        .nest("/attachments", attachment::router(state.clone()))
}

/// Returns challenge.
#[utoipa::path(
    get,
    path = "/",
    tag = "admin-challenge",
    params(
        ("challenge_id" = i64, Path, description = "Challenge id"),
    ),
    responses(
        (status = 200, description = "Challenge", body = AdminChallengeResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
#[tracing::instrument(skip_all, fields(handler = "get_challenge"))]
pub async fn get_challenge(
    State(s): State<Arc<AppState>>,
    Path(challenge_id): Path<i64>,
) -> Result<Json<AdminChallengeResponse>, WebError> {
    let challenge = crate::util::loader::prepare_challenge(&s.db.conn, challenge_id).await?;
    Ok(Json(AdminChallengeResponse { challenge }))
}

#[derive(Debug, Serialize, Deserialize, Validate, utoipa::ToSchema)]
pub struct UpdateChallengeRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub category: Option<i32>,
    pub tags: Option<Vec<String>>,
    pub public: Option<bool>,
    pub has_instance: Option<bool>,
    pub has_attachment: Option<bool>,
    pub has_writeup: Option<bool>,
}

/// Updates challenge.
#[utoipa::path(
    put,
    path = "/",
    tag = "admin-challenge",
    params(
        ("challenge_id" = i64, Path, description = "Challenge id"),
    ),
    request_body = UpdateChallengeRequest,
    responses(
        (status = 200, description = "Updated", body = AdminChallengeResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
#[tracing::instrument(skip_all, fields(handler = "update_challenge"))]
pub async fn update_challenge(
    State(s): State<Arc<AppState>>,
    Path(challenge_id): Path<i64>,
    VJson(body): VJson<UpdateChallengeRequest>,
) -> Result<Json<AdminChallengeResponse>, WebError> {
    let challenge = crate::util::loader::prepare_challenge(&s.db.conn, challenge_id).await?;

    let challenge = cds_db::challenge::update(
        &s.db.conn,
        cds_db::challenge::ActiveModel {
            id: Unchanged(challenge.id),
            title: body.title.map_or(NotSet, Set),
            description: body.description.map_or(NotSet, Set),
            tags: body.tags.map_or(NotSet, Set),
            category: body.category.map_or(NotSet, Set),
            public: body.public.map_or(NotSet, Set),
            has_instance: body.has_instance.map_or(NotSet, Set),
            has_attachment: body.has_attachment.map_or(NotSet, Set),
            has_writeup: body.has_writeup.map_or(NotSet, Set),
            ..Default::default()
        },
    )
    .await?;

    Ok(Json(AdminChallengeResponse { challenge }))
}

/// Deletes challenge.
#[utoipa::path(
    delete,
    path = "/",
    tag = "admin-challenge",
    params(
        ("challenge_id" = i64, Path, description = "Challenge id"),
    ),
    responses(
        (status = 200, description = "Deleted", body = EmptyJson),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
#[tracing::instrument(skip_all, fields(handler = "delete_challenge"))]
pub async fn delete_challenge(
    State(s): State<Arc<AppState>>,
    Path(challenge_id): Path<i64>,
) -> Result<Json<EmptyJson>, WebError> {
    let challenge = crate::util::loader::prepare_challenge(&s.db.conn, challenge_id).await?;
    cds_db::challenge::delete(&s.db.conn, challenge.id).await?;
    Ok(Json(EmptyJson::default()))
}

#[derive(Debug, Serialize, Deserialize, Validate, utoipa::ToSchema)]
pub struct UpdateChallengeInstanceRequest {
    pub instance: Option<cds_db::challenge::Instance>,
}

/// Updates challenge instance.
#[utoipa::path(
    put,
    path = "/instance",
    tag = "admin-challenge",
    params(
        ("challenge_id" = i64, Path, description = "Challenge id"),
    ),
    request_body = UpdateChallengeInstanceRequest,
    responses(
        (status = 200, description = "Updated", body = EmptyJson),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
#[tracing::instrument(skip_all, fields(handler = "update_challenge_instance"))]
pub async fn update_challenge_instance(
    State(s): State<Arc<AppState>>,
    Path(challenge_id): Path<i64>,
    VJson(body): VJson<UpdateChallengeInstanceRequest>,
) -> Result<Json<EmptyJson>, WebError> {
    let _ = crate::util::loader::prepare_challenge(&s.db.conn, challenge_id).await?;

    let _ = cds_db::challenge::update::<Challenge>(
        &s.db.conn,
        cds_db::challenge::ActiveModel {
            id: Unchanged(challenge_id),
            instance: body.instance.map_or(NotSet, |v| Set(Some(v))),
            ..Default::default()
        },
    )
    .await?;

    Ok(Json(EmptyJson::default()))
}
