pub mod calculator;

use std::str::FromStr;

use axum::{
    Router,
    extract::{DefaultBodyLimit, Multipart},
    http::StatusCode,
    response::IntoResponse,
};
use cds_db::{
    entity::{submission::Status, user::Group},
    get_db,
    transfer::{GameTeam, Submission},
};
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set, Unchanged},
    ColumnTrait, Condition, EntityTrait, JoinType, Order, PaginatorTrait, QueryFilter, QueryOrder,
    QuerySelect, RelationTrait,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;
use validator::Validate;

use crate::{
    extract::{Extension, Json, Path, Query, VJson},
    model::Metadata,
    traits::{Ext, WebError, WebResponse},
    util,
};

pub async fn router() -> Router {
    calculator::init().await;

    Router::new()
        .route("/", axum::routing::get(get_game))
        .route("/", axum::routing::post(create_game))
        .route("/{id}", axum::routing::put(update_game))
        .route("/{id}", axum::routing::delete(delete_game))
        .route("/{id}/challenges", axum::routing::get(get_game_challenge))
        .route(
            "/{id}/challenges",
            axum::routing::post(create_game_challenge),
        )
        .route(
            "/{id}/challenges/{challenge_id}",
            axum::routing::put(update_game_challenge),
        )
        .route(
            "/{id}/challenges/{challenge_id}",
            axum::routing::delete(delete_game_challenge),
        )
        .route("/{id}/teams", axum::routing::get(get_game_team))
        .route("/{id}/teams", axum::routing::post(create_game_team))
        .route(
            "/{id}/teams/{team_id}",
            axum::routing::put(update_game_team),
        )
        .route(
            "/{id}/teams/{team_id}",
            axum::routing::delete(delete_game_team),
        )
        .route("/{id}/notices", axum::routing::get(get_notice))
        .route("/{id}/notices", axum::routing::post(create_notice))
        .route(
            "/{id}/notices/{notice_id}",
            axum::routing::put(update_notice),
        )
        .route(
            "/{id}/notices/{notice_id}",
            axum::routing::delete(delete_notice),
        )
        .route("/{id}/calculate", axum::routing::post(calculate_game))
        .route("/{id}/scoreboard", axum::routing::get(get_game_scoreboard))
        .route("/{id}/icon", axum::routing::get(get_game_icon))
        .route(
            "/{id}/icon",
            axum::routing::post(save_game_icon)
                .layer(DefaultBodyLimit::max(3 * 1024 * 1024 /* MB */)),
        )
        .route(
            "/{id}/icon/metadata",
            axum::routing::get(get_game_icon_metadata),
        )
        .route("/{id}/icon", axum::routing::delete(delete_game_icon))
        .route("/{id}/poster", axum::routing::get(get_game_poster))
        .route(
            "/{id}/poster",
            axum::routing::post(save_game_poster)
                .layer(DefaultBodyLimit::max(3 * 1024 * 1024 /* MB */)),
        )
        .route(
            "/{id}/poster/metadata",
            axum::routing::get(get_game_poster_metadata),
        )
        .route("/{id}/poster", axum::routing::delete(delete_game_poster))
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetGameRequest {
    pub id: Option<i64>,
    pub title: Option<String>,
    pub is_enabled: Option<bool>,
    pub page: Option<u64>,
    pub size: Option<u64>,
    pub sorts: Option<String>,
}

/// Get games with given params.
pub async fn get_game(
    Extension(ext): Extension<Ext>, Query(params): Query<GetGameRequest>,
) -> Result<WebResponse<Vec<cds_db::transfer::Game>>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    if operator.group != Group::Admin && !params.is_enabled.unwrap_or(true) {
        return Err(WebError::Forbidden(json!("")));
    }

    let mut sql = cds_db::entity::game::Entity::find();

    if let Some(id) = params.id {
        sql = sql.filter(cds_db::entity::game::Column::Id.eq(id));
    }

    if let Some(title) = params.title {
        sql = sql.filter(cds_db::entity::game::Column::Title.contains(title));
    }

    if let Some(is_enabled) = params.is_enabled {
        sql = sql.filter(cds_db::entity::game::Column::IsEnabled.eq(is_enabled));
    }

    if let Some(sorts) = params.sorts {
        let sorts = sorts.split(",").collect::<Vec<&str>>();
        for sort in sorts {
            let col = match cds_db::entity::game::Column::from_str(sort.replace("-", "").as_str()) {
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

    let total = sql.clone().count(get_db()).await?;

    if let (Some(page), Some(size)) = (params.page, params.size) {
        let offset = (page - 1) * size;
        sql = sql.offset(offset).limit(size);
    }

    let games = sql
        .all(get_db())
        .await?
        .into_iter()
        .map(cds_db::transfer::Game::from)
        .collect::<Vec<cds_db::transfer::Game>>();

    Ok(WebResponse {
        code: StatusCode::OK.as_u16(),
        data: Some(games),
        total: Some(total),
        ..WebResponse::default()
    })
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
pub struct CreateGameRequest {
    pub title: String,
    pub sketch: Option<String>,
    pub description: Option<String>,
    pub is_enabled: Option<bool>,
    pub is_public: Option<bool>,
    pub member_limit_min: Option<i64>,
    pub member_limit_max: Option<i64>,
    pub is_need_write_up: Option<bool>,
    pub started_at: i64,
    pub ended_at: i64,
}

pub async fn create_game(
    Extension(ext): Extension<Ext>, VJson(body): VJson<CreateGameRequest>,
) -> Result<WebResponse<cds_db::transfer::Game>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    if operator.group != Group::Admin {
        return Err(WebError::Forbidden(json!("")));
    }

    let game = cds_db::entity::game::ActiveModel {
        title: Set(body.title),
        sketch: Set(body.sketch),
        description: Set(body.description),

        is_enabled: Set(body.is_enabled.unwrap_or(false)),
        is_public: Set(body.is_public.unwrap_or(false)),
        is_need_write_up: Set(body.is_need_write_up.unwrap_or(false)),

        member_limit_min: body.member_limit_min.map_or(NotSet, Set),
        member_limit_max: body.member_limit_max.map_or(NotSet, Set),

        started_at: Set(body.started_at),
        ended_at: Set(body.ended_at),
        frozen_at: Set(body.ended_at),
        ..Default::default()
    }
    .insert(get_db())
    .await?;
    let game = cds_db::transfer::Game::from(game);

    Ok(WebResponse {
        code: StatusCode::OK.as_u16(),
        data: Some(game),
        ..WebResponse::default()
    })
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
pub struct UpdateGameRequest {
    pub id: Option<i64>,
    pub title: Option<String>,
    pub sketch: Option<String>,
    pub description: Option<String>,
    pub is_enabled: Option<bool>,
    pub is_public: Option<bool>,
    pub member_limit_min: Option<i64>,
    pub member_limit_max: Option<i64>,
    pub is_need_write_up: Option<bool>,
    pub started_at: Option<i64>,
    pub ended_at: Option<i64>,
    pub frozen_at: Option<i64>,
}

pub async fn update_game(
    Extension(ext): Extension<Ext>, Path(id): Path<i64>, VJson(mut body): VJson<UpdateGameRequest>,
) -> Result<WebResponse<cds_db::transfer::Game>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    if operator.group != Group::Admin {
        return Err(WebError::Forbidden(json!("")));
    }

    let game = cds_db::entity::game::Entity::find_by_id(id)
        .one(get_db())
        .await?
        .ok_or(WebError::BadRequest(json!("game_not_found")))?;

    let game = cds_db::entity::game::ActiveModel {
        id: Unchanged(game.id),
        title: body.title.map_or(NotSet, Set),
        sketch: body.sketch.map_or(NotSet, |v| Set(Some(v))),
        description: body.description.map_or(NotSet, |v| Set(Some(v))),
        is_enabled: body.is_enabled.map_or(NotSet, Set),
        is_public: body.is_public.map_or(NotSet, Set),
        is_need_write_up: body.is_need_write_up.map_or(NotSet, Set),

        member_limit_min: body.member_limit_min.map_or(NotSet, Set),
        member_limit_max: body.member_limit_max.map_or(NotSet, Set),

        started_at: body.started_at.map_or(NotSet, Set),
        ended_at: body.ended_at.map_or(NotSet, Set),
        frozen_at: body.frozen_at.map_or(NotSet, Set),
        ..Default::default()
    }
    .update(get_db())
    .await?;
    let game = cds_db::transfer::Game::from(game);

    Ok(WebResponse {
        code: StatusCode::OK.as_u16(),
        data: Some(game),
        ..WebResponse::default()
    })
}

pub async fn delete_game(
    Extension(ext): Extension<Ext>, Path(id): Path<i64>,
) -> Result<WebResponse<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    if operator.group != Group::Admin {
        return Err(WebError::Forbidden(json!("")));
    }

    let game = cds_db::entity::game::Entity::find_by_id(id)
        .one(get_db())
        .await?
        .ok_or(WebError::BadRequest(json!("game_not_found")))?;

    let _ = cds_db::entity::game::Entity::delete_by_id(game.id)
        .exec(get_db())
        .await?;

    Ok(WebResponse {
        code: StatusCode::OK.as_u16(),
        ..WebResponse::default()
    })
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetGameChallengeRequest {
    pub game_id: Option<i64>,
    pub challenge_id: Option<i64>,
    pub category: Option<i32>,
    pub is_enabled: Option<bool>,

    pub page: Option<u64>,
    pub size: Option<u64>,
}

/// Get challenges by given params.
///
/// # Prerequisite
/// - If the operator is admin, there is no prerequisite.
/// - Operator is in one of the `is_allowed` = `true` game teams.
/// - Operating time is between related game's `started_at` and `ended_at`.
pub async fn get_game_challenge(
    Extension(ext): Extension<Ext>, Path(id): Path<i64>,
    Query(mut params): Query<GetGameChallengeRequest>,
) -> Result<WebResponse<Vec<cds_db::transfer::GameChallenge>>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;

    let game = cds_db::entity::game::Entity::find_by_id(id)
        .one(get_db())
        .await?
        .map(|game| cds_db::transfer::Game::from(game))
        .ok_or(WebError::BadRequest(json!("game_not_found")))?;

    if operator.group != Group::Admin {
        let now = chrono::Utc::now().timestamp();
        let in_game = cds_db::util::is_user_in_game(&operator, &game, Some(true)).await?;

        if !in_game
            || !(game.started_at..=game.ended_at).contains(&now)
            || params.is_enabled != Some(true)
        {
            return Err(WebError::Forbidden(json!("")));
        }
    }

    // Using inner join to access fields in related tables.
    let mut sql = cds_db::entity::game_challenge::Entity::find()
        .inner_join(cds_db::entity::challenge::Entity)
        .inner_join(cds_db::entity::game::Entity);

    sql = sql.filter(cds_db::entity::game_challenge::Column::GameId.eq(id));

    if let Some(challenge_id) = params.challenge_id {
        sql = sql.filter(cds_db::entity::game_challenge::Column::ChallengeId.eq(challenge_id));
    }

    if let Some(is_enabled) = params.is_enabled {
        sql = sql.filter(cds_db::entity::game_challenge::Column::IsEnabled.eq(is_enabled));
    }

    if let Some(category) = params.category {
        sql = sql.filter(cds_db::entity::challenge::Column::Category.eq(category));
    }

    let total = sql.clone().count(get_db()).await?;

    if let (Some(page), Some(size)) = (params.page, params.size) {
        let offset = (page - 1) * size;
        sql = sql.offset(offset).limit(size);
    }

    let game_challenges =
        cds_db::transfer::game_challenge::preload(sql.all(get_db()).await?).await?;

    Ok(WebResponse {
        code: StatusCode::OK.as_u16(),
        data: Some(game_challenges),
        total: Some(total),
        ..WebResponse::default()
    })
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateGameChallengeRequest {
    pub game_id: i64,
    pub challenge_id: Uuid,
    pub is_enabled: Option<bool>,
    pub difficulty: Option<i64>,
    pub max_pts: Option<i64>,
    pub min_pts: Option<i64>,
    pub first_blood_reward_ratio: Option<i64>,
    pub second_blood_reward_ratio: Option<i64>,
    pub third_blood_reward_ratio: Option<i64>,
}

pub async fn create_game_challenge(
    Extension(ext): Extension<Ext>, Json(body): Json<CreateGameChallengeRequest>,
) -> Result<WebResponse<cds_db::transfer::GameChallenge>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    if operator.group != Group::Admin {
        return Err(WebError::Forbidden(json!("")));
    }

    let game = cds_db::entity::game::Entity::find_by_id(body.game_id)
        .one(get_db())
        .await?
        .ok_or(WebError::BadRequest(json!("game_not_found")))?;

    let challenge = cds_db::entity::challenge::Entity::find_by_id(body.challenge_id)
        .one(get_db())
        .await?
        .ok_or(WebError::BadRequest(json!("challenge_not_found")))?;

    let game_challenge = cds_db::entity::game_challenge::ActiveModel {
        game_id: Set(game.id),
        challenge_id: Set(challenge.id),
        difficulty: body.difficulty.map_or(NotSet, Set),
        is_enabled: body.is_enabled.map_or(NotSet, Set),
        max_pts: body.max_pts.map_or(NotSet, Set),
        min_pts: body.min_pts.map_or(NotSet, Set),
        first_blood_reward_ratio: body.first_blood_reward_ratio.map_or(NotSet, Set),
        second_blood_reward_ratio: body.second_blood_reward_ratio.map_or(NotSet, Set),
        third_blood_reward_ratio: body.third_blood_reward_ratio.map_or(NotSet, Set),
        ..Default::default()
    }
    .insert(get_db())
    .await?;
    let game_challenge = cds_db::transfer::GameChallenge::from(game_challenge);

    Ok(WebResponse {
        code: StatusCode::OK.as_u16(),
        data: Some(game_challenge),
        ..WebResponse::default()
    })
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateGameChallengeRequest {
    pub game_id: Option<i64>,
    pub challenge_id: Option<Uuid>,
    pub is_enabled: Option<bool>,
    pub difficulty: Option<i64>,
    pub max_pts: Option<i64>,
    pub min_pts: Option<i64>,
    pub first_blood_reward_ratio: Option<i64>,
    pub second_blood_reward_ratio: Option<i64>,
    pub third_blood_reward_ratio: Option<i64>,
}

pub async fn update_game_challenge(
    Extension(ext): Extension<Ext>, Path((id, challenge_id)): Path<(i64, Uuid)>,
    Json(mut body): Json<UpdateGameChallengeRequest>,
) -> Result<WebResponse<cds_db::transfer::GameChallenge>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    if operator.group != Group::Admin {
        return Err(WebError::Forbidden(json!("")));
    }

    let game_challenge = cds_db::entity::game_challenge::Entity::find()
        .filter(
            Condition::all()
                .add(cds_db::entity::game_challenge::Column::GameId.eq(id))
                .add(cds_db::entity::game_challenge::Column::ChallengeId.eq(challenge_id)),
        )
        .one(get_db())
        .await?
        .ok_or(WebError::BadRequest(json!("game_challenge_not_found")))?;

    let game_challenge = cds_db::entity::game_challenge::ActiveModel {
        game_id: Unchanged(game_challenge.game_id),
        challenge_id: Unchanged(game_challenge.challenge_id),
        difficulty: body.difficulty.map_or(NotSet, Set),
        is_enabled: body.is_enabled.map_or(NotSet, Set),
        max_pts: body.max_pts.map_or(NotSet, Set),
        min_pts: body.min_pts.map_or(NotSet, Set),
        first_blood_reward_ratio: body.first_blood_reward_ratio.map_or(NotSet, Set),
        second_blood_reward_ratio: body.second_blood_reward_ratio.map_or(NotSet, Set),
        third_blood_reward_ratio: body.third_blood_reward_ratio.map_or(NotSet, Set),
        ..Default::default()
    }
    .update(get_db())
    .await?;
    let game_challenge = cds_db::transfer::GameChallenge::from(game_challenge);

    Ok(WebResponse {
        code: StatusCode::OK.as_u16(),
        data: Some(game_challenge),
        ..WebResponse::default()
    })
}

pub async fn delete_game_challenge(
    Extension(ext): Extension<Ext>, Path((id, challenge_id)): Path<(i64, Uuid)>,
) -> Result<WebResponse<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    if operator.group != Group::Admin {
        return Err(WebError::Forbidden(json!("")));
    }

    let game_challenge = cds_db::entity::game_challenge::Entity::find()
        .filter(
            Condition::all()
                .add(cds_db::entity::game_challenge::Column::GameId.eq(id))
                .add(cds_db::entity::game_challenge::Column::ChallengeId.eq(challenge_id)),
        )
        .one(get_db())
        .await?
        .ok_or(WebError::BadRequest(json!("game_challenge_not_found")))?;

    let _ = cds_db::entity::game_challenge::Entity::delete_many()
        .filter(cds_db::entity::game_challenge::Column::GameId.eq(game_challenge.game_id))
        .filter(cds_db::entity::game_challenge::Column::ChallengeId.eq(game_challenge.challenge_id))
        .exec(get_db())
        .await?;

    Ok(WebResponse {
        code: StatusCode::OK.as_u16(),
        ..WebResponse::default()
    })
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
    ///     INNER JOIN "user_teams" ON "teams"."id" = "user_teams"."team_id"
    /// WHERE "game_teams"."game_id" = ? AND "user_teams"."user_id" = ?;
    /// ```
    pub user_id: Option<i64>,
}

/// Get game teams with given data.
pub async fn get_game_team(
    Extension(ext): Extension<Ext>, Path(id): Path<i64>, Query(params): Query<GetGameTeamRequest>,
) -> Result<WebResponse<Vec<GameTeam>>, WebError> {
    let _ = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;

    let mut sql = cds_db::entity::game_team::Entity::find();

    sql = sql.filter(cds_db::entity::game_team::Column::GameId.eq(id));

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
                cds_db::entity::user_team::Relation::Team.def().rev(),
            )
            .filter(cds_db::entity::user_team::Column::UserId.eq(user_id))
    }

    let total = sql.clone().count(get_db()).await?;

    let game_teams = sql.all(get_db()).await?;
    let mut game_teams = game_teams
        .into_iter()
        .map(GameTeam::from)
        .collect::<Vec<GameTeam>>();

    game_teams = cds_db::transfer::game_team::preload(game_teams).await?;

    Ok(WebResponse {
        code: StatusCode::OK.as_u16(),
        data: Some(game_teams),
        total: Some(total),
        ..WebResponse::default()
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
    Extension(ext): Extension<Ext>, Path(id): Path<i64>, Json(body): Json<CreateGameTeamRequest>,
) -> Result<WebResponse<GameTeam>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;

    let game = cds_db::entity::game::Entity::find_by_id(id)
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
        code: StatusCode::OK.as_u16(),
        data: Some(game_team),
        ..WebResponse::default()
    })
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateGameTeamRequest {
    pub game_id: Option<i64>,
    pub team_id: Option<i64>,
    pub is_allowed: Option<bool>,
}

