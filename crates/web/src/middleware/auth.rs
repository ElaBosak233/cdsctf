//! Session-based authentication helpers and an admin-only gate.
//!
//! [`extract`] hydrates [`crate::traits::AuthPrincipal`] from `tower-sessions`
//! (`user_id` key). [`admin_only`] rejects callers whose
//! [`cds_db::user::Group`] is below Admin.

use std::sync::Arc;

use axum::{
    extract::{FromRequestParts, Request, State},
    middleware::Next,
    response::{IntoResponse, Response},
};
use cds_db::{User, user::Group};
use serde_json::json;
use tower_sessions::Session;
use tracing::{Span, debug, warn};

use crate::{
    extract::Extension,
    traits::{AppState, AuthPrincipal, WebError},
};

/// Loads the signed-in user (if any), enforces banned accounts, bumps a session
/// counter, then continues the chain.
pub async fn extract(
    State(s): State<Arc<AppState>>,

    req: Request,
    next: Next,
) -> Result<Response, WebError> {
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
        if let Some(user) = cds_db::user::find_by_id::<User>(&s.db.conn, user_id).await? {
            if user.group == Group::Banned {
                warn!(user_id = user.id, username = %user.username, "banned user rejected");
                return Err(WebError::Forbidden(json!("forbidden")));
            }

            debug!(
                user_id = user.id,
                username = %user.username,
                group = ?user.group,
                "authenticated request principal loaded"
            );
            Span::current().record("username", user.username.as_str());
            ext.operator = Some(user);

            let called_times = session.get::<i64>("called_times").await?.unwrap_or(0);
            session.insert("called_times", called_times + 1).await?;
            debug!(
                called_times = called_times + 1,
                "session call counter updated"
            );
        }
    }

    req.extensions_mut().insert(ext);

    Ok(next.run(req).await)
}

/// Requires an authenticated operator with [`Group::Admin`] or higher
/// privileges.
pub async fn admin_only(
    Extension(ap): Extension<AuthPrincipal>,
    req: Request,
    next: Next,
) -> Result<impl IntoResponse, WebError> {
    let operator = ap.operator.ok_or(WebError::Unauthorized(json!("")))?;

    if operator.group < Group::Admin {
        warn!(
            user_id = operator.id,
            username = %operator.username,
            group = ?operator.group,
            "non-admin user rejected"
        );
        return Err(WebError::Forbidden(json!("forbidden")));
    }

    debug!(
        user_id = operator.id,
        username = %operator.username,
        group = ?operator.group,
        "admin access granted"
    );
    Span::current().record("username", operator.username.as_str());
    Ok(next.run(req).await)
}
