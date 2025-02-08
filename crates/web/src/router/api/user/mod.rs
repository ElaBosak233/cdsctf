use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};
use axum::{
    Router,
    extract::{DefaultBodyLimit, Multipart},
    http::{HeaderMap, StatusCode, header::SET_COOKIE},
    response::IntoResponse,
};
use cds_db::{entity::user::Group, get_db};
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set, Unchanged},
    ColumnTrait, Condition, EntityTrait, JoinType, PaginatorTrait, QueryFilter, QuerySelect,
    RelationTrait,
    prelude::Expr,
    sea_query::Func,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use validator::Validate;

use crate::{
    extract::{Extension, Json, Path, Query, VJson},
    model::Metadata,
    traits::{Ext, WebError, WebResponse},
    util,
    util::jwt,
};

pub fn router() -> Router {
    Router::new()
        .route("/", axum::routing::get(get_user))
        .route("/", axum::routing::post(create_user))
        .route("/{id}", axum::routing::put(update_user))
        .route("/{id}", axum::routing::delete(delete_user))
        .route("/{id}/teams", axum::routing::get(get_user_teams))
        .route("/profile", axum::routing::get(get_user_profile))
        .route("/profile", axum::routing::put(update_user_profile))
        .route("/profile", axum::routing::delete(delete_user_profile))
        .route("/login", axum::routing::post(user_login))
        .route("/register", axum::routing::post(user_register))
        .route("/{id}/avatar", axum::routing::get(get_user_avatar))
        .route(
            "/{id}/avatar/metadata",
            axum::routing::get(get_user_avatar_metadata),
        )
        .route(
            "/{id}/avatar",
            axum::routing::post(save_user_avatar)
                .layer(DefaultBodyLimit::max(3 * 1024 * 1024 /* MB */)),
        )
        .route("/{id}/avatar", axum::routing::delete(delete_user_avatar))
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetUserRequest {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub email: Option<String>,
    pub group: Option<Group>,
    pub page: Option<u64>,
    pub size: Option<u64>,
    pub sorts: Option<String>,
}

pub async fn get_user(
    Query(params): Query<GetUserRequest>,
) -> Result<WebResponse<Vec<cds_db::transfer::User>>, WebError> {
    let (mut users, total) = cds_db::transfer::user::find(
        params.id,
        params.name,
        None,
        params.group,
        params.email,
        params.sorts,
        params.page,
        params.size,
    )
    .await?;

    for user in users.iter_mut() {
        user.desensitize();
    }

    Ok(WebResponse {
        code: StatusCode::OK.as_u16(),
        data: Some(users),
        total: Some(total),
        ..Default::default()
    })
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
pub struct CreateUserRequest {
    pub username: String,
    pub nickname: String,
    pub email: String,
    pub password: String,
    pub group: Group,
}

pub async fn create_user(
    Extension(ext): Extension<Ext>, VJson(mut body): VJson<CreateUserRequest>,
) -> Result<WebResponse<cds_db::transfer::User>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    if operator.group != Group::Admin {
        return Err(WebError::Unauthorized(json!("")));
    }

    body.email = body.email.to_lowercase();
    body.username = body.username.to_lowercase();

    let hashed_password = Argon2::default()
        .hash_password(body.password.as_bytes(), &SaltString::generate(&mut OsRng))
        .unwrap()
        .to_string();

    let user = cds_db::entity::user::ActiveModel {
        username: Set(body.username),
        nickname: Set(body.nickname),
        email: Set(body.email),
        hashed_password: Set(hashed_password),
        group: Set(body.group),
        ..Default::default()
    }
    .insert(get_db())
    .await?;
    let mut user = cds_db::transfer::User::from(user);

    user.desensitize();

    Ok(WebResponse {
        code: StatusCode::OK.as_u16(),
        data: Some(user),
        ..Default::default()
    })
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
pub struct UpdateUserRequest {
    pub id: Option<i64>,
    #[validate(length(min = 3, max = 20))]
    pub username: Option<String>,
    pub nickname: Option<String>,
    #[validate(email)]
    pub email: Option<String>,
    pub password: Option<String>,
    pub group: Option<Group>,
    pub description: Option<String>,
}

/// Update a user with given data.
///
/// # Prerequisite
/// - Operator is admin.
pub async fn update_user(
    Extension(ext): Extension<Ext>, Path(id): Path<i64>, VJson(mut body): VJson<UpdateUserRequest>,
) -> Result<WebResponse<cds_db::transfer::User>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized("".into()))?;
    body.id = Some(id);
    if operator.group != Group::Admin {
        return Err(WebError::Forbidden("".into()));
    }

    let user = cds_db::entity::user::Entity::find_by_id(body.id.unwrap_or(0))
        .filter(cds_db::entity::user::Column::DeletedAt.is_null())
        .one(get_db())
        .await?
        .ok_or(WebError::BadRequest("".into()))?;

    if let Some(email) = body.email {
        body.email = Some(email.to_lowercase());
    }

    if let Some(username) = body.username {
        body.username = Some(username.to_lowercase());
    }

    if let Some(password) = body.password {
        let hashed_password = Argon2::default()
            .hash_password(password.as_bytes(), &SaltString::generate(&mut OsRng))
            .unwrap()
            .to_string();
        body.password = Some(hashed_password);
    }

    let user = cds_db::entity::user::ActiveModel {
        id: Unchanged(user.id),
        username: body.username.map_or(NotSet, Set),
        nickname: body.nickname.map_or(NotSet, Set),
        email: body.email.map_or(NotSet, Set),
        hashed_password: body.password.map_or(NotSet, Set),
        group: body.group.map_or(NotSet, Set),
        description: body.description.map_or(NotSet, |v| Set(Some(v))),
        ..Default::default()
    }
    .update(get_db())
    .await?;
    let user = cds_db::transfer::User::from(user);

    Ok(WebResponse {
        code: StatusCode::OK.as_u16(),
        data: Some(user),
        ..Default::default()
    })
}

