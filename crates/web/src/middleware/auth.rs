//! Session-based authentication helpers and permission gates.
//!
//! [`extract`] hydrates [`crate::traits::AuthPrincipal`] from `tower-sessions`
//! (`user_id` key). Management routes use [`require_management`], which is
//! backed by the same permission matcher that future permission groups can
//! reuse.

use std::sync::Arc;

use axum::{
    extract::{FromRequestParts, Request, State},
    http::header::AUTHORIZATION,
    middleware::Next,
    response::{IntoResponse, Response},
};
use cds_db::{Permission, UserAccountView};
use serde_json::json;
use tower_sessions::Session;
use tracing::{Span, debug, warn};

use crate::{
    extract::Extension,
    traits::{AppState, AuthMethod, AuthPrincipal, WebError},
};

/// Permissions that grant access to the management area.
pub const MANAGEMENT_PERMISSIONS: &[Permission] = &[
    Permission::ManagementConfig,
    Permission::ManagementChallenge,
    Permission::ManagementGame,
    Permission::ManagementIdp,
    Permission::ManagementUser,
];

/// Returns whether an account has every requested permission.
pub fn has_permissions(user: &UserAccountView, required: &[Permission]) -> bool {
    required
        .iter()
        .all(|required| user.permissions.iter().any(|granted| granted == required))
}

/// Returns whether an account has at least one permission from a set.
pub fn has_any_permissions(user: &UserAccountView, required: &[Permission]) -> bool {
    user.permissions
        .iter()
        .any(|granted| required.iter().any(|required| granted == required))
}

