//! HTTP handlers for `writeup` within the `challenge_id` API segment.

use std::sync::Arc;

use axum::{Json, Router, extract::State};
use cds_db::sea_orm::{Set, Unchanged};
use serde::{Deserialize, Serialize};
use utoipa_axum::{
    router::{OpenApiRouter, UtoipaMethodRouterExt},
    routes,
};
use validator::Validate;

use super::AdminChallengeResponse;
use crate::{
    extract::{Path, VJson},
    traits::{AppState, WebError},
};

/// Builds the Axum router fragment for this module.

pub fn router(state: Arc<AppState>) -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::from(Router::new().with_state(state.clone()))
        .routes(routes!(update_writeup).with_state(state.clone()))
}

#[derive(Debug, Serialize, Deserialize, Validate, utoipa::ToSchema)]
pub struct UpdateWriteupRequest {
    pub writeup: String,
}

/// Updates writeup.
#[utoipa::path(
    put,
    path = "/",
    tag = "admin-challenge",
    params(
        ("challenge_id" = i64, Path, description = "Challenge id"),
    ),
    request_body = UpdateWriteupRequest,
    responses(
        (status = 200, description = "Updated writeup", body = AdminChallengeResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
#[tracing::instrument(skip_all, fields(handler = "update_writeup"))]
pub async fn update_writeup(
    State(s): State<Arc<AppState>>,
    Path(challenge_id): Path<i64>,
    VJson(body): VJson<UpdateWriteupRequest>,
) -> Result<Json<AdminChallengeResponse>, WebError> {
    let challenge = crate::util::loader::prepare_challenge(&s.db.conn, challenge_id).await?;

    let challenge = cds_db::challenge::update(
        &s.db.conn,
        cds_db::challenge::ActiveModel {
            id: Unchanged(challenge.id),
            writeup: Set(Some(body.writeup)),
            ..Default::default()
        },
    )
    .await?;

    Ok(Json(AdminChallengeResponse { challenge }))
}
