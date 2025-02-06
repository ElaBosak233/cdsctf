use std::str::FromStr;

use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};
use axum::{
    Router,
    extract::{DefaultBodyLimit, Multipart},
    http::StatusCode,
    response::IntoResponse,
};
use cds_db::{entity::user::Group, get_db};
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set, Unchanged},
    ColumnTrait, EntityTrait, JoinType, Order, PaginatorTrait, QueryFilter, QueryOrder,
    QuerySelect, RelationTrait,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use validator::Validate;

use crate::{
    extract::{Extension, Json, Path, Query, VJson},
    model::Metadata,
    traits::{Ext, WebError, WebResponse},
    util,
};

pub fn router() -> Router {
    Router::new()
        .route("/", axum::routing::get(get_team))
        .route("/", axum::routing::post(create_team))
        .route("/register", axum::routing::post(team_register))
        .route("/{id}", axum::routing::put(update_team))
        .route("/{id}", axum::routing::delete(delete_team))
        .route("/{id}/users", axum::routing::post(create_team_user))
        .route(
            "/{id}/users/{user_id}",
            axum::routing::delete(delete_team_user),
        )
        .route("/{id}/join", axum::routing::post(join_team))
        .route("/{id}/quit", axum::routing::delete(quit_team))
        .route("/{id}/avatar", axum::routing::get(get_team_avatar))
        .route(
            "/{id}/avatar/metadata",
            axum::routing::get(get_team_avatar_metadata),
        )
        .route(
            "/{id}/avatar",
            axum::routing::post(save_team_avatar)
                .layer(DefaultBodyLimit::max(3 * 1024 * 1024 /* MB */)),
        )
        .route("/{id}/avatar", axum::routing::delete(delete_team_avatar))
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
        code: StatusCode::OK.as_u16(),
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
        code: StatusCode::OK.as_u16(),
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
        code: StatusCode::OK.as_u16(),
        data: Some(team),
        ..Default::default()
    })
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
pub struct UpdateTeamRequest {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub slogan: Option<String>,
    pub description: Option<String>,
}

/// Update a team by given data.
///
/// # Prerequisite
/// - Operator is admin or the members of current team.
pub async fn update_team(
    Extension(ext): Extension<Ext>, Path(id): Path<i64>, VJson(mut body): VJson<UpdateTeamRequest>,
) -> Result<WebResponse<cds_db::transfer::Team>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    let team = cds_db::transfer::Team::from(
        cds_db::entity::team::Entity::find_by_id(id)
            .filter(cds_db::entity::team::Column::DeletedAt.is_null())
            .one(get_db())
            .await?
            .ok_or(WebError::BadRequest(json!("team_not_found")))?,
    );

    if !cds_db::util::can_user_modify_team(&operator, &team) {
        return Err(WebError::Forbidden(json!("")));
    }
    body.id = Some(id);

    if let Some(password) = body.password {
        let hashed_password = Argon2::default()
            .hash_password(password.as_bytes(), &SaltString::generate(&mut OsRng))
            .unwrap()
            .to_string();
        body.password = Some(hashed_password);
    }

    let team = cds_db::entity::team::ActiveModel {
        id: Unchanged(team.id),
        name: body.name.map_or(NotSet, Set),
        email: body.email.map_or(NotSet, Set),
        hashed_password: body.password.map_or(NotSet, Set),
        slogan: body.slogan.map_or(NotSet, |v| Set(Some(v))),
        description: body.description.map_or(NotSet, |v| Set(Some(v))),
        ..Default::default()
    }
    .update(get_db())
    .await?;
    let team = cds_db::transfer::Team::from(team);

    Ok(WebResponse {
        code: StatusCode::OK.as_u16(),
        data: Some(team),
        ..Default::default()
    })
}

/// Delete a team by `id`.
///
/// The team won't be permanently deleted.
/// For safety, it's marked as deleted using the `is_deleted` field.
///
/// # Prerequisite
/// - Operator is admin or the members of current team.
pub async fn delete_team(
    Extension(ext): Extension<Ext>, Path(id): Path<i64>,
) -> Result<WebResponse<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    let team = cds_db::transfer::Team::from(
        cds_db::entity::team::Entity::find_by_id(id)
            .filter(cds_db::entity::team::Column::DeletedAt.is_null())
            .one(get_db())
            .await?
            .ok_or(WebError::BadRequest(json!("team_not_found")))?,
    );

    if !cds_db::util::can_user_modify_team(&operator, &team) {
        return Err(WebError::Forbidden(json!("")));
    }

    let _ = cds_db::entity::team::ActiveModel {
        id: Set(id),
        is_locked: Set(true),
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
pub struct CreateTeamUserRequest {
    pub user_id: i64,
    pub team_id: i64,
}

/// Add a user into a team by given data.
///
/// Only admins can use this function.
///
/// # Prerequisite
/// - Operator is admin.
pub async fn create_team_user(
    Extension(ext): Extension<Ext>, Json(body): Json<CreateTeamUserRequest>,
) -> Result<WebResponse<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    let team = cds_db::transfer::Team::from(
        cds_db::entity::team::Entity::find_by_id(body.team_id)
            .filter(cds_db::entity::team::Column::DeletedAt.is_null())
            .one(get_db())
            .await?
            .ok_or(WebError::BadRequest(json!("team_not_found")))?,
    );

    if operator.group != Group::Admin {
        return Err(WebError::Forbidden(json!("")));
    }

    let _ = cds_db::entity::team_user::ActiveModel {
        user_id: Set(body.user_id),
        team_id: Set(team.id),
    }
    .insert(get_db())
    .await?;

    Ok(WebResponse {
        code: StatusCode::OK.as_u16(),
        ..Default::default()
    })
}