/// Checks a permission set and returns the standard forbidden response.
pub fn require_permissions(
    user: &UserAccountView,
    required: &[Permission],
) -> Result<(), WebError> {
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
    let request_span = req
        .extensions()
        .get::<Span>()
        .cloned()
        .unwrap_or_else(Span::current);
    let (mut parts, body) = req.into_parts();

    let authorization = parts.headers.get(AUTHORIZATION).cloned();
    if authorization.is_some() {
        request_span.record("auth_method", "bearer");
    }
    let bearer = authorization.as_ref().and_then(parse_bearer);

    if authorization.is_some() && bearer.is_none() {
        return Err(WebError::Unauthorized(json!("unauthorized")));
    }

    let session = if bearer.is_none() {
        Some(
            Session::from_request_parts(&mut parts, &())
                .await
                .map_err(|_| WebError::Unauthorized(json!("session_error")))?,
        )
    } else {
        None
    };

    let mut req = Request::from_parts(parts, body);

    let mut ext = req
        .extensions()
        .get::<AuthPrincipal>()
        .unwrap_or(&AuthPrincipal::default())
        .to_owned();

    if let Some(bearer) = bearer.as_deref() {
        request_span.record("auth_method", "personal_access_token");
        let token = bearer
            .strip_prefix("cds_pat_")
            .and_then(|value| value.split_once('.'))
            .ok_or(WebError::Unauthorized(json!("unauthorized")))?;
        let token_record = cds_db::personal_access_token::find_by_token_id(&s.db.conn, token.0)
            .await?
            .filter(|record| {
                record.revoked_at.is_none() && record.expires_at > time::OffsetDateTime::now_utc()
            })
            .ok_or(WebError::Unauthorized(json!("unauthorized")))?;

        if !crate::util::crypto::verify_access_token(token.1, &token_record.token_hash) {
            return Err(WebError::Unauthorized(json!("unauthorized")));
        }

        let user = cds_db::user::find_by_id::<UserAccountView>(&s.db.conn, token_record.user_id)
            .await?
            .ok_or(WebError::Unauthorized(json!("unauthorized")))?;
        request_span.record("operator_kind", "user");
        request_span.record("operator_id", user.id);
        request_span.record("operator_username", user.username.as_str());
        request_span.record("credential_id", token_record.token_id.as_str());
        if user.banned {
            warn!(user_id = user.id, username = %user.username, "banned user rejected");
            return Err(WebError::Forbidden(json!("forbidden")));
        }

        if token_record
            .last_used_at
            .map(|last| time::OffsetDateTime::now_utc() - last > time::Duration::minutes(5))
            .unwrap_or(true)
        {
            cds_db::personal_access_token::touch_last_used(&s.db.conn, token_record.id).await?;
        }
        ext.auth_method = AuthMethod::PersonalAccessToken;
        ext.operator = Some(user);
    } else if let Some(session) = session
        && let Ok(Some(user_id)) = session.get::<i64>("user_id").await
    {
        if let Some(user) = cds_db::user::find_by_id::<UserAccountView>(&s.db.conn, user_id).await?
        {
            request_span.record("operator_kind", "user");
            request_span.record("operator_id", user.id);
            request_span.record("operator_username", user.username.as_str());
            request_span.record("auth_method", "session");
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
            ext.auth_method = AuthMethod::Session;
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

fn parse_bearer(value: &axum::http::HeaderValue) -> Option<String> {
    let value = value.to_str().ok()?;
    let (scheme, credentials) = value.split_once(' ')?;
    if !scheme.eq_ignore_ascii_case("bearer") || credentials.is_empty() {
        return None;
    }
    Some(credentials.to_owned())
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

    if !has_any_permissions(&operator, MANAGEMENT_PERMISSIONS) {
        let error = WebError::Forbidden(json!("forbidden"));
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
    Ok(next.run(req).await)
}

macro_rules! define_permission_middleware {
    ($name:ident, $permission:expr) => {
        pub async fn $name(
            Extension(ap): Extension<AuthPrincipal>,
            req: Request,
            next: Next,
        ) -> Result<Response, WebError> {
            let operator = ap
                .operator
                .ok_or(WebError::Unauthorized(json!("unauthorized")))?;
            require_permissions(&operator, &[$permission])?;
            Ok(next.run(req).await)
        }
    };
}

define_permission_middleware!(require_management_config, Permission::ManagementConfig);
define_permission_middleware!(
    require_management_challenge,
    Permission::ManagementChallenge
);
define_permission_middleware!(require_management_game, Permission::ManagementGame);
define_permission_middleware!(require_management_idp, Permission::ManagementIdp);
define_permission_middleware!(require_management_user, Permission::ManagementUser);

#[cfg(test)]
mod tests {
    use super::*;

    fn user_with_permissions(permissions: Vec<Permission>) -> UserAccountView {
        UserAccountView {
            id: 1,
            name: "Test User".to_owned(),
            username: "test".to_owned(),
            verified: Some(true),
            banned: false,
            permissions,
            description: None,
            hashed_password: String::new(),
            avatar_hash: None,
            created_at: time::OffsetDateTime::UNIX_EPOCH,
            updated_at: time::OffsetDateTime::UNIX_EPOCH,
        }
    }

    #[test]
    fn bearer_scheme_is_case_insensitive() {
        let header = axum::http::HeaderValue::from_static("bearer cds_pat_id.secret");
        assert_eq!(parse_bearer(&header).as_deref(), Some("cds_pat_id.secret"));
    }

    #[test]
    fn management_permissions_are_checked_exactly() {
        let user = user_with_permissions(vec![Permission::ManagementChallenge]);

        assert!(has_permissions(&user, &[Permission::ManagementChallenge]));
        assert!(!has_permissions(&user, &[Permission::ManagementConfig]));
    }

    #[test]
    fn any_management_permission_grants_management_access() {
        let user = user_with_permissions(vec![Permission::ManagementChallenge]);

        assert!(has_any_permissions(&user, MANAGEMENT_PERMISSIONS));
    }

    #[test]
    fn basic_permission_does_not_grant_management_access() {
        let user = user_with_permissions(vec![Permission::Basic]);

        assert!(!has_any_permissions(&user, MANAGEMENT_PERMISSIONS));
    }

    #[test]
    fn resource_permissions_do_not_bleed_into_each_other() {
        let user = user_with_permissions(vec![Permission::ManagementUser]);

        assert!(has_permissions(&user, &[Permission::ManagementUser]));
        assert!(!has_permissions(&user, &[Permission::ManagementChallenge]));
        assert!(!has_permissions(&user, &[Permission::ManagementGame]));
    }
}
