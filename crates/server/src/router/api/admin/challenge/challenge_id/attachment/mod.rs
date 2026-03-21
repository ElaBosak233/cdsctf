mod filename;

use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{DefaultBodyLimit, Multipart, State},
};
use serde_json::json;
use utoipa_axum::{
    router::{OpenApiRouter, UtoipaMethodRouterExt},
    routes,
};

use crate::{
    extract::Path,
    model::Metadata,
    traits::{AppState, EmptySuccess, WebError},
};

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", axum::routing::get(get_challenge_attachment))
        .route(
            "/",
            axum::routing::post(save_challenge_attachment)
                .layer(DefaultBodyLimit::max(512 * 1024 * 1024 /* MB */)),
        )
        .nest("/{filename}", filename::router())
}

pub fn openapi_router(state: Arc<AppState>) -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::from(Router::new().with_state(state.clone()))
        .routes(routes!(get_challenge_attachment).with_state(state.clone()))
        .routes(routes!(save_challenge_attachment).with_state(state.clone()))
        .nest(
            "/{filename}",
            filename::openapi_router(state.clone()),
        )
}

#[derive(Clone, Debug, serde::Serialize, utoipa::ToSchema)]
pub struct AdminChallengeAttachmentsListResponse {
    pub items: Vec<Metadata>,
    pub total: u64,
}

#[utoipa::path(
    get,
    path = "/",
    tag = "admin-challenge",
    params(
        ("challenge_id" = i64, Path, description = "Challenge id"),
    ),
    responses(
        (status = 200, description = "Attachments", body = AdminChallengeAttachmentsListResponse),
        (status = 404, description = "Not found", body = crate::traits::ApiJsonError),
        (status = 500, description = "Server error", body = crate::traits::ApiJsonError),
    )
)]
pub async fn get_challenge_attachment(
    State(s): State<Arc<AppState>>,
    Path(challenge_id): Path<i64>,
) -> Result<Json<AdminChallengeAttachmentsListResponse>, WebError> {
    let _ = crate::util::loader::prepare_challenge(&s.db.conn, challenge_id)
        .await?
        .has_attachment
        .then_some(())
        .ok_or_else(|| WebError::NotFound(json!("challenge_has_not_attachment")))?;

    let path = crate::util::media::build_challenge_attachment_path(challenge_id);
    let metadata = s
        .media
        .scan_dir(path.clone())
        .await?
        .into_iter()
        .map(|(filename, size)| Metadata { filename, size })
        .collect::<Vec<Metadata>>();
    let total = metadata.len() as u64;

    Ok(Json(AdminChallengeAttachmentsListResponse { items: metadata, total }))
}

#[utoipa::path(
    post,
    path = "/",
    tag = "admin-challenge",
    params(
        ("challenge_id" = i64, Path, description = "Challenge id"),
    ),
    responses(
        (status = 200, description = "Uploaded", body = EmptySuccess),
        (status = 400, description = "Bad request", body = crate::traits::ApiJsonError),
        (status = 500, description = "Server error", body = crate::traits::ApiJsonError),
    )
)]
pub async fn save_challenge_attachment(
    State(s): State<Arc<AppState>>,
    Path(challenge_id): Path<i64>,
    mut multipart: Multipart,
) -> Result<Json<EmptySuccess>, WebError> {
    let _ = crate::util::loader::prepare_challenge(&s.db.conn, challenge_id).await?;

    let path = crate::util::media::build_challenge_attachment_path(challenge_id);
    let mut filename = String::new();
    let mut data = Vec::<u8>::new();
    while let Some(field) = multipart.next_field().await? {
        if let Some(name) = field.file_name() {
            filename = name.to_string();
            data = match field.bytes().await {
                Ok(bytes) => bytes.to_vec(),
                _ => return Err(WebError::BadRequest(json!("size_too_large"))),
            };
            break;
        }
    }

    s.media
        .save(path, filename, data)
        .await
        .map_err(|_| WebError::InternalServerError(json!("")))?;

    Ok(Json(EmptySuccess::default()))
}
