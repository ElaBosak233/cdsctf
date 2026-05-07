//! Public routes for a single IdP.

/// Defines the `avatar` submodule (see sibling `*.rs` files).
mod avatar;

use std::{collections::HashMap, sync::Arc};

use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
};
use cds_db::{
    Email, Idp, User, UserIdp,
    sea_orm::ActiveValue::Set,
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
use validator::Validate;
use cds_db::sea_orm::NotSet;
use crate::{
    extract::{Extension, Json as ReqJson},
    router::api::idp::{IdpAuthRequest},
    traits::{AppState, AuthPrincipal, WebError},
    util,
};

pub fn router(state: Arc<AppState>) -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::from(Router::new().with_state(state.clone()))
        .routes(routes!(get_idp, bind).with_state(state.clone()))
        .routes(routes!(login).with_state(state.clone()))
        .routes(routes!(register).with_state(state.clone()))
        .nest(
            "/avatar",
            OpenApiRouter::from(Router::new())
                .routes(routes!(avatar::get_idp_avatar).with_state(state.clone())),
        )
}

#[derive(Clone, Debug, Serialize, utoipa::ToSchema)]
pub struct IdpBindResponse {
    pub idp: UserIdp,
}

#[derive(Clone, Debug, Serialize, utoipa::ToSchema)]
pub struct IdpResponse {
    pub idp: Idp,
}

#[utoipa::path(
    get,
    path = "/",
    tag = "idp",
    params(("idp_id" = i64, Path, description = "IdP id")),
    responses(
        (status = 200, description = "IdP info", body = IdpResponse),
        (status = 404, description = "Not found", body = crate::traits::ErrorResponse)
    )
)]
#[tracing::instrument(skip_all, fields(handler = "get_idp"))]
pub async fn get_idp(
    State(s): State<Arc<AppState>>,
    Path(idp_id): Path<i64>,
) -> Result<Json<IdpResponse>, WebError> {
    let idp = enabled_idp(&s, idp_id).await?;
    Ok(Json(IdpResponse {
        idp: idp.desensitize(),
    }))
}

#[utoipa::path(
    post,
    path = "/bind",
    tag = "idp",
    params(("idp_id" = i64, Path, description = "IdP id")),
    request_body = IdpAuthRequest,
    responses(
        (status = 201, description = "IdP bound", body = IdpBindResponse),
        (status = 401, description = "Unauthorized", body = crate::traits::ErrorResponse),
        (status = 409, description = "Already bound", body = crate::traits::ErrorResponse)
    )
)]
#[tracing::instrument(skip_all, fields(handler = "idp_bind"))]
pub async fn bind(
    State(s): State<Arc<AppState>>,
    Extension(ext): Extension<AuthPrincipal>,
    Path(idp_id): Path<i64>,
    ReqJson(body): ReqJson<IdpAuthRequest>,
) -> Result<(StatusCode, Json<IdpBindResponse>), WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    let idp = enabled_idp(&s, idp_id).await?;
    cds_idp::Idp::preload(idp.id, &idp.script)
        .await
        .map_err(|err| WebError::BadRequest(json!(err.to_string())))?;

    let payload = cds_idp::Idp::bind(idp.id, body.params, user_map(&s, &operator).await?).await?;

    if cds_db::user_idp::find_user_idp_by_auth_key::<UserIdp>(&s.db.conn, idp.id, &payload.auth_key)
        .await?
        .is_some()
    {
        return Err(WebError::Conflict(json!("user_idp_already_bound")));
    }
    if cds_db::user_idp::find_user_idp_by_user_and_idp::<UserIdp>(&s.db.conn, operator.id, idp.id)
        .await?
        .is_some()
    {
        return Err(WebError::Conflict(json!("idp_already_bound")));
    }

    let identity = create_user_idp(&s, &idp, operator.id, &payload).await?;
    Ok((StatusCode::CREATED, Json(IdpBindResponse { idp: identity })))
}

#[derive(Clone, Debug, Serialize, utoipa::ToSchema)]
pub struct IdpLoginResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity: Option<UserIdp>,
    pub registered: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requires_registration: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_identity: Option<PendingIdentity>,
}

#[derive(Clone, Debug, Serialize, Deserialize, utoipa::ToSchema)]
pub struct PendingIdentity {
    pub token: String,
    pub idp_id: i64,
    #[serde(default)]
    pub data: HashMap<String, String>,
}

#[derive(Clone, Debug, Deserialize, Validate, utoipa::ToSchema)]
pub struct IdpRegisterRequest {
    pub token: String,
    #[validate(length(min = 3, max = 20))]
    pub username: String,
    pub name: String,
    #[validate(email)]
    pub email: String,
    pub password: String,
}

