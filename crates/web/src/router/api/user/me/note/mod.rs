//! HTTP routing for `note` — Axum router wiring and OpenAPI route registration.

use std::sync::Arc;

use axum::{Json, Router, extract::State};
use cds_db::{
    note::{ActiveModel, FindNotesOptions, Note},
    sea_orm::{Set, Unchanged},
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
        .routes(routes!(get_my_note).with_state(state.clone()))
        .routes(routes!(save_my_note).with_state(state.clone()))
}

#[derive(Clone, Debug, Serialize, Deserialize, utoipa::ToSchema, utoipa::IntoParams)]
#[into_params(parameter_in = Query)]
pub struct GetMyNoteRequest {
    pub challenge_id: Option<i64>,
    pub size: Option<u64>,
    pub page: Option<u64>,
    pub sorts: Option<String>,
}

#[derive(Clone, Debug, Serialize, utoipa::ToSchema)]
pub struct MyNotesListResponse {
    pub notes: Vec<Note>,
    pub total: u64,
}

#[utoipa::path(
    get,
    path = "/",
    tag = "user",
    params(GetMyNoteRequest),
    responses(
        (status = 200, description = "My notes", body = MyNotesListResponse),
        (status = 401, description = "Unauthorized", body = crate::traits::ErrorResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]

/// Returns my note.
pub async fn get_my_note(
    State(s): State<Arc<AppState>>,
    Extension(ap): Extension<AuthPrincipal>,
    Query(params): Query<GetMyNoteRequest>,
) -> Result<Json<MyNotesListResponse>, WebError> {
    let operator = ap.operator.ok_or(WebError::Unauthorized(json!("")))?;

    let (notes, total) = cds_db::note::find(
        &s.db.conn,
        FindNotesOptions {
            user_id: Some(operator.id),
            challenge_id: params.challenge_id,
            size: params.size,
            page: params.page,
            sorts: params.sorts,
            ..Default::default()
        },
    )
    .await?;

    Ok(Json(MyNotesListResponse {
        notes,
        total,
    }))
}

#[derive(Clone, Debug, Serialize, Deserialize, utoipa::ToSchema)]
pub struct SaveMyNoteRequest {
    pub content: String,
    pub public: bool,
    pub challenge_id: i64,
}

#[derive(Clone, Debug, Serialize, utoipa::ToSchema)]
pub struct NoteResponse {
    pub note: Note,
}

#[utoipa::path(
    post,
    path = "/",
    tag = "user",
    request_body = SaveMyNoteRequest,
    responses(
        (status = 200, description = "Note saved", body = NoteResponse),
        (status = 401, description = "Unauthorized", body = crate::traits::ErrorResponse),
        (status = 403, description = "Forbidden", body = crate::traits::ErrorResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]

/// Persists the authenticated user's personal note blob.
pub async fn save_my_note(
    State(s): State<Arc<AppState>>,
    Extension(ap): Extension<AuthPrincipal>,
    ReqJson(body): ReqJson<SaveMyNoteRequest>,
) -> Result<Json<NoteResponse>, WebError> {
    let operator = ap.operator.ok_or(WebError::Unauthorized(json!("")))?;

    let challenge = crate::util::loader::prepare_challenge(&s.db.conn, body.challenge_id).await?;

    if !challenge.public {
        return Err(WebError::Forbidden(json!("")));
    }

    let note = match cds_db::note::find_by_user_id_and_challenge_id::<Note>(
        &s.db.conn,
        operator.id,
        body.challenge_id,
    )
    .await?
    {
        Some(note) => {
            cds_db::note::update::<Note>(
                &s.db.conn,
                ActiveModel {
                    id: Unchanged(note.id),
                    content: Set(body.content),
                    public: Set(body.public),
                    challenge_id: Unchanged(note.challenge_id),
                    user_id: Unchanged(note.user_id),
                    ..Default::default()
                },
            )
            .await?
        }
        None => {
            cds_db::note::create::<Note>(
                &s.db.conn,
                ActiveModel {
                    content: Set(body.content),
                    public: Set(body.public),
                    challenge_id: Set(body.challenge_id),
                    user_id: Set(operator.id),
                    ..Default::default()
                },
            )
            .await?
        }
    };

    Ok(Json(NoteResponse { note }))
}
