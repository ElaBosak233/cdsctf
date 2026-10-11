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
use cds_db::{Permission, UserAccountView};
use serde_json::json;
use tower_sessions::Session;
use tracing::{Span, debug, warn};

use crate::{
    extract::Extension,
    traits::{AppState, AuthPrincipal, WebError},
};

/// Permissions required to access the complete management API.
pub const MANAGEMENT_PERMISSIONS: &[Permission] = &[
    Permission::ManagementConfig,
    Permission::ManagementChallenge,
    Permission::ManagementGame,
    Permission::ManagementIdp,
    Permission::ManagementUser,
];

/// Wildcard permission used to grant access to the management area.
pub const MANAGEMENT_PERMISSION: Permission = Permission::ManagementAny;

fn permission_matches(granted: Permission, required: Permission) -> bool {
    match (granted, required) {
        (Permission::Any, _) | (_, Permission::Any) => true,
        (Permission::ManagementAny, permission) | (permission, Permission::ManagementAny) => {
            matches!(
                permission,
                Permission::ManagementConfig
                    | Permission::ManagementChallenge
                    | Permission::ManagementGame
                    | Permission::ManagementIdp
                    | Permission::ManagementUser
                    | Permission::ManagementAny
            )
        }
        (granted, required) => granted == required,
    }
}

/// Returns whether an account has every requested permission.
pub fn has_permissions(user: &UserAccountView, required: &[Permission]) -> bool {
    required.iter().all(|required| {
        user.permissions
            .iter()
            .any(|granted| permission_matches(*granted, *required))
    })
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

    if let Err(error) = require_permissions(&operator, &[MANAGEMENT_PERMISSION]) {
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
    fn management_wildcard_matches_any_management_permission() {
        let user = user_with_permissions(vec![Permission::ManagementChallenge]);

        assert!(has_permissions(&user, &[Permission::ManagementAny]));
        assert!(!has_permissions(&user, &[Permission::ManagementConfig]));
    }

    #[test]
    fn granted_management_wildcard_matches_concrete_management_permission() {
        let user = user_with_permissions(vec![Permission::ManagementAny]);

        assert!(has_permissions(&user, &[Permission::ManagementChallenge]));
        assert!(!has_permissions(&user, &[Permission::Basic]));
    }

    #[test]
    fn basic_permission_does_not_grant_management_access() {
        let user = user_with_permissions(vec![Permission::Basic]);

        assert!(!has_permissions(&user, &[Permission::ManagementAny]));
    }

    #[test]
    fn resource_permissions_do_not_bleed_into_each_other() {
        let user = user_with_permissions(vec![Permission::ManagementUser]);

        assert!(has_permissions(&user, &[Permission::ManagementUser]));
        assert!(!has_permissions(&user, &[Permission::ManagementChallenge]));
        assert!(!has_permissions(&user, &[Permission::ManagementGame]));
    }

    #[test]
    fn global_wildcard_matches_every_permission() {
        let user = user_with_permissions(vec![Permission::Any]);

        assert!(has_permissions(&user, &[Permission::Basic]));
        assert!(has_permissions(&user, &[Permission::ManagementChallenge]));
    }
}