#[utoipa::path(
    post,
    path = "/login",
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
            user: Some(user),
            identity: Some(identity),
            registered: false,
            requires_registration: None,
            pending_identity: None,
        }));
    }

    let token = nanoid::nanoid!();
    s.cache
        .set_ex(
            format!("idp_pending:{}", token),
            serde_json::json!({
                "idp_id": idp.id,
                "auth_key": payload.auth_key,
            }),
            600,
        )
        .await?;

    Ok(Json(IdpLoginResponse {
        user: None,
        identity: None,
        registered: false,
        requires_registration: Some(true),
        pending_identity: Some(PendingIdentity {
            token,
            idp_id: idp.id,
            data: payload.data,
        }),
    }))
}

#[utoipa::path(
    post,
    path = "/register",
    tag = "idp",
    params(("idp_id" = i64, Path, description = "IdP id")),
    request_body = IdpRegisterRequest,
    responses(
        (status = 201, description = "Registered through IdP", body = crate::router::api::user::UserResponse),
        (status = 400, description = "Bad request", body = crate::traits::ErrorResponse),
        (status = 403, description = "Registration disabled", body = crate::traits::ErrorResponse),
        (status = 404, description = "Not found", body = crate::traits::ErrorResponse),
        (status = 409, description = "Conflict", body = crate::traits::ErrorResponse)
    )
)]
#[tracing::instrument(skip_all, fields(handler = "idp_register"))]
pub async fn register(
    State(s): State<Arc<AppState>>,
    session: Session,
    Path(idp_id): Path<i64>,
    ReqJson(mut body): ReqJson<IdpRegisterRequest>,
) -> Result<(StatusCode, Json<crate::router::api::user::UserResponse>), WebError> {
    let idp = enabled_idp(&s, idp_id).await?;

    // Resolve auth_key from one-time token stored in cache
    let pending_key = format!("idp_pending:{}", body.token);
    let pending: Option<serde_json::Value> = s.cache.get_del(&pending_key).await?;
    let pending = pending.ok_or(WebError::BadRequest(json!("invalid_or_expired_token")))?;
    let auth_key = pending
        .get("auth_key")
        .and_then(|v| v.as_str())
        .ok_or(WebError::BadRequest(json!("invalid_token_payload")))?
        .to_string();

    // Verify the auth_key is not already bound
    if cds_db::user_idp::find_user_idp_by_auth_key::<cds_db::user_idp::UserIdpModel>(
        &s.db.conn,
        idp.id,
        &auth_key,
    )
    .await?
    .is_some()
    {
        return Err(WebError::Conflict(json!("user_idp_already_bound")));
    }

    body.email = body.email.to_lowercase();
    if !cds_db::user::is_email_unique(&s.db.conn, &body.email).await? {
        return Err(WebError::Conflict(json!("email_already_exists")));
    }

    body.username = body.username.to_lowercase();
    if !cds_db::user::is_username_unique(&s.db.conn, 0, &body.username).await? {
        return Err(WebError::Conflict(json!("username_already_exists")));
    }

    let hashed_password = util::crypto::hash_password(body.password);

    let user = cds_db::user::create::<User>(
        &s.db.conn,
        cds_db::user::ActiveModel {
            username: Set(body.username),
            name: Set(body.name),
            hashed_password: Set(hashed_password),
            group: Set(
                if cds_db::user::find::<User>(&s.db.conn, FindUserOptions::default())
                    .await?
                    .1
                    == 0
                {
                    Group::Admin
                } else {
                    Group::User
                },
            ),
            ..Default::default()
        },
    )
    .await?;

    let _ = cds_db::email::create::<Email>(
        &s.db.conn,
        cds_db::email::ActiveModel {
            user_id: Set(user.id),
            email: Set(body.email),
            verified: Set(!cds_db::get_config(&s.db.conn).await.email.enabled),
        },
    )
    .await?;

    let _ = create_user_idp(
        &s,
        &idp,
        user.id,
        &cds_idp::IdentityPayload {
            auth_key,
            data: HashMap::new(),
        },
    )
    .await?;

    session.insert("user_id", user.id).await?;
    Span::current().record("username", user.username.as_str());

    info!(
        user_id = user.id,
        username = %user.username,
        idp_id = idp.id,
        "user registered through idp"
    );

    Ok((StatusCode::CREATED, Json(crate::router::api::user::UserResponse { user })))
}

async fn enabled_idp(s: &AppState, idp_id: i64) -> Result<Idp, WebError> {
    let idp = cds_db::idp::find_idp_by_id::<Idp>(&s.db.conn, idp_id)
        .await?
        .ok_or(WebError::NotFound(json!("idp_not_found")))?;
    if !idp.enabled {
        return Err(WebError::NotFound(json!("idp_not_found")));
    }
    Ok(idp)
}

async fn user_map(
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

async fn create_user_idp(
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
