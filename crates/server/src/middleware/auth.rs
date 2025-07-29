use axum::{
    body::Body,
    extract::{FromRequestParts, Request},
    middleware::Next,
    response::{IntoResponse, Response},
};
use cds_db::{User, entity::user::Group, sea_orm::EntityTrait};
use serde_json::json;
use tower_sessions::Session;

use crate::{
    extract::Extension,
    traits::{AuthPrincipal, WebError},
};

pub async fn extract(req: Request<Body>, next: Next) -> Result<Response, WebError> {
    let (mut parts, body) = req.into_parts();

    let session = Session::from_request_parts(&mut parts, &())
        .await
        .map_err(|_| WebError::Unauthorized(json!("session_error")))?;

    let mut req = Request::from_parts(parts, body);

    let mut ext = req
        .extensions()
        .get::<AuthPrincipal>()
        .unwrap_or(&AuthPrincipal::default())
        .to_owned();

    if let Ok(Some(user_id)) = session.get::<i64>("user_id").await {
        if let Some(user) = cds_db::user::find_by_id::<User>(user_id).await? {
            if user.group == Group::Banned {
                return Err(WebError::Forbidden(json!("forbidden")));
            }

            ext.operator = Some(user);

            let called_times = session.get::<i64>("called_times").await?.unwrap_or(0);
            session.insert("called_times", called_times + 1).await?;
        }
    }

    req.extensions_mut().insert(ext);

    Ok(next.run(req).await)
}

pub async fn admin_only(
    Extension(ap): Extension<AuthPrincipal>,
    req: Request,
    next: Next,
) -> Result<impl IntoResponse, WebError> {
    if ap.operator.ok_or(WebError::Unauthorized(json!("")))?.group < Group::Admin {
        return Err(WebError::Forbidden(json!("forbidden")));
    }

    Ok(next.run(req).await)
}