/// Kick a user from a team by `id` and `user_id`.
///
/// # Prerequisite
/// - Operator is admin.
pub async fn delete_team_user(
    Extension(ext): Extension<Ext>, Path((id, user_id)): Path<(i64, i64)>,
) -> Result<WebResponse<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    let team = cds_db::transfer::Team::from(
        cds_db::entity::team::Entity::find_by_id(id)
            .filter(cds_db::entity::team::Column::DeletedAt.is_null())
            .one(get_db())
            .await?
            .ok_or(WebError::BadRequest(json!("team_not_found")))?,
    );

    if operator.group != Group::Admin || team.deleted_at.is_some() {
        return Err(WebError::Forbidden(json!("")));
    }

    let _ = cds_db::entity::team_user::Entity::delete_many()
        .filter(cds_db::entity::team_user::Column::UserId.eq(user_id))
        .filter(cds_db::entity::team_user::Column::TeamId.eq(id))
        .exec(get_db())
        .await?;

    Ok(WebResponse {
        code: StatusCode::OK.as_u16(),
        ..Default::default()
    })
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JoinTeamRequest {
    pub user_id: i64,
    pub team_id: i64,
    pub password: String,
}

/// Join a team by given data.
///
/// The field `user_id` will be overwritten by operator's id.
pub async fn join_team(
    Extension(ext): Extension<Ext>, Path(id): Path<i64>, Json(mut body): Json<JoinTeamRequest>,
) -> Result<WebResponse<cds_db::transfer::TeamUser>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    let team = cds_db::entity::team::Entity::find_by_id(id)
        .filter(cds_db::entity::team::Column::DeletedAt.is_null())
        .one(get_db())
        .await?
        .ok_or(WebError::BadRequest(json!("team_not_found")))?;

    body.user_id = operator.id;

    if Argon2::default()
        .verify_password(
            body.password.as_bytes(),
            &PasswordHash::new(&team.hashed_password).unwrap(),
        )
        .is_err()
    {
        return Err(WebError::BadRequest(json!("invalid_password")));
    }

    let team_user = cds_db::entity::team_user::ActiveModel {
        user_id: Set(body.user_id),
        team_id: Set(id),
        ..Default::default()
    }
    .insert(get_db())
    .await?;
    let team_user = cds_db::transfer::TeamUser::from(team_user);

    Ok(WebResponse {
        code: StatusCode::OK.as_u16(),
        data: Some(team_user),
        ..Default::default()
    })
}

/// Quit a team by `id`.
///
/// Remove the operator from the current team.
pub async fn quit_team(
    Extension(ext): Extension<Ext>, Path(id): Path<i64>,
) -> Result<WebResponse<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    let team = cds_db::entity::team::Entity::find_by_id(id)
        .filter(cds_db::entity::team::Column::DeletedAt.is_null())
        .one(get_db())
        .await?
        .ok_or(WebError::BadRequest(json!("team_not_found")))?;

    if cds_db::entity::team_user::Entity::find()
        .filter(cds_db::entity::team_user::Column::TeamId.eq(team.id))
        .count(get_db())
        .await?
        == 1
    {
        return Err(WebError::BadRequest(json!("delete_instead_of_leave")));
    }

    let _ = cds_db::entity::team_user::Entity::delete_many()
        .filter(cds_db::entity::team_user::Column::UserId.eq(operator.id))
        .filter(cds_db::entity::team_user::Column::TeamId.eq(team.id))
        .exec(get_db())
        .await?;

    Ok(WebResponse {
        code: StatusCode::OK.as_u16(),
        ..Default::default()
    })
}

pub async fn get_team_avatar(Path(id): Path<i64>) -> Result<impl IntoResponse, WebError> {
    let path = format!("teams/{}/avatar", id);

    util::media::get_img(path).await
}

pub async fn get_team_avatar_metadata(
    Path(id): Path<i64>,
) -> Result<WebResponse<Metadata>, WebError> {
    let path = format!("teams/{}/avatar", id);

    util::media::get_img_metadata(path).await
}

/// Save an avatar for the team.
///
/// # Prerequisite
/// - Operator is admin or the members of current team.
pub async fn save_team_avatar(
    Extension(ext): Extension<Ext>, Path(id): Path<i64>, multipart: Multipart,
) -> Result<WebResponse<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    let team = cds_db::transfer::Team::from(
        cds_db::entity::team::Entity::find_by_id(id)
            .filter(cds_db::entity::team::Column::DeletedAt.is_null())
            .one(get_db())
            .await?
            .ok_or(WebError::BadRequest(json!("team_not_found")))?,
    );

    if !cds_db::util::can_user_modify_team(&operator, &team) {
        return Err(WebError::Forbidden(json!("")));
    }

    let path = format!("teams/{}/avatar", id);

    util::media::save_img(path, multipart).await
}

/// Delete avatar for the team.
///
/// # Prerequisite
/// - Operator is admin or the members of current team.
pub async fn delete_team_avatar(
    Extension(ext): Extension<Ext>, Path(id): Path<i64>,
) -> Result<WebResponse<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    let team = cds_db::transfer::Team::from(
        cds_db::entity::team::Entity::find_by_id(id)
            .filter(cds_db::entity::team::Column::DeletedAt.is_null())
            .one(get_db())
            .await?
            .ok_or(WebError::BadRequest(json!("team_not_found")))?,
    );

    if !cds_db::util::can_user_modify_team(&operator, &team) {
        return Err(WebError::Forbidden(json!("")));
    }

    let path = format!("teams/{}/avatar", id);

    util::media::delete_img(path).await
}
