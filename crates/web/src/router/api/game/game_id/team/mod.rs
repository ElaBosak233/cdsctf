mod team_id;

use axum::{Router, http::StatusCode};
use cds_db::{get_db, transfer::GameTeam};
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
        .route("/", axum::routing::get(get_game_team))
        .route("/", axum::routing::post(create_game_team))
        .nest("/{team_id}", team_id::router())
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetGameTeamRequest {
    /// The game id of expected game teams.
    ///
    /// It will be overwritten by `id` in path.
    pub game_id: Option<i64>,

    /// The team id of expected game teams.
    pub team_id: Option<i64>,

    /// The user id of expected game teams.
    ///
    /// `user_id` is not in table `game_teams`, so it relies on JOIN queries.
    /// Essentially, it is unrelated to game team.
    ///
    /// ```sql
    /// SELECT *
    /// FROM "game_teams"
    ///     INNER JOIN "teams" ON "game_teams"."team_id" = "teams"."id"
    ///     INNER JOIN "team_users" ON "teams"."id" = "team_users"."team_id"
    /// WHERE "game_teams"."game_id" = ? AND "team_users"."user_id" = ?;
    /// ```
    pub user_id: Option<i64>,
}

/// Get game teams with given data.
pub async fn get_game_team(
    Extension(ext): Extension<Ext>, Path(game_id): Path<i64>,
    Query(params): Query<GetGameTeamRequest>,
) -> Result<WebResponse<Vec<GameTeam>>, WebError> {
    let _ = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;

    let mut sql = cds_db::entity::game_team::Entity::find();

    sql = sql.filter(cds_db::entity::game_team::Column::GameId.eq(game_id));

    if let Some(team_id) = params.team_id {
        sql = sql.filter(cds_db::entity::game_team::Column::TeamId.eq(team_id));
    }

    if let Some(user_id) = params.user_id {
        // If you are a little confused about the following statement,
        // you can refer to the comments on the field `user_id` in `GetTeamRequest`
        sql = sql
            .join(
                JoinType::InnerJoin,
                cds_db::entity::game_team::Relation::Team.def(),
            )
            .join(
                JoinType::InnerJoin,
                cds_db::entity::team_user::Relation::Team.def().rev(),
            )
            .filter(cds_db::entity::team_user::Column::UserId.eq(user_id))
    }

    let total = sql.clone().count(get_db()).await?;

    let game_teams = sql.all(get_db()).await?;
    let mut game_teams = game_teams
        .into_iter()
        .map(GameTeam::from)
        .collect::<Vec<GameTeam>>();

    game_teams = cds_db::transfer::game_team::preload(game_teams).await?;

    Ok(WebResponse {
        code: StatusCode::OK,
        data: Some(game_teams),
        total: Some(total),
        ..Default::default()
    })
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateGameTeamRequest {
    pub game_id: i64,
    pub team_id: i64,
}

/// Add a team to a game with given path and data.
///
/// # Prerequisite
/// - Operator is admin or one of the current team's members.
/// - No user in the team is already in the game.
pub async fn create_game_team(
    Extension(ext): Extension<Ext>, Path(game_id): Path<i64>,
    Json(body): Json<CreateGameTeamRequest>,
) -> Result<WebResponse<GameTeam>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;

    let game = cds_db::entity::game::Entity::find_by_id(game_id)
        .one(get_db())
        .await?
        .map(|game| cds_db::transfer::Game::from(game))
        .ok_or(WebError::BadRequest(json!("game_not_found")))?;

    let team = cds_db::entity::team::Entity::find_by_id(body.team_id)
        .one(get_db())
        .await?
        .map(|team| cds_db::transfer::Team::from(team))
        .ok_or(WebError::BadRequest(json!("team_not_found")))?;

    if !cds_db::util::can_user_modify_team(&operator, &team) {
        return Err(WebError::Forbidden(json!("")));
    }

    if cds_db::util::is_user_in_game(&operator, &game, None).await? {
        return Err(WebError::BadRequest(json!(
            "one_user_in_team_already_in_game"
        )));
    }

    let game_team = cds_db::entity::game_team::ActiveModel {
        game_id: Set(game.id),
        team_id: Set(team.id),
        is_allowed: Set(matches!(game.is_public, true)),
        ..Default::default()
    }
    .insert(get_db())
    .await?;
    let game_team = cds_db::transfer::GameTeam::from(game_team);

    Ok(WebResponse {
        code: StatusCode::OK,
        data: Some(game_team),
        ..Default::default()
    })
}
