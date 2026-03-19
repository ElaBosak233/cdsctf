mod attachment;
mod checker;
mod writeup;

use std::sync::Arc;

use axum::{Router, extract::State};
use cds_db::{
    Challenge,
    sea_orm::{
        ActiveValue::{Set, Unchanged},
        NotSet,
    },
};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    extract::{Path, VJson},
    traits::{AppState, WebError, WebResponse},
};

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", axum::routing::get(get_challenge))
        .route("/", axum::routing::put(update_challenge))
        .route("/", axum::routing::delete(delete_challenge))
        .route("/instance", axum::routing::put(update_challenge_instance))
        .nest("/checker", checker::router())
        .nest("/writeup", writeup::router())
        .nest("/attachments", attachment::router())
}

pub async fn get_challenge(
    State(s): State<Arc<AppState>>,

    Path(challenge_id): Path<i64>,
) -> Result<WebResponse<Challenge>, WebError> {
    let challenge = crate::util::loader::prepare_challenge(&s.db.conn, challenge_id).await?;

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
    pub public: Option<bool>,
    pub has_instance: Option<bool>,
    pub has_attachment: Option<bool>,
    pub has_writeup: Option<bool>,
}

pub async fn update_challenge(
    State(s): State<Arc<AppState>>,

    Path(challenge_id): Path<i64>,
    VJson(body): VJson<UpdateChallengeRequest>,
) -> Result<WebResponse<Challenge>, WebError> {
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

    Ok(WebResponse {
        data: challenge,
        ..Default::default()
    })
}

pub async fn delete_challenge(
    State(s): State<Arc<AppState>>,

    Path(challenge_id): Path<i64>,
) -> Result<WebResponse<()>, WebError> {
    let challenge = crate::util::loader::prepare_challenge(&s.db.conn, challenge_id).await?;

    cds_db::challenge::delete(&s.db.conn, challenge.id).await?;

    Ok(WebResponse::default())
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateChallengeInstanceRequest {
    pub instance: Option<cds_db::challenge::Instance>,
}

pub async fn update_challenge_instance(
    State(s): State<Arc<AppState>>,

    Path(challenge_id): Path<i64>,
    VJson(body): VJson<UpdateChallengeInstanceRequest>,
) -> Result<WebResponse<()>, WebError> {
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

    Ok(WebResponse::default())
}