/// Update a game team with given path and data.
///
/// This function is only used to switch whether
/// the game_team is allowed to access the game or not.
///
/// # Prerequisite
/// - Operator is admin.
pub async fn update_game_team(
    Extension(ext): Extension<Ext>, Path((id, team_id)): Path<(i64, i64)>,
    Json(mut body): Json<UpdateGameTeamRequest>,
) -> Result<WebResponse<GameTeam>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    if operator.group != Group::Admin {
        return Err(WebError::Forbidden(json!("")));
    }

    let game_team = cds_db::entity::game_team::Entity::find()
        .filter(
            Condition::all()
                .add(cds_db::entity::game_team::Column::GameId.eq(id))
                .add(cds_db::entity::game_team::Column::TeamId.eq(team_id)),
        )
        .one(get_db())
        .await?
        .ok_or(WebError::BadRequest(json!("game_team_not_found")))?;

    let game_team = cds_db::entity::game_team::ActiveModel {
        game_id: Unchanged(game_team.game_id),
        team_id: Unchanged(game_team.team_id),
        is_allowed: body.is_allowed.map_or(NotSet, Set),
        ..Default::default()
    }
    .update(get_db())
    .await?;
    let game_team = cds_db::transfer::GameTeam::from(game_team);

    Ok(WebResponse {
        code: StatusCode::OK.as_u16(),
        data: Some(game_team),
        ..WebResponse::default()
    })
}

