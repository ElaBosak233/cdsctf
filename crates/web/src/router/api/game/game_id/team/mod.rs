mod profile;
mod team_id;

use std::str::FromStr;

use axum::{Router, http::StatusCode};
use cds_db::{
    entity::team::State,
    get_db,
    sea_orm::{
        ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, JoinType, Order,
        PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, RelationTrait,
    },
    transfer::Team,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use cds_db::traits::EagerLoading;
use crate::{
    extract::{Extension, Json, Path, Query},
    traits::{Ext, WebError, WebResponse},
};

pub fn router() -> Router {
    Router::new()
        .route("/register", axum::routing::post(team_register))
        .nest("/profile", profile::router())
        .nest("/{team_id}", team_id::router())
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetTeamRequest {
    /// The team id of expected game teams.
    pub id: Option<i64>,
    pub name: Option<String>,
    pub state: Option<State>,

    /// The user id of expected game teams.
    ///
    /// `user_id` is not in table `teams`, so it relies on JOIN queries.
    /// Essentially, it is unrelated to game team.
    ///
    /// ```sql
    /// SELECT *
    /// FROM "teams"
    ///     INNER JOIN "team_users" ON "teams"."id" = "team_users"."profile"
    /// WHERE "team_users"."game_id" = ? AND "team_users"."user_id" = ?;
    /// ```
    pub user_id: Option<i64>,

    pub page: Option<u64>,
    pub size: Option<u64>,
    pub sorts: Option<String>,
}

/// Get game teams with given data.
pub async fn get_team(
    Extension(ext): Extension<Ext>,
    Path(game_id): Path<i64>,
    Query(params): Query<GetTeamRequest>,
) -> Result<WebResponse<Vec<Team>>, WebError> {
    let _ = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;

    let mut sql = cds_db::entity::team::Entity::find();

    sql = sql.filter(cds_db::entity::team::Column::GameId.eq(game_id));

    if let Some(id) = params.id {
        sql = sql.filter(cds_db::entity::team::Column::Id.eq(id));
    }

    if let Some(name) = params.name {
        sql = sql.filter(cds_db::entity::team::Column::Name.contains(name));
    }

    if let Some(state) = params.state {
        sql = sql.filter(cds_db::entity::team::Column::State.eq(state));
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

    if let (Some(page), Some(size)) = (params.page, params.size) {
        let offset = (page - 1) * size;
        sql = sql.offset(offset).limit(size);
    }

    let teams = sql.all(get_db()).await?.eager_load(get_db()).await?;

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
/// - No user in the team is already in the game.
pub async fn team_register(
    Extension(ext): Extension<Ext>,
    Path(game_id): Path<i64>,
    Json(body): Json<CreateTeamRequest>,
) -> Result<WebResponse<Team>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;

    let game = cds_db::entity::game::Entity::find_by_id(game_id)
        .one(get_db())
        .await?
        .map(cds_db::transfer::Game::from)
        .ok_or(WebError::BadRequest(json!("game_not_found")))?;

    if cds_db::util::is_user_in_game(&operator, &game, None).await? {
        return Err(WebError::BadRequest(json!("user_already_in_game")));
    }

    let team = cds_db::entity::team::ActiveModel {
        name: Set(body.name),
        email: Set(body.email),
        slogan: Set(body.slogan),
        game_id: Set(game.id),
        state: Set(State::Preparing),
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
        .map(cds_db::transfer::Team::from)
        .unwrap();

    Ok(WebResponse {
        code: StatusCode::OK,
        data: Some(team),
        ..Default::default()
    })
}
