use std::sync::Arc;

use axum::{
    Router,
    extract::{DefaultBodyLimit, Multipart, State},
};
use cds_db::{
    User,
    sea_orm::{Set, Unchanged},
};

use crate::{
    extract::Extension,
    traits::{AppState, AuthPrincipal, WebError, WebResponse},
    util,
};

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route(
            "/",
            axum::routing::post(save_user_avatar)
                .layer(DefaultBodyLimit::max(3 * 1024 * 1024 /* MB */)),
        )
        .route("/", axum::routing::delete(delete_user_avatar))
}

pub async fn save_user_avatar(
    State(s): State<Arc<AppState>>,

    Extension(ext): Extension<AuthPrincipal>,
    multipart: Multipart,
) -> Result<WebResponse<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized("".into()))?;

    let path = format!("users/{}/avatar", operator.id);
    let _ = util::media::save_img(s.media.clone(), path, multipart).await?;

    let _ = cds_db::user::update::<User>(
        &s.db.conn,
        cds_db::user::ActiveModel {
            id: Unchanged(operator.id),
            has_avatar: Set(true),
            ..Default::default()
        },
    )
    .await?;

    Ok(WebResponse::default())
}

pub async fn delete_user_avatar(
    State(s): State<Arc<AppState>>,

    Extension(ext): Extension<AuthPrincipal>,
) -> Result<WebResponse<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized("".into()))?;

    let path = format!("users/{}/avatar", operator.id);
    let _ = util::media::delete_img(s.media.clone(), path).await?;

    let _ = cds_db::user::update::<User>(
        &s.db.conn,
        cds_db::user::ActiveModel {
            id: Unchanged(operator.id),
            has_avatar: Set(false),
            ..Default::default()
        },
    )
    .await?;

    Ok(WebResponse::default())
}
