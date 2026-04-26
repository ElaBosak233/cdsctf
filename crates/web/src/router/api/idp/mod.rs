//! Public IdP routes: IdP list, Rune-backed login, bind, and unbind.

/// Defines the `idp_id` submodule (see sibling `*.rs` files).
mod idp_id;

use std::{collections::HashMap, sync::Arc};

use axum::{
    Json, Router,
    extract::{Path, State},
};
use cds_db::{
    Email, Idp, User, UserIdp,
    sea_orm::ActiveValue::{NotSet, Set},
    user::{FindUserOptions, Group},
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tower_sessions::Session;
use tracing::{Span, info};
use utoipa_axum::{
    router::{OpenApiRouter, UtoipaMethodRouterExt},
    routes,
};

use crate::{
    extract::{Extension, Json as ReqJson},
    traits::{AppState, AuthPrincipal, WebError},
    util,
};

pub fn router(state: Arc<AppState>) -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::from(Router::new().with_state(state.clone()))
        .routes(routes!(list_idps).with_state(state.clone()))
        .routes(routes!(login).with_state(state.clone()))
        .nest("/{idp_id}", idp_id::router(state.clone()))
}

#[derive(Clone, Debug, Serialize, utoipa::ToSchema)]
pub struct IdpsResponse {
    pub idps: Vec<Idp>,
}

#[derive(Clone, Debug, Serialize, utoipa::ToSchema)]
pub struct IdpLoginResponse {
    pub user: User,
    pub identity: UserIdp,
    pub registered: bool,
}

#[derive(Clone, Debug, Deserialize, utoipa::ToSchema)]
pub struct IdpAuthRequest {
    #[serde(default)]
    pub params: HashMap<String, String>,
}

#[utoipa::path(
    get,
    path = "/",
    tag = "idp",
    responses((status = 200, description = "Enabled IdPs", body = IdpsResponse))
)]
#[tracing::instrument(skip_all, fields(handler = "list_idps"))]
pub async fn list_idps(State(s): State<Arc<AppState>>) -> Result<Json<IdpsResponse>, WebError> {
    let idps = cds_db::idp::find_public_idps::<Idp>(&s.db.conn)
        .await?
        .into_iter()
        .map(Idp::desensitize)
        .collect();
    Ok(Json(IdpsResponse { idps }))
}

#[utoipa::path(
    post,
    path = "/{idp_id}/login",
    tag = "idp",
    params(("idp_id" = i64, Path, description = "IdP id")),
    request_body = IdpAuthRequest,
    responses(
        (status = 200, description = "Logged in through IdP", body = IdpLoginResponse),
        (status = 403, description = "Registration disabled", body = crate::traits::ErrorResponse),
        (status = 404, description = "Not found", body = crate::traits::ErrorResponse)
    )
)]
#[tracing::instrument(skip_all, fields(handler = "idp_login"))]
pub async fn login(
    State(s): State<Arc<AppState>>,
    session: Session,
    Path(idp_id): Path<i64>,
    ReqJson(body): ReqJson<IdpAuthRequest>,
) -> Result<Json<IdpLoginResponse>, WebError> {
    let idp = enabled_idp(&s, idp_id).await?;
    cds_idp::Idp::preload(idp.id, &idp.script)
        .await
        .map_err(|err| WebError::BadRequest(json!(err.to_string())))?;
    let payload = cds_idp::Idp::login(idp.id, body.params).await?;

    if let Some(identity) = cds_db::user_idp::find_user_idp_by_auth_key::<
        cds_db::user_idp::UserIdpModel,
    >(&s.db.conn, idp.id, &payload.auth_key)
    .await?
    {
        cds_db::user_idp::update_user_idp_data(&s.db.conn, &identity, Some(json!(payload.data)))
            .await?;
        let user = cds_db::user::find_by_id::<User>(&s.db.conn, identity.user_id)
            .await?
            .ok_or(WebError::NotFound(json!("user_not_found")))?;
        session.insert("user_id", user.id).await?;
        Span::current().record("username", user.username.as_str());
        info!(
            user_id = user.id,
            idp_id = idp.id,
            "user logged in through idp"
        );
        let identity = cds_db::user_idp::find_user_idp_by_auth_key::<UserIdp>(
            &s.db.conn,
            idp.id,
            &payload.auth_key,
        )
        .await?
        .ok_or(WebError::NotFound(json!("user_idp_not_found")))?;
        return Ok(Json(IdpLoginResponse {
            user,
            identity,
            registered: false,
        }));
    }

    let user = create_user_from_payload(&s, &payload).await?;
    let identity = create_user_idp(&s, &idp, user.id, &payload).await?;
    session.insert("user_id", user.id).await?;
    Span::current().record("username", user.username.as_str());
    info!(
        user_id = user.id,
        idp_id = idp.id,
        "user registered through idp"
    );

    Ok(Json(IdpLoginResponse {
        user,
        identity,
        registered: true,
    }))
}

