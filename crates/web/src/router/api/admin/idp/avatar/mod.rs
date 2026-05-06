//! Admin IdP avatar upload/delete routes.

use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{DefaultBodyLimit, Multipart, State},
};
use cds_db::{
    Idp,
    sea_orm::{Set, Unchanged},
};
use utoipa_axum::{
    router::{OpenApiRouter, UtoipaMethodRouterExt},
    routes,
};

use serde_json::json;

use crate::{
    extract::Path,
    traits::{AppState, EmptyJson, WebError},
    util::media::handle_multipart,
};

pub fn router(state: Arc<AppState>) -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::from(Router::new().with_state(state.clone()))
        .routes(
            routes!(save_idp_avatar)
                .with_state(state.clone())
                .layer(DefaultBodyLimit::max(512 * 1024 * 1024 /* MB */)),
        )
        .routes(routes!(delete_idp_avatar).with_state(state.clone()))
}

#[utoipa::path(
    post,
    path = "/",
    tag = "admin-idp",
    params(("idp_id" = i64, Path, description = "IdP id")),
    responses(
        (status = 200, description = "Avatar saved", body = EmptyJson),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
#[tracing::instrument(skip_all, fields(handler = "save_idp_avatar"))]
pub async fn save_idp_avatar(
    State(s): State<Arc<AppState>>,
    Path(idp_id): Path<i64>,
    multipart: Multipart,
) -> Result<Json<EmptyJson>, WebError> {
    let data = handle_multipart(multipart, mime::IMAGE).await?;
    let data = cds_media::util::img_convert_to_webp(data).await?;

    let hash = cds_media::util::hash(data.clone());

    s.media.save("media".to_owned(), hash.clone(), data).await?;

    let _ = cds_db::idp::update_idp::<Idp>(
        &s.db.conn,
        cds_db::idp::IdpActiveModel {
            id: Unchanged(idp_id),
            avatar_hash: Set(Some(hash)),
            ..Default::default()
        },
    )
    .await?;

    Ok(Json(EmptyJson::default()))
}

#[utoipa::path(
    delete,
    path = "/",
    tag = "admin-idp",
    params(("idp_id" = i64, Path, description = "IdP id")),
    responses(
        (status = 200, description = "Avatar removed", body = EmptyJson),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
#[tracing::instrument(skip_all, fields(handler = "delete_idp_avatar"))]
pub async fn delete_idp_avatar(
    State(s): State<Arc<AppState>>,
    Path(idp_id): Path<i64>,
) -> Result<Json<EmptyJson>, WebError> {
    let idp = cds_db::idp::find_idp_by_id::<Idp>(&s.db.conn, idp_id)
        .await?
        .ok_or(WebError::NotFound(json!("idp_not_found")))?;

    if let Some(hash) = idp.avatar_hash {
        s.media.delete("media".to_owned(), hash).await?;
    }

    let _ = cds_db::idp::update_idp::<Idp>(
        &s.db.conn,
        cds_db::idp::IdpActiveModel {
            id: Unchanged(idp_id),
            avatar_hash: Set(None),
            ..Default::default()
        },
    )
    .await?;

    Ok(Json(EmptyJson::default()))
}