/// Delete a user with given data.
///
/// # Prerequisite
/// - Operator is admin.
pub async fn delete_user(
    Extension(ext): Extension<Ext>, Path(id): Path<i64>,
) -> Result<WebResponse<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    if operator.group != Group::Admin {
        return Err(WebError::Forbidden(json!("")));
    }

    let user = cds_db::entity::user::Entity::find_by_id(id)
        .filter(cds_db::entity::user::Column::DeletedAt.is_null())
        .one(get_db())
        .await?
        .ok_or(WebError::BadRequest("".into()))?;

    let _ = cds_db::entity::user::ActiveModel {
        id: Unchanged(id),
        username: Set(format!("[DELETED]_{}", user.username)),
        email: Set(format!("deleted_{}@del.cdsctf", user.email)),
        deleted_at: Set(Some(chrono::Utc::now().timestamp())),
        ..Default::default()
    }
    .update(get_db())
    .await?;

    Ok(WebResponse {
        code: StatusCode::OK.as_u16(),
        ..Default::default()
    })
}

pub async fn get_user_teams(
    Extension(ext): Extension<Ext>, Path(id): Path<i64>,
) -> Result<WebResponse<Vec<cds_db::transfer::Team>>, WebError> {
    let _ = ext.operator.ok_or(WebError::Unauthorized("".into()))?;

    let teams = cds_db::entity::team::Entity::find()
        .join(
            JoinType::InnerJoin,
            cds_db::entity::team_user::Relation::Team.def().rev(),
        )
        .filter(cds_db::entity::team_user::Column::UserId.eq(id))
        .all(get_db())
        .await?
        .into_iter()
        .map(|team| cds_db::transfer::Team::from(team))
        .collect::<Vec<cds_db::transfer::Team>>();

    let teams = cds_db::transfer::team::preload(teams).await?;

    Ok(WebResponse {
        code: StatusCode::OK.as_u16(),
        data: Some(teams),
        ..Default::default()
    })
}