pub(crate) async fn enabled_idp(s: &AppState, idp_id: i64) -> Result<Idp, WebError> {
    let idp = cds_db::idp::find_idp_by_id::<Idp>(&s.db.conn, idp_id)
        .await?
        .ok_or(WebError::NotFound(json!("idp_not_found")))?;
    if !idp.enabled {
        return Err(WebError::NotFound(json!("idp_not_found")));
    }
    Ok(idp)
}

pub(crate) async fn user_map(
    s: &AppState,
    user: &User,
) -> Result<HashMap<String, String>, WebError> {
    let mut map = HashMap::from([
        ("id".to_string(), user.id.to_string()),
        ("username".to_string(), user.username.clone()),
        ("name".to_string(), user.name.clone()),
        (
            "group".to_string(),
            format!("{:?}", user.group).to_lowercase(),
        ),
    ]);
    if let Some(email) = cds_db::email::find_by_user_id::<Email>(&s.db.conn, user.id)
        .await?
        .into_iter()
        .find(|email| email.verified)
    {
        map.insert("email".to_string(), email.email);
    }
    Ok(map)
}

async fn create_user_from_payload(
    s: &AppState,
    payload: &cds_idp::IdentityPayload,
) -> Result<User, WebError> {
    let username = payload
        .data
        .get("username")
        .or_else(|| payload.data.get("login"))
        .cloned()
        .unwrap_or_else(|| payload.auth_key.clone())
        .to_lowercase();
    let name = payload
        .data
        .get("name")
        .cloned()
        .unwrap_or_else(|| username.clone());
    let email = payload.data.get("email").cloned();

    if !cds_db::user::is_username_unique(&s.db.conn, 0, &username).await? {
        return Err(WebError::Conflict(json!("username_already_exists")));
    }
    if let Some(email) = &email {
        if !cds_db::user::is_email_unique(&s.db.conn, email).await? {
            return Err(WebError::Conflict(json!("email_already_exists")));
        }
    }

    let user = cds_db::user::create::<User>(
        &s.db.conn,
        cds_db::user::ActiveModel {
            username: Set(username),
            name: Set(name),
            hashed_password: Set(util::crypto::hash_password(nanoid::nanoid!())),
            group: Set(Group::User),
            ..Default::default()
        },
    )
    .await?;

    if let Some(email) = email {
        let _ = cds_db::email::create::<Email>(
            &s.db.conn,
            cds_db::email::ActiveModel {
                email: Set(email.to_lowercase()),
                user_id: Set(user.id),
                verified: Set(true),
            },
        )
        .await?;
    }

    Ok(user)
}

pub(crate) async fn create_user_idp(
    s: &AppState,
    idp: &Idp,
    user_id: i64,
    payload: &cds_idp::IdentityPayload,
) -> Result<UserIdp, WebError> {
    Ok(cds_db::user_idp::create_user_idp::<UserIdp>(
        &s.db.conn,
        cds_db::user_idp::UserIdpActiveModel {
            id: NotSet,
            user_id: Set(user_id),
            idp_id: Set(idp.id),
            auth_key: Set(payload.auth_key.clone()),
            data: Set(Some(json!(payload.data))),
            ..Default::default()
        },
    )
    .await?)
}
