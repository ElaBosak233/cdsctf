mod team_id;

use axum::{Router, http::StatusCode};
use cds_db::{
    entity::{team::State, user::Group},
    get_db,
    transfer::Team,
};
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, JoinType, PaginatorTrait,
    QueryFilter, QuerySelect, RelationTrait,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    extract::{Extension, Json, Path, Query},
    traits::{Ext, WebError, WebResponse},
};

pub fn router() -> Router {
    Router::new()
        .route("/", axum::routing::get(get_team))
        .route("/", axum::routing::post(create_team))
        .route("/register", axum::routing::post(team_register))
        .nest("/{team_id}", team_id::router())
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetGameTeamRequest {
    /// The team id of expected game teams.
    pub id: Option<i64>,

    /// The user id of expected game teams.
    ///
    /// `user_id` is not in table `teams`, so it relies on JOIN queries.
    /// Essentially, it is unrelated to game team.
    ///
    /// ```sql
    /// SELECT *
    /// FROM "teams"
    ///     INNER JOIN "team_users" ON "teams"."id" = "team_users"."team_id"
    /// WHERE "team_users"."game_id" = ? AND "team_users"."user_id" = ?;
    /// ```
    pub user_id: Option<i64>,
}

/// Get game teams with given data.
pub async fn get_team(
    Extension(ext): Extension<Ext>, Path(game_id): Path<i64>,
    Query(params): Query<GetGameTeamRequest>,
) -> Result<WebResponse<Vec<Team>>, WebError> {
    let _ = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;

    let mut sql = cds_db::entity::team::Entity::find();

    sql = sql.filter(cds_db::entity::team::Column::GameId.eq(game_id));

    if let Some(id) = params.id {
        sql = sql.filter(cds_db::entity::team::Column::Id.eq(id));
    }

    if let Some(user_id) = params.user_id {
        // If you are a little confused about the following statement,
        // you can refer to the comments on the field `user_id` in `GetTeamRequest`
        sql = sql
            .join(
                JoinType::InnerJoin,
                cds_db::entity::team_user::Relation::Team.def().rev(),
            )
            .filter(cds_db::entity::team_user::Column::UserId.eq(user_id))
    }

    let total = sql.clone().count(get_db()).await?;

    let teams = sql.all(get_db()).await?;
    let mut teams = teams
        .into_iter()
        .map(|team| Team::from(team))
        .collect::<Vec<Team>>();

    teams = cds_db::transfer::team::preload(teams).await?;

    Ok(WebResponse {
        code: StatusCode::OK,
        data: Some(teams),
        total: Some(total),
        ..Default::default()
    })
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateTeamRequest {
    pub name: String,
    pub email: Option<String>,
    pub slogan: Option<String>,
    pub description: Option<String>,
}

/// Add a team to a game with given path and data.
///
/// # Prerequisite
/// - Operator is admin.
pub async fn create_team(
    Extension(ext): Extension<Ext>, Path(game_id): Path<i64>, Json(body): Json<CreateTeamRequest>,
) -> Result<WebResponse<Team>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;

    if operator.group != Group::Admin {
        return Err(WebError::Forbidden(json!("")));
    }

    let game = cds_db::entity::game::Entity::find_by_id(game_id)
        .one(get_db())
        .await?
        .map(|game| cds_db::transfer::Game::from(game))
        .ok_or(WebError::BadRequest(json!("game_not_found")))?;

    let team = cds_db::entity::team::ActiveModel {
        name: Set(body.name),
        email: Set(body.email),
        slogan: Set(body.slogan),
        description: Set(body.description),
        game_id: Set(game.id),
        state: Set(State::Passed),
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TeamRegisterRequest {
    pub name: String,
    pub email: Option<String>,
    pub slogan: Option<String>,
    pub description: Option<String>,
}

/// Add a team to a game with given path and data.
///
/// # Prerequisite
/// - No user in the team is already in the game.
pub async fn team_register(
    Extension(ext): Extension<Ext>, Path(game_id): Path<i64>, Json(body): Json<CreateTeamRequest>,
) -> Result<WebResponse<Team>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;

    let game = cds_db::entity::game::Entity::find_by_id(game_id)
        .one(get_db())
        .await?
        .map(|game| cds_db::transfer::Game::from(game))
        .ok_or(WebError::BadRequest(json!("game_not_found")))?;

    if cds_db::util::is_user_in_game(&operator, &game, None).await? {
        return Err(WebError::BadRequest(json!(
            "one_user_in_team_already_in_game"
        )));
    }

    let team = cds_db::entity::team::ActiveModel {
        name: Set(body.name),
        email: Set(body.email),
        slogan: Set(body.slogan),
        description: Set(body.description),
        game_id: Set(game.id),
        state: Set(if game.is_public {
            State::Passed
        } else {
            State::Pending
        }),
        ..Default::default()
    }
    .insert(get_db())
    .await?;

    let _ = cds_db::entity::team_user::ActiveModel {
        team_id: Set(team.id),
        user_id: Set(operator.id),
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