pub async fn get_user_profile(
    Extension(ext): Extension<Ext>,
) -> Result<WebResponse<cds_db::transfer::User>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized("".into()))?;

    Ok(WebResponse {
        code: StatusCode::OK.as_u16(),
        data: Some(operator),
        ..Default::default()
    })
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
pub struct UpdateUserProfileRequest {
    pub nickname: Option<String>,
    #[validate(email)]
    pub email: Option<String>,
    pub password: Option<String>,
    pub description: Option<String>,
}

pub async fn update_user_profile(
    Extension(ext): Extension<Ext>, Json(mut body): Json<UpdateUserProfileRequest>,
) -> Result<WebResponse<cds_db::transfer::User>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized("".into()))?;

    if let Some(email) = body.email {
        body.email = Some(email.to_lowercase());
    }

    if let Some(password) = body.password {
        let hashed_password = Argon2::default()
            .hash_password(password.as_bytes(), &SaltString::generate(&mut OsRng))
            .unwrap()
            .to_string();
        body.password = Some(hashed_password);
    }

    let user = cds_db::entity::user::ActiveModel {
        id: Unchanged(operator.id),
        nickname: body.nickname.map_or(NotSet, Set),
        email: body.email.map_or(NotSet, Set),
        hashed_password: body.password.map_or(NotSet, Set),
        description: body.description.map_or(NotSet, |v| Set(Some(v))),
        ..Default::default()
    }
    .update(get_db())
    .await?;
    let user = cds_db::transfer::User::from(user);

    Ok(WebResponse {
        code: StatusCode::OK.as_u16(),
        data: Some(user),
        ..Default::default()
    })
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeleteUserProfileRequest {
    pub password: String,
    pub captcha: Option<cds_captcha::Answer>,
}

pub async fn delete_user_profile(
    Extension(ext): Extension<Ext>, Json(mut body): Json<DeleteUserProfileRequest>,
) -> Result<WebResponse<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized("".into()))?;

    if !cds_captcha::check(&cds_captcha::Answer {
        client_ip: Some(ext.client_ip),
        ..body.captcha.unwrap_or_default()
    })
    .await?
    {
        return Err(WebError::BadRequest(json!("captcha_invalid")));
    }

    let hashed_password = operator.hashed_password.clone();

    if Argon2::default()
        .verify_password(
            body.password.as_bytes(),
            &PasswordHash::new(&hashed_password).unwrap(),
        )
        .is_err()
    {
        return Err(WebError::BadRequest(json!("password_invalid")));
    }

    let _ = cds_db::entity::user::ActiveModel {
        id: Unchanged(operator.id),
        username: Set(format!("[DELETED]_{}", operator.username)),
        email: Set(format!("deleted_{}@del.cdsctf", operator.email)),
        deleted_at: Set(Some(chrono::Utc::now().timestamp())),
        ..Default::default()
    }
    .update(get_db())
    .await?;

    Ok(WebResponse {
        code: StatusCode::OK.as_u16(),
        ..Default::default()
    })
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserLoginRequest {
    pub account: String,
    pub password: String,
    pub captcha: Option<cds_captcha::Answer>,
}

