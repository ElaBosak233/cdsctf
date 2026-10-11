//! Session-based authentication helpers and permission gates.
//!
//! [`extract`] hydrates [`crate::traits::AuthPrincipal`] from `tower-sessions`
//! (`user_id` key). Management routes use [`require_management`], which is
//! backed by the same permission matcher that future permission groups can
//! reuse.

use std::sync::Arc;

use axum::{
    extract::{FromRequestParts, Request, State},
    middleware::Next,
    response::{IntoResponse, Response},
};
use cds_db::UserAccountView;
use serde_json::json;
use tower_sessions::Session;
use tracing::{Span, debug, warn};

use crate::{
    extract::Extension,
    traits::{AppState, AuthPrincipal, WebError},
};

/// Permission required to access the management API.
pub const MANAGEMENT_ACCESS: &str = "management:access";

/// Returns whether an account has every requested permission.
///
/// A wildcard permission (`*`) grants all capabilities, while a namespace
/// wildcard such as `management:*` grants permissions in that namespace.
pub fn has_permissions(user: &UserAccountView, required: &[&str]) -> bool {
    required.iter().all(|required| {
        user.permissions.iter().any(|granted| {
            granted == "*"
                || granted == *required
                || granted
                    .strip_suffix('*')
                    .is_some_and(|prefix| required.starts_with(prefix))
        })
    })
}

/// Checks a permission set and returns the standard forbidden response.
pub fn require_permissions(user: &UserAccountView, required: &[&str]) -> Result<(), WebError> {
    if has_permissions(user, required) {
        Ok(())
    } else {
        Err(WebError::Forbidden(json!("forbidden")))
    }
}

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
        if let Some(user) = cds_db::user::find_by_id::<UserAccountView>(&s.db.conn, user_id).await?
        {
            if user.banned {
                warn!(user_id = user.id, username = %user.username, "banned user rejected");
                return Err(WebError::Forbidden(json!("forbidden")));
            }

            debug!(
                user_id = user.id,
                username = %user.username,
                permissions = ?user.permissions,
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

/// Requires an authenticated operator with management access.
pub async fn require_management(
    Extension(ap): Extension<AuthPrincipal>,
    req: Request,
    next: Next,
) -> Result<impl IntoResponse, WebError> {
    let operator = ap
        .operator
        .ok_or(WebError::Unauthorized(json!("unauthorized")))?;

    if let Err(error) = require_permissions(&operator, &[MANAGEMENT_ACCESS]) {
        warn!(
            user_id = operator.id,
            username = %operator.username,
            permissions = ?operator.permissions,
            "operator missing management permission"
        );
        return Err(error);
    }

    debug!(
        user_id = operator.id,
        username = %operator.username,
        permissions = ?operator.permissions,
        "management access granted"
    );
    Span::current().record("username", operator.username.as_str());
    Ok(next.run(req).await)
}
