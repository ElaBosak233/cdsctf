//! HTTP routing for `notice` — Axum router wiring and OpenAPI route
//! registration.

use std::sync::Arc;

use axum::{Json, Router, extract::State, http::StatusCode};
use cds_db::{
    AdminGameNoticeView,
    sea_orm::{
        ActiveValue::{Set, Unchanged},
        NotSet,
    },
};
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::json;
use utoipa_axum::{
    router::{OpenApiRouter, UtoipaMethodRouterExt},
    routes,
};

use crate::{
    extract::{Extension, Json as ReqJson, Path},
    traits::{AppState, AuthPrincipal, EmptyJson, WebError},
};

/// Builds the Axum router fragment for this module.

pub fn router(state: Arc<AppState>) -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::from(Router::new().with_state(state.clone()))
        .routes(routes!(list_game_notices).with_state(state.clone()))
        .routes(routes!(create_game_notice).with_state(state.clone()))
        .routes(routes!(update_game_notice).with_state(state.clone()))
        .routes(routes!(delete_game_notice).with_state(state.clone()))
}

fn deserialize_optional_nullable_i64<'de, D>(
    deserializer: D,
) -> Result<Option<Option<i64>>, D::Error>
where
    D: Deserializer<'de>, {
    Option::<i64>::deserialize(deserializer).map(Some)
}

#[derive(Clone, Debug, Serialize, utoipa::ToSchema)]
pub struct AdminGameNoticesListResponse {
    pub notices: Vec<AdminGameNoticeView>,
    pub total: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize, utoipa::ToSchema)]
pub struct CreateGameNoticeRequest {
    pub title: String,
    pub content: String,
    #[serde(default)]
    pub challenge_id: Option<i64>,
}

#[derive(Clone, Debug, Serialize, Deserialize, utoipa::ToSchema)]
pub struct UpdateGameNoticeRequest {
    pub title: Option<String>,
    pub content: Option<String>,
    #[serde(default, deserialize_with = "deserialize_optional_nullable_i64")]
    #[schema(value_type = Option<i64>)]
    pub challenge_id: Option<Option<i64>>,
}

#[derive(Clone, Debug, Serialize, utoipa::ToSchema)]
pub struct GameNoticeResponse {
    pub notice: AdminGameNoticeView,
}