pub async fn delete_game_team(
    Extension(ext): Extension<Ext>, Path((id, team_id)): Path<(i64, i64)>,
) -> Result<WebResponse<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    if operator.group != Group::Admin {
        return Err(WebError::Forbidden(json!("")));
    }

    let game_team = cds_db::entity::game_team::Entity::find()
        .filter(
            Condition::all()
                .add(cds_db::entity::game_team::Column::GameId.eq(id))
                .add(cds_db::entity::game_team::Column::TeamId.eq(team_id)),
        )
        .one(get_db())
        .await?
        .ok_or(WebError::BadRequest(json!("game_team_not_found")))?;

    let _ = cds_db::entity::game_team::Entity::delete_many()
        .filter(cds_db::entity::game_team::Column::GameId.eq(game_team.game_id))
        .filter(cds_db::entity::game_team::Column::TeamId.eq(game_team.team_id))
        .exec(get_db())
        .await?;

    Ok(WebResponse {
        code: StatusCode::OK.as_u16(),
        ..WebResponse::default()
    })
}

pub async fn get_notice() -> Result<impl IntoResponse, WebError> {
    Ok("")
}

pub async fn create_notice() -> Result<impl IntoResponse, WebError> {
    Ok("")
}

pub async fn update_notice() -> Result<impl IntoResponse, WebError> {
    Ok("")
}

