mod team_id;

use std::str::FromStr;

use argon2::{
    Argon2, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};
use axum::{Router, http::StatusCode, response::IntoResponse};
use cds_db::{entity::user::Group, get_db};
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set},
    ColumnTrait, EntityTrait, JoinType, Order, PaginatorTrait, QueryFilter, QueryOrder,
    QuerySelect, RelationTrait,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use validator::Validate;

use crate::{
    extract::{Extension, Query, VJson},
    traits::{Ext, WebError, WebResponse},
};

pub fn router() -> Router {
    Router::new()
        .route("/", axum::routing::get(get_team))
        .route("/", axum::routing::post(create_team))
        .route("/register", axum::routing::post(team_register))
        .nest("/{team_id}", team_id::router())
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct GetTeamRequest {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub email: Option<String>,

    /// The user id of expected teams.
    ///
    /// `user_id` is not in table `teams`, so it relies on JOIN queries.
    ///
    /// ```sql
    /// SELECT *
    /// FROM "teams"
    ///     INNER JOIN "team_users" ON "teams"."id" = "team_users"."team_id"
    /// WHERE "team_users"."user_id" = ?;
    /// ```
    pub user_id: Option<i64>,

    pub page: Option<u64>,
    pub size: Option<u64>,
    pub sorts: Option<String>,
}

/// Get teams by given params.
pub async fn get_team(
    Query(params): Query<GetTeamRequest>,
) -> Result<WebResponse<Vec<cds_db::transfer::Team>>, WebError> {
    let mut sql = cds_db::entity::team::Entity::find();

    if let Some(id) = params.id {
        sql = sql.filter(cds_db::entity::team::Column::Id.eq(id));
    }

    if let Some(name) = params.name {
        sql = sql.filter(cds_db::entity::team::Column::Name.contains(name));
    }

    if let Some(email) = params.email {
        sql = sql.filter(cds_db::entity::team::Column::Email.eq(email));
    }

    if let Some(user_id) = params.user_id {
        sql = sql
            .join(
                JoinType::InnerJoin,
                cds_db::entity::team_user::Relation::Team.def().rev(),
            )
            .filter(cds_db::entity::team_user::Column::UserId.eq(user_id))
    }

    // Exclude teams which has been deleted.
    sql = sql.filter(cds_db::entity::team::Column::DeletedAt.is_null());

    let total = sql.clone().count(get_db()).await?;

    // Sort according to the `sorts` parameter.
    if let Some(sorts) = params.sorts {
        let sorts = sorts.split(",").collect::<Vec<&str>>();
        for sort in sorts {
            let col = match cds_db::entity::team::Column::from_str(sort.replace("-", "").as_str()) {
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

    // Paginate according to the `page` and `size` parameters.
    if let (Some(page), Some(size)) = (params.page, params.size) {
        let offset = (page - 1) * size;
        sql = sql.offset(offset).limit(size);
    }

    let mut teams = sql
        .all(get_db())
        .await?
        .into_iter()
        .map(cds_db::transfer::Team::from)
        .collect::<Vec<cds_db::transfer::Team>>();

    teams = cds_db::transfer::team::preload(teams).await?;

    Ok(WebResponse {
        code: StatusCode::OK,
        data: Some(teams),
        total: Some(total),
        ..Default::default()
    })
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
pub struct CreateTeamRequest {
    pub name: String,
    pub email: String,
    pub password: String,
    pub slogan: Option<String>,
    pub description: Option<String>,
}

/// Create a team by given data.
///
/// Unlike the `register` function,
/// no users will be added to the newly created team.
///
/// # Prerequisite
/// - Operator is admin.
pub async fn create_team(
    Extension(ext): Extension<Ext>, VJson(body): VJson<CreateTeamRequest>,
) -> Result<WebResponse<cds_db::transfer::Team>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;

    if operator.group != Group::Admin {
        return Err(WebError::Forbidden(json!("")));
    }

    let hashed_password = Argon2::default()
        .hash_password(body.password.as_bytes(), &SaltString::generate(&mut OsRng))
        .unwrap()
        .to_string();

    let team = cds_db::entity::team::ActiveModel {
        name: Set(body.name),
        email: Set(body.email),
        hashed_password: Set(hashed_password),
        slogan: body.slogan.map_or(NotSet, |v| Set(Some(v))),
        description: body.description.map_or(NotSet, |v| Set(Some(v))),
        ..Default::default()
    }
    .insert(get_db())
    .await?;

    let team = cds_db::transfer::Team::from(team);

    Ok(WebResponse {
        code: StatusCode::OK,
        data: Some(team),
        ..Default::default()
    })
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
pub struct TeamRegisterRequest {
    pub name: String,
    pub email: String,
    pub password: String,
}

/// Register a team by given data.
///
/// The operator of this function will be added into the newly created team.
/// So you should call this function in general teams page, not in admin panel.
pub async fn team_register(
    Extension(ext): Extension<Ext>, VJson(body): VJson<TeamRegisterRequest>,
) -> Result<WebResponse<cds_db::transfer::Team>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;

    let hashed_password = Argon2::default()
        .hash_password(body.password.as_bytes(), &SaltString::generate(&mut OsRng))
        .unwrap()
        .to_string();

    let team = cds_db::entity::team::ActiveModel {
        name: Set(body.name),
        email: Set(body.email),
        hashed_password: Set(hashed_password),
        ..Default::default()
    }
    .insert(get_db())
    .await?;

    let _ = cds_db::entity::team_user::ActiveModel {
        user_id: Set(operator.id),
        team_id: Set(team.id.clone()),
        ..Default::default()
    }
    .insert(get_db())
    .await?;

    let team = cds_db::entity::team::Entity::find_by_id(team.id)
        .one(get_db())
        .await?
        .map(|team| cds_db::transfer::Team::from(team))
        .unwrap();

    Ok(WebResponse {
        code: StatusCode::OK,
        data: Some(team),
        ..Default::default()
    })
}