pub async fn user_login(
    Extension(ext): Extension<Ext>, Json(mut body): Json<UserLoginRequest>,
) -> Result<impl IntoResponse, WebError> {
    if !cds_captcha::check(&cds_captcha::Answer {
        client_ip: Some(ext.client_ip),
        ..body.captcha.unwrap_or_default()
    })
    .await?
    {
        return Err(WebError::BadRequest(json!("captcha_invalid")));
    }

    body.account = body.account.to_lowercase();

    let user = cds_db::entity::user::Entity::find()
        .filter(
            Condition::any()
                .add(
                    Expr::expr(Func::lower(Expr::col(
                        cds_db::entity::user::Column::Username,
                    )))
                    .eq(body.account.clone()),
                )
                .add(
                    Expr::expr(Func::lower(Expr::col(cds_db::entity::user::Column::Email)))
                        .eq(body.account.clone()),
                ),
        )
        .filter(cds_db::entity::user::Column::DeletedAt.is_null())
        .one(get_db())
        .await?
        .ok_or(WebError::BadRequest(json!("invalid")))?;

    let mut user = cds_db::transfer::User::from(user);

    let hashed_password = user.hashed_password.clone();

    if Argon2::default()
        .verify_password(
            body.password.as_bytes(),
            &PasswordHash::new(&hashed_password).unwrap(),
        )
        .is_err()
    {
        return Err(WebError::BadRequest(json!("invalid")));
    }

    let token = jwt::generate_jwt_token(user.id).await;
    user.desensitize();

    let mut headers = HeaderMap::new();
    headers.insert(
        SET_COOKIE,
        format!(
            "token={}; Max-Age={}; Path=/; HttpOnly; SameSite=Strict",
            token,
            chrono::Duration::minutes(cds_config::get_config().auth.expiration).num_seconds()
        )
        .parse()
        .unwrap(),
    );

    Ok((StatusCode::OK, headers, WebResponse {
        code: StatusCode::OK.as_u16(),
        data: Some(user),
        ..Default::default()
    }))
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
pub struct UserRegisterRequest {
    #[validate(length(min = 3, max = 20))]
    pub username: String,
    pub nickname: String,
    #[validate(email)]
    pub email: String,
    pub password: String,
    pub captcha: Option<cds_captcha::Answer>,
}

pub async fn user_register(
    Extension(ext): Extension<Ext>, Json(mut body): Json<UserRegisterRequest>,
) -> Result<WebResponse<cds_db::transfer::User>, WebError> {
    if !cds_captcha::check(&cds_captcha::Answer {
        client_ip: Some(ext.client_ip),
        ..body.captcha.unwrap_or_default()
    })
    .await?
    {
        return Err(WebError::BadRequest(json!("captcha_invalid")));
    }

    body.email = body.email.to_lowercase();
    body.username = body.username.to_lowercase();

    let is_conflict = cds_db::entity::user::Entity::find()
        .filter(
            Condition::any()
                .add(
                    Expr::expr(Func::lower(Expr::col(
                        cds_db::entity::user::Column::Username,
                    )))
                    .eq(body.username.clone()),
                )
                .add(
                    Expr::expr(Func::lower(Expr::col(cds_db::entity::user::Column::Email)))
                        .eq(body.email.clone()),
                ),
        )
        .count(get_db())
        .await?
        > 0;

    if is_conflict {
        return Err(WebError::Conflict("".into()));
    }

    let hashed_password = Argon2::default()
        .hash_password(body.password.as_bytes(), &SaltString::generate(&mut OsRng))
        .unwrap()
        .to_string();

    let user = cds_db::entity::user::ActiveModel {
        username: Set(body.username),
        nickname: Set(body.nickname),
        email: Set(body.email),
        hashed_password: Set(hashed_password),
        group: Set(Group::User),
        ..Default::default()
    }
    .insert(get_db())
    .await?;
    let user = cds_db::transfer::User::from(user);

    Ok(WebResponse {
        code: StatusCode::OK.as_u16(),
        data: Some(user),
        ..Default::default()
    })
}

pub async fn get_user_avatar(Path(id): Path<i64>) -> Result<impl IntoResponse, WebError> {
    let path = format!("users/{}/avatar", id);

    util::media::get_img(path).await
}

pub async fn get_user_avatar_metadata(
    Path(id): Path<i64>,
) -> Result<WebResponse<Metadata>, WebError> {
    let path = format!("users/{}/avatar", id);

    util::media::get_img_metadata(path).await
}

pub async fn save_user_avatar(
    Extension(ext): Extension<Ext>, Path(id): Path<i64>, multipart: Multipart,
) -> Result<WebResponse<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized("".into()))?;
    if operator.group != Group::Admin && operator.id != id {
        return Err(WebError::Forbidden("".into()));
    }

    let path = format!("users/{}/avatar", id);

    util::media::save_img(path, multipart).await
}

pub async fn delete_user_avatar(
    Extension(ext): Extension<Ext>, Path(id): Path<i64>,
) -> Result<WebResponse<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized("".into()))?;
    if operator.group != Group::Admin && operator.id != id {
        return Err(WebError::Forbidden("".into()));
    }

    let path = format!("users/{}/avatar", id);

    util::media::delete_img(path).await
}