pub async fn delete_notice() -> Result<impl IntoResponse, WebError> {
    Ok("")
}

pub async fn calculate_game(
    Extension(ext): Extension<Ext>, Path(id): Path<i64>,
) -> Result<WebResponse<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    if operator.group != Group::Admin {
        return Err(WebError::Forbidden(json!("")));
    }

    cds_queue::publish("calculator", calculator::Payload { game_id: Some(id) }).await?;

    Ok(WebResponse {
        code: StatusCode::OK.as_u16(),
        ..WebResponse::default()
    })
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetGameScoreboardRequest {
    pub size: Option<u64>,
    pub page: Option<u64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ScoreRecord {
    pub game_team: GameTeam,
    pub submissions: Vec<Submission>,
}

pub async fn get_game_scoreboard(
    Path(id): Path<i64>, Query(params): Query<GetGameScoreboardRequest>,
) -> Result<WebResponse<Vec<ScoreRecord>>, WebError> {
    let mut sql = cds_db::entity::game_team::Entity::find()
        .filter(cds_db::entity::game_team::Column::GameId.eq(id))
        .order_by(cds_db::entity::game_team::Column::Pts, Order::Desc);

    let total = sql.clone().count(get_db()).await?;

    if let (Some(page), Some(size)) = (params.page, params.size) {
        let offset = (page - 1) * size;
        sql = sql.offset(offset).limit(size);
    }

    let game_teams = sql.all(get_db()).await?;
    let mut game_teams = game_teams
        .into_iter()
        .map(GameTeam::from)
        .collect::<Vec<GameTeam>>();

    game_teams = cds_db::transfer::game_team::preload(game_teams).await?;

    let team_ids = game_teams.iter().map(|t| t.team_id).collect::<Vec<i64>>();

    let submissions = cds_db::transfer::submission::get_by_game_id_and_team_ids(
        id,
        team_ids,
        Some(Status::Correct),
    )
    .await?;

    let mut result: Vec<ScoreRecord> = Vec::new();

    for game_team in game_teams {
        let mut submissions = submissions
            .iter()
            .filter(|s| s.team_id.unwrap() == game_team.team_id)
            .cloned()
            .collect::<Vec<Submission>>();
        for submission in submissions.iter_mut() {
            submission.flag.clear();
            submission.team = None;
            submission.game = None;
        }

        result.push(ScoreRecord {
            game_team,
            submissions,
        });
    }

    Ok(WebResponse {
        code: StatusCode::OK.as_u16(),
        data: Some(result),
        total: Some(total),
        ..Default::default()
    })
}

pub async fn get_game_poster(Path(id): Path<i64>) -> Result<impl IntoResponse, WebError> {
    let path = format!("games/{}/poster", id);

    util::media::get_img(path).await
}

pub async fn get_game_poster_metadata(
    Path(id): Path<i64>,
) -> Result<WebResponse<Metadata>, WebError> {
    let path = format!("games/{}/poster", id);

    util::media::get_img_metadata(path).await
}

pub async fn save_game_poster(
    Extension(ext): Extension<Ext>, Path(id): Path<i64>, multipart: Multipart,
) -> Result<WebResponse<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    if operator.group != Group::Admin {
        return Err(WebError::Forbidden(json!("")));
    }

    let path = format!("games/{}/poster", id);

    util::media::save_img(path, multipart).await
}

