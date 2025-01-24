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
    ActiveValue::{NotSet, Set},
    Condition, EntityTrait, PaginatorTrait, QueryFilter,
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
        .route("/", axum::routing::get(get))
        .route("/", axum::routing::post(create))
        .route("/{id}", axum::routing::put(update))
        .route("/{id}", axum::routing::delete(delete))
        .route("/{id}/teams", axum::routing::get(get_teams))
        .route("/login", axum::routing::post(login))
        .route("/register", axum::routing::post(register))
        .route("/{id}/avatar", axum::routing::get(get_avatar))
        .route(
            "/{id}/avatar/metadata",
            axum::routing::get(get_avatar_metadata),
        )
        .route(
            "/{id}/avatar",
            axum::routing::post(save_avatar)
                .layer(DefaultBodyLimit::max(3 * 1024 * 1024 /* MB */)),
        )
        .route("/{id}/avatar", axum::routing::delete(delete_avatar))
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetRequest {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub email: Option<String>,
    pub group: Option<String>,
    pub page: Option<u64>,
    pub size: Option<u64>,
}

pub async fn get(
    Query(params): Query<GetRequest>,
) -> Result<WebResponse<Vec<cds_db::transfer::User>>, WebError> {
    let (mut users, total) = cds_db::transfer::user::find(
        params.id,
        params.name,
        None,
        params.group,
        params.email,
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
        ..WebResponse::default()
    })
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
pub struct CreateRequest {
    pub username: String,
    pub nickname: String,
    pub email: String,
    pub password: String,
    pub group: Group,
}

pub async fn create(
    Extension(ext): Extension<Ext>, VJson(mut body): VJson<CreateRequest>,
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
        ..WebResponse::default()
    })
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
pub struct UpdateRequest {
    pub id: Option<i64>,
    #[validate(length(min = 3, max = 20))]
    pub username: Option<String>,
    pub nickname: Option<String>,
    #[validate(email)]
    pub email: Option<String>,
    pub password: Option<String>,
    pub group: Option<Group>,
}

pub async fn update(
    Extension(ext): Extension<Ext>, Path(id): Path<i64>, VJson(mut body): VJson<UpdateRequest>,
) -> Result<WebResponse<cds_db::transfer::User>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    body.id = Some(id);
    if !(operator.group == Group::Admin
        || (operator.id == body.id.unwrap_or(0)
            && (body.group.clone().is_none() || operator.group == body.group.clone().unwrap())))
    {
        return Err(WebError::Forbidden(json!("")));
    }

    if let Some(password) = body.password {
        let hashed_password = Argon2::default()
            .hash_password(password.as_bytes(), &SaltString::generate(&mut OsRng))
            .unwrap()
            .to_string();
        body.password = Some(hashed_password);
    }

    let user = cds_db::entity::user::ActiveModel {
        id: Set(body.id.unwrap_or(0)),
        username: body.username.map_or(NotSet, Set),
        nickname: body.nickname.map_or(NotSet, Set),
        email: body.email.map_or(NotSet, Set),
        hashed_password: body.password.map_or(NotSet, Set),
        group: body.group.map_or(NotSet, Set),
        ..Default::default()
    }
    .update(get_db())
    .await?;
    let user = cds_db::transfer::User::from(user);

    Ok(WebResponse {
        code: StatusCode::OK.as_u16(),
        data: Some(user),
        ..WebResponse::default()
    })
}

pub async fn delete(
    Extension(ext): Extension<Ext>, Path(id): Path<i64>,
) -> Result<WebResponse<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    if !(operator.group == Group::Admin || operator.id == id) {
        return Err(WebError::Forbidden(json!("")));
    }

    let _ = cds_db::entity::user::ActiveModel {
        id: Set(id),
        is_deleted: Set(true),
        ..Default::default()
    }
    .update(get_db())
    .await?;

    Ok(WebResponse {
        code: StatusCode::OK.as_u16(),
        ..WebResponse::default()
    })
}

pub async fn get_teams(
    Extension(ext): Extension<Ext>, Path(id): Path<i64>,
) -> Result<WebResponse<Vec<cds_db::transfer::Team>>, WebError> {
    let _ = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;

    let teams = cds_db::transfer::team::find_by_user_id(id).await?;

    Ok(WebResponse {
        code: StatusCode::OK.as_u16(),
        data: Some(teams),
        ..WebResponse::default()
    })
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub account: String,
    pub password: String,
}

pub async fn login(Json(mut body): Json<LoginRequest>) -> Result<impl IntoResponse, WebError> {
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
        .one(get_db())
        .await?
        .ok_or_else(|| WebError::BadRequest(json!("invalid")))?;

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
            chrono::Duration::minutes(jwt::get_jwt_config().await.expiration)
                .num_seconds()
        )
        .parse()
        .unwrap(),
    );

    Ok((StatusCode::OK, headers, WebResponse {
        code: StatusCode::OK.as_u16(),
        data: Some(user),
        ..WebResponse::default()
    }))
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(length(min = 3, max = 20))]
    pub username: String,
    pub nickname: String,
    #[validate(email)]
    pub email: String,
    pub password: String,
    pub token: Option<String>,
}

pub async fn register(
    Extension(ext): Extension<Ext>, Json(mut body): Json<RegisterRequest>,
) -> Result<WebResponse<cds_db::transfer::User>, WebError> {
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
        return Err(WebError::Conflict(json!("")));
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
        ..WebResponse::default()
    })
}

pub async fn get_avatar(Path(id): Path<i64>) -> Result<impl IntoResponse, WebError> {
    let path = format!("users/{}/avatar", id);

    util::media::get_img(path).await
}

pub async fn get_avatar_metadata(Path(id): Path<i64>) -> Result<WebResponse<Metadata>, WebError> {
    let path = format!("users/{}/avatar", id);

    util::media::get_img_metadata(path).await
}

pub async fn save_avatar(
    Extension(ext): Extension<Ext>, Path(id): Path<i64>, multipart: Multipart,
) -> Result<WebResponse<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    if operator.group != Group::Admin && operator.id != id {
        return Err(WebError::Forbidden(json!("")));
    }

    let path = format!("users/{}/avatar", id);

    util::media::save_img(path, multipart).await
}

pub async fn delete_avatar(
    Extension(ext): Extension<Ext>, Path(id): Path<i64>,
) -> Result<WebResponse<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    if operator.group != Group::Admin && operator.id != id {
        return Err(WebError::Forbidden(json!("")));
    }

    let path = format!("users/{}/avatar", id);

    util::media::delete_img(path).await
}
