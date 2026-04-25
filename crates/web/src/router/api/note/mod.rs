//! HTTP routing for `note` — Axum router wiring and OpenAPI route registration.

use std::sync::Arc;

use axum::{Json, Router, extract::State};
use cds_db::note::{FindNotesOptions, Note};
use serde::{Deserialize, Serialize};
use serde_json::json;
use utoipa_axum::{
    router::{OpenApiRouter, UtoipaMethodRouterExt},
    routes,
};

use crate::{
    extract::{Extension, Query},
    traits::{AppState, AuthPrincipal, WebError},
};

/// Builds the Axum router fragment for this module.

pub fn router(state: Arc<AppState>) -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::from(Router::new().with_state(state.clone()))
        .routes(routes!(list_notes).with_state(state.clone()))
}

#[derive(Clone, Debug, Serialize, Deserialize, utoipa::ToSchema, utoipa::IntoParams)]
#[into_params(parameter_in = Query)]
pub struct ListNotesRequest {
    pub id: Option<i64>,
    pub user_id: Option<i64>,
    pub challenge_id: Option<i64>,
    pub size: Option<u64>,
    pub page: Option<u64>,
    pub sorts: Option<String>,
}

#[derive(Clone, Debug, Serialize, utoipa::ToSchema)]
pub struct ListNotesResponse {
    pub notes: Vec<Note>,
    pub total: u64,
}

/// Lists public notes (collection).
#[utoipa::path(
    get,
    path = "/",
    tag = "note",
    params(ListNotesRequest),
    responses(
        (status = 200, description = "Public notes", body = ListNotesResponse),
        (status = 401, description = "Unauthorized", body = crate::traits::ErrorResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
#[tracing::instrument(skip_all, fields(handler = "list_notes"))]
pub async fn list_notes(
    State(s): State<Arc<AppState>>,

    Extension(ap): Extension<AuthPrincipal>,
    Query(params): Query<ListNotesRequest>,
) -> Result<Json<ListNotesResponse>, WebError> {
    let _ = ap.operator.ok_or(WebError::Unauthorized(json!("")))?;

    let (notes, total) = cds_db::note::find(
        &s.db.conn,
        FindNotesOptions {
            id: params.id,
            user_id: params.user_id,
            challenge_id: params.challenge_id,
            public: Some(true),
            size: params.size,
            page: params.page,
            sorts: params.sorts,
            ..Default::default()
        },
    )
    .await?;

    Ok(Json(ListNotesResponse { notes, total }))
}