pub async fn delete_game_poster(
    Extension(ext): Extension<Ext>, Path(id): Path<i64>,
) -> Result<WebResponse<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    if operator.group != Group::Admin {
        return Err(WebError::Forbidden(json!("")));
    }

    let path = format!("games/{}/poster", id);

    util::media::delete_img(path).await
}

pub async fn get_game_icon(Path(id): Path<i64>) -> Result<impl IntoResponse, WebError> {
    let path = format!("games/{}/icon", id);

    util::media::get_img(path).await
}

pub async fn get_game_icon_metadata(
    Path(id): Path<i64>,
) -> Result<WebResponse<Metadata>, WebError> {
    let path = format!("games/{}/icon", id);

    util::media::get_img_metadata(path).await
}

pub async fn save_game_icon(
    Extension(ext): Extension<Ext>, Path(id): Path<i64>, multipart: Multipart,
) -> Result<WebResponse<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    if operator.group != Group::Admin {
        return Err(WebError::Forbidden(json!("")));
    }

    let path = format!("games/{}/icon", id);

    util::media::save_img(path, multipart).await
}

pub async fn delete_game_icon(
    Extension(ext): Extension<Ext>, Path(id): Path<i64>,
) -> Result<WebResponse<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    if operator.group != Group::Admin {
        return Err(WebError::Forbidden(json!("")));
    }

    let path = format!("games/{}/icon", id);

    util::media::delete_img(path).await
}