/// Lists all notices for a game with author and editor metadata.
#[utoipa::path(
    get,
    path = "/",
    tag = "management-game",
    params(
        ("game_id" = i64, Path, description = "Game id"),
    ),
    responses(
        (status = 200, description = "Notices", body = AdminGameNoticesListResponse),
        (status = 404, description = "Game not found", body = crate::traits::ErrorResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
#[tracing::instrument(skip_all, fields(handler = "list_management_game_notices"))]
pub async fn list_game_notices(
    State(s): State<Arc<AppState>>,
    Path(game_id): Path<i64>,
) -> Result<Json<AdminGameNoticesListResponse>, WebError> {
    let game = crate::util::loader::prepare_game(&s.db.conn, game_id).await?;
    let notices = cds_db::game_notice::find_admin_views_by_game_id(&s.db.conn, game.id).await?;
    let total = notices.len() as u64;

    Ok(Json(AdminGameNoticesListResponse { notices, total }))
}

/// Creates game notice.
#[utoipa::path(
    post,
    path = "/",
    tag = "management-game",
    params(
        ("game_id" = i64, Path, description = "Game id"),
    ),
    request_body = CreateGameNoticeRequest,
    responses(
        (status = 201, description = "Notice created", body = GameNoticeResponse),
        (status = 400, description = "Invalid input", body = crate::traits::ErrorResponse),
        (status = 404, description = "Game or challenge not found", body = crate::traits::ErrorResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
#[tracing::instrument(skip_all, fields(handler = "create_game_notice"))]
pub async fn create_game_notice(
    State(s): State<Arc<AppState>>,
    Extension(ext): Extension<AuthPrincipal>,
    Path(game_id): Path<i64>,
    ReqJson(body): ReqJson<CreateGameNoticeRequest>,
) -> Result<(StatusCode, Json<GameNoticeResponse>), WebError> {
    let game = crate::util::loader::prepare_game(&s.db.conn, game_id).await?;

    let title = body.title.trim().to_owned();
    let content = body.content.trim().to_owned();
    if title.is_empty() || content.is_empty() {
        return Err(WebError::BadRequest(json!("invalid_notice")));
    }

    if let Some(challenge_id) = body.challenge_id
        && !cds_db::game_challenge::exists(&s.db.conn, game.id, challenge_id).await?
    {
        return Err(WebError::NotFound(json!("game_challenge_not_found")));
    }

    let operator_id = ext.operator.as_ref().map(|u| u.id);

    let game_notice = cds_db::game_notice::create(
        &s.db.conn,
        cds_db::game_notice::ActiveModel {
            game_id: Set(game.id),
            challenge_id: Set(body.challenge_id),
            title: Set(title),
            content: Set(content),
            created_by: Set(operator_id),
            updated_by: Set(operator_id),
            ..Default::default()
        },
    )
    .await?;

    Ok((
        StatusCode::CREATED,
        Json(GameNoticeResponse {
            notice: game_notice,
        }),
    ))
}

/// Updates an existing game notice and records the last editor.
#[utoipa::path(
    patch,
    path = "/{notice_id}",
    tag = "management-game",
    params(
        ("game_id" = i64, Path, description = "Game id"),
        ("notice_id" = i64, Path, description = "Notice id"),
    ),
    request_body = UpdateGameNoticeRequest,
    responses(
        (status = 200, description = "Notice updated", body = GameNoticeResponse),
        (status = 400, description = "Invalid input", body = crate::traits::ErrorResponse),
        (status = 404, description = "Notice or challenge not found", body = crate::traits::ErrorResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
#[tracing::instrument(skip_all, fields(handler = "update_game_notice"))]
pub async fn update_game_notice(
    State(s): State<Arc<AppState>>,
    Extension(ext): Extension<AuthPrincipal>,
    Path((game_id, notice_id)): Path<(i64, i64)>,
    ReqJson(body): ReqJson<UpdateGameNoticeRequest>,
) -> Result<Json<GameNoticeResponse>, WebError> {
    let existing = cds_db::game_notice::find_model_by_id(&s.db.conn, notice_id, game_id)
        .await?
        .ok_or(WebError::NotFound(json!("notice_not_found")))?;

    let next_title = match body.title {
        Some(raw) => {
            let trimmed = raw.trim().to_owned();
            if trimmed.is_empty() {
                return Err(WebError::BadRequest(json!("invalid_notice")));
            }
            Set(trimmed)
        }
        None => NotSet,
    };

    let next_content = match body.content {
        Some(raw) => {
            let trimmed = raw.trim().to_owned();
            if trimmed.is_empty() {
                return Err(WebError::BadRequest(json!("invalid_notice")));
            }
            Set(trimmed)
        }
        None => NotSet,
    };

    let next_challenge_id = match body.challenge_id {
        Some(Some(challenge_id)) => {
            if !cds_db::game_challenge::exists(&s.db.conn, game_id, challenge_id).await? {
                return Err(WebError::NotFound(json!("game_challenge_not_found")));
            }
            Set(Some(challenge_id))
        }
        Some(None) => Set(None),
        None => NotSet,
    };

    let operator_id = ext.operator.as_ref().map(|u| u.id);

    let updated = cds_db::game_notice::update(
        &s.db.conn,
        cds_db::game_notice::ActiveModel {
            id: Unchanged(existing.id),
            game_id: Unchanged(existing.game_id),
            challenge_id: next_challenge_id,
            title: next_title,
            content: next_content,
            updated_by: Set(operator_id),
            ..Default::default()
        },
    )
    .await?;

    Ok(Json(GameNoticeResponse { notice: updated }))
}

/// Deletes game notice.
#[utoipa::path(
    delete,
    path = "/{notice_id}",
    tag = "management-game",
    params(
        ("game_id" = i64, Path, description = "Game id"),
        ("notice_id" = i64, Path, description = "Notice id"),
    ),
    responses(
        (status = 200, description = "Deleted", body = EmptyJson),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
#[tracing::instrument(skip_all, fields(handler = "delete_game_notice"))]
pub async fn delete_game_notice(
    State(s): State<Arc<AppState>>,
    Path((game_id, notice_id)): Path<(i64, i64)>,
) -> Result<Json<EmptyJson>, WebError> {
    cds_db::game_notice::delete(&s.db.conn, notice_id, game_id).await?;
    Ok(Json(EmptyJson::default()))
}

#[cfg(test)]
mod tests {
    use super::UpdateGameNoticeRequest;

    #[test]
    fn update_request_distinguishes_omitted_null_and_set_challenge_id() {
        let omitted: UpdateGameNoticeRequest = serde_json::from_str(r#"{"title":"Hi"}"#).unwrap();
        assert_eq!(omitted.challenge_id, None);

        let cleared: UpdateGameNoticeRequest =
            serde_json::from_str(r#"{"challenge_id":null}"#).unwrap();
        assert_eq!(cleared.challenge_id, Some(None));

        let bound: UpdateGameNoticeRequest =
            serde_json::from_str(r#"{"challenge_id":42}"#).unwrap();
        assert_eq!(bound.challenge_id, Some(Some(42)));
    }
}
