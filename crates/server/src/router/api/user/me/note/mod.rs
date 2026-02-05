use std::sync::Arc;

use axum::{Router, extract::State};
use cds_db::{
    note::{ActiveModel, FindNotesOptions, Note},
    sea_orm::{Set, Unchanged},
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    extract::{Extension, Json, Query},
    traits::{AppState, AuthPrincipal, WebError, WebResponse},
};

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", axum::routing::get(get_my_note))
        .route("/", axum::routing::post(save_my_note))
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetMyNoteRequest {
    pub challenge_id: Option<i64>,
    pub size: Option<u64>,
    pub page: Option<u64>,
    pub sorts: Option<String>,
}

pub async fn get_my_note(
    State(s): State<Arc<AppState>>,

    Extension(ap): Extension<AuthPrincipal>,
    Query(params): Query<GetMyNoteRequest>,
) -> Result<WebResponse<Vec<Note>>, WebError> {
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

    Ok(WebResponse {
        data: Some(notes),
        total: Some(total),
        ..Default::default()
    })
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SaveMyNoteRequest {
    pub content: String,
    pub public: bool,
    pub challenge_id: i64,
}

pub async fn save_my_note(
    State(s): State<Arc<AppState>>,

    Extension(ap): Extension<AuthPrincipal>,
    Json(body): Json<SaveMyNoteRequest>,
) -> Result<WebResponse<Note>, WebError> {
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

    Ok(WebResponse {
        data: Some(note),
        ..Default::default()
    })
}
