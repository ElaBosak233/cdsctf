mod user_id;

use std::str::FromStr;

use argon2::{
    Argon2, PasswordHasher,
    password_hash::{SaltString, rand_core::OsRng},
};
use axum::{Router, http::StatusCode};
use cds_db::{
    entity::user::Group,
    get_db,
    sea_orm::{
        ActiveModelTrait, ActiveValue::Set, ColumnTrait, Condition, EntityTrait, Order,
        PaginatorTrait, QueryFilter, QueryOrder, QuerySelect,
    },
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use validator::Validate;

use crate::{
    extract::{Query, VJson},
    model::user::User,
    traits::{WebError, WebResponse},
};

pub fn router() -> Router {
    Router::new()
        .route("/", axum::routing::get(get_user))
        .route("/", axum::routing::post(create_user))
        .nest("/{user_id}", user_id::router())
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
) -> Result<WebResponse<Vec<User>>, WebError> {
    let page = params.page.unwrap_or(1);
    let size = params.size.unwrap_or(10).min(100);

    let mut sql = cds_db::entity::user::Entity::find();

    if let Some(id) = params.id {
        sql = sql.filter(cds_db::entity::user::Column::Id.eq(id));
    }

    if let Some(name) = params.name {
        let pattern = format!("%{}%", name);
        let condition = Condition::any()
            .add(cds_db::entity::user::Column::Username.like(&pattern))
            .add(cds_db::entity::user::Column::Name.like(&pattern));
        sql = sql.filter(condition);
    }

    if let Some(group) = params.group {
        sql = sql.filter(cds_db::entity::user::Column::Group.eq(group));
    }

    if let Some(email) = params.email {
        sql = sql.filter(cds_db::entity::user::Column::Email.eq(email));
    }

    sql = sql.filter(cds_db::entity::user::Column::DeletedAt.is_null());

    let total = sql.clone().count(get_db()).await?;

    if let Some(sorts) = params.sorts {
        let sorts = sorts.split(",").collect::<Vec<&str>>();
        for sort in sorts {
            let col = match cds_db::entity::user::Column::from_str(sort.replace("-", "").as_str()) {
                Ok(col) => col,
                Err(_) => continue,
            };
            if sort.starts_with("-") {
                sql = sql.order_by(col, Order::Desc);
            } else {
                sql = sql.order_by(col, Order::Asc);
            }
        }
    }

    let offset = (page - 1) * size;
    sql = sql.offset(offset).limit(size);

    let users = sql.into_model::<User>().all(get_db()).await?;

    Ok(WebResponse {
        code: StatusCode::OK,
        data: Some(users),
        total: Some(total),
        ..Default::default()
    })
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
pub struct CreateUserRequest {
    pub name: String,
    #[validate(length(min = 3, max = 20))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    pub password: String,
    pub group: Group,
}

pub async fn create_user(
    VJson(mut body): VJson<CreateUserRequest>,
) -> Result<WebResponse<User>, WebError> {
    body.email = body.email.to_lowercase();
    if !cds_db::util::is_user_email_unique(0, &body.email).await? {
        return Err(WebError::Conflict(json!("email_already_exists")));
    }

    body.username = body.username.to_lowercase();
    if !cds_db::util::is_user_username_unique(0, &body.username).await? {
        return Err(WebError::Conflict(json!("username_already_exists")));
    }

    let hashed_password = Argon2::default()
        .hash_password(body.password.as_bytes(), &SaltString::generate(&mut OsRng))
        .unwrap()
        .to_string();

    let user = cds_db::entity::user::ActiveModel {
        name: Set(body.name),
        username: Set(body.username),
        email: Set(body.email),
        is_verified: Set(true),
        hashed_password: Set(hashed_password),
        group: Set(body.group),
        ..Default::default()
    }
    .insert(get_db())
    .await?;

    let user = crate::util::loader::prepare_user(user.id).await?;

    Ok(WebResponse {
        code: StatusCode::OK,
        data: Some(user),
        ..Default::default()
    })
}
