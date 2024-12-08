pub mod calculator;

use axum::{
    body::Body,
    extract::{DefaultBodyLimit, Multipart, Path, Query},
    http::{Response, StatusCode},
    response::IntoResponse,
    Router,
};
use sea_orm::{ActiveModelTrait, ActiveValue::NotSet, ColumnTrait, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use serde_json::json;
use validator::Validate;

use crate::{
    db::{entity::user::Group, get_db},
    media::util::hash,
    web::{
        extract::{Extension, Json},
        model::Metadata,
        traits::{Ext, WebError, WebResult},
        util::handle_image_multipart,
    },
};
use crate::web::extract::VJson;

pub async fn router() -> Router {
    calculator::init().await;

    Router::new()
        .route("/", axum::routing::get(get))
        .route("/", axum::routing::post(create))
        .route("/:id", axum::routing::put(update))
        .route("/:id", axum::routing::delete(delete))
        .route("/:id/challenges", axum::routing::get(get_challenge))
        .route("/:id/challenges", axum::routing::post(create_challenge))
        .route(
            "/:id/challenges/:challenge_id",
            axum::routing::put(update_challenge),
        )
        .route(
            "/:id/challenges/:challenge_id",
            axum::routing::delete(delete_challenge),
        )
        .route("/:id/teams", axum::routing::get(get_team))
        .route("/:id/teams", axum::routing::post(create_team))
        .route("/:id/teams/:team_id", axum::routing::put(update_team))
        .route("/:id/teams/:team_id", axum::routing::delete(delete_team))
        .route("/:id/notices", axum::routing::get(get_notice))
        .route("/:id/notices", axum::routing::post(create_notice))
        .route("/:id/notices/:notice_id", axum::routing::put(update_notice))
        .route(
            "/:id/notices/:notice_id",
            axum::routing::delete(delete_notice),
        )
        .route("/:id/calculate", axum::routing::post(calculate))
        // .route(
        //     "/:id/submissions",
        //     get(handler::game::get_submission).layer(from_fn(auth::jwt(Group::User))),
        // )
        .route("/:id/poster", axum::routing::get(get_poster))
        .route(
            "/:id/poster",
            axum::routing::post(save_poster)
                .layer(DefaultBodyLimit::max(3 * 1024 * 1024 /* MB */)),
        )
        .route(
            "/:id/poster/metadata",
            axum::routing::get(get_poster_metadata),
        )
        .route("/:id/poster", axum::routing::delete(delete_poster))
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetRequest {
    pub id: Option<i64>,
    pub title: Option<String>,
    pub is_enabled: Option<bool>,
    pub page: Option<u64>,
    pub size: Option<u64>,
}

pub async fn get(
    Extension(ext): Extension<Ext>, Query(params): Query<GetRequest>,
) -> Result<WebResult<Vec<crate::db::transfer::Game>>, WebError> {
    let operator = ext
        .operator
        .ok_or(WebError::Unauthorized(json!("")))?;
    if operator.group != Group::Admin && !params.is_enabled.unwrap_or(true) {
        return Err(WebError::Forbidden(json!("")));
    }

    let (games, total) = crate::db::transfer::game::find(
        params.id,
        params.title,
        params.is_enabled,
        params.page,
        params.size,
    )
    .await?;
    let games = games
        .into_iter()
        .map(crate::db::transfer::Game::from)
        .collect::<Vec<crate::db::transfer::Game>>();

    Ok(WebResult {
        code: StatusCode::OK.as_u16(),
        data: Some(games),
        total: Some(total),
        ..WebResult::default()
    })
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
pub struct CreateRequest {
    pub title: String,
    pub started_at: i64,
    pub ended_at: i64,
    pub sketch: Option<String>,
    pub description: Option<String>,
    pub is_enabled: Option<bool>,
    pub is_public: Option<bool>,
    pub member_limit_min: Option<i64>,
    pub member_limit_max: Option<i64>,
    pub parallel_container_limit: Option<i64>,
    pub is_need_write_up: Option<bool>,
}

pub async fn create(
    Extension(ext): Extension<Ext>, VJson(body): VJson<CreateRequest>,
) -> Result<WebResult<crate::db::transfer::Game>, WebError> {
    let operator = ext
        .operator
        .ok_or(WebError::Unauthorized(json!("")))?;
    if operator.group != Group::Admin {
        return Err(WebError::Forbidden(json!("")));
    }

    let game = crate::db::entity::game::ActiveModel {
        title: Set(body.title),
        sketch: Set(body.sketch),
        description: Set(body.description),
        started_at: Set(body.started_at),
        ended_at: Set(body.ended_at),
        frozen_at: Set(body.ended_at),

        is_enabled: Set(body.is_enabled.unwrap_or(false)),
        is_public: Set(body.is_public.unwrap_or(false)),

        member_limit_min: body.member_limit_min.map_or(NotSet, Set),
        member_limit_max: body.member_limit_max.map_or(NotSet, Set),
        parallel_container_limit: body.parallel_container_limit.map_or(NotSet, Set),

        is_need_write_up: Set(body.is_need_write_up.unwrap_or(false)),

        ..Default::default()
    }
    .insert(get_db())
    .await?;
    let game = crate::db::transfer::Game::from(game);

    Ok(WebResult {
        code: StatusCode::OK.as_u16(),
        data: Some(game),
        ..WebResult::default()
    })
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
pub struct UpdateRequest {
    pub id: Option<i64>,
    pub title: Option<String>,
    pub sketch: Option<String>,
    pub description: Option<String>,
    pub is_enabled: Option<bool>,
    pub is_public: Option<bool>,
    pub member_limit_min: Option<i64>,
    pub member_limit_max: Option<i64>,
    pub parallel_container_limit: Option<i64>,
    pub is_need_write_up: Option<bool>,
    pub started_at: Option<i64>,
    pub ended_at: Option<i64>,
    pub frozen_at: Option<i64>,
}

pub async fn update(
    Extension(ext): Extension<Ext>, Path(id): Path<i64>, VJson(mut body): VJson<UpdateRequest>,
) -> Result<WebResult<crate::db::transfer::Game>, WebError> {
    let operator = ext
        .operator
        .ok_or(WebError::Unauthorized(json!("")))?;
    if operator.group != Group::Admin {
        return Err(WebError::Forbidden(json!("")));
    }

    body.id = Some(id);

    let game = crate::db::entity::game::ActiveModel {
        id: body.id.map_or(NotSet, Set),
        title: body.title.map_or(NotSet, Set),
        sketch: body.sketch.map_or(NotSet, |v| Set(Some(v))),
        description: body.description.map_or(NotSet, |v| Set(Some(v))),
        is_enabled: body.is_enabled.map_or(NotSet, Set),
        is_public: body.is_public.map_or(NotSet, Set),

        member_limit_min: body.member_limit_min.map_or(NotSet, Set),
        member_limit_max: body.member_limit_max.map_or(NotSet, Set),
        parallel_container_limit: body.parallel_container_limit.map_or(NotSet, Set),

        is_need_write_up: body.is_need_write_up.map_or(NotSet, Set),
        started_at: body.started_at.map_or(NotSet, Set),
        ended_at: body.ended_at.map_or(NotSet, Set),
        frozen_at: body.frozen_at.map_or(NotSet, Set),
        ..Default::default()
    }
    .update(get_db())
    .await?;
    let game = crate::db::transfer::Game::from(game);

    Ok(WebResult {
        code: StatusCode::OK.as_u16(),
        data: Some(game),
        ..WebResult::default()
    })
}

pub async fn delete(
    Extension(ext): Extension<Ext>, Path(id): Path<i64>,
) -> Result<WebResult<()>, WebError> {
    let operator = ext
        .operator
        .ok_or(WebError::Unauthorized(json!("")))?;
    if operator.group != Group::Admin {
        return Err(WebError::Forbidden(json!("")));
    }

    let _ = crate::db::entity::game::Entity::delete_by_id(id)
        .exec(get_db())
        .await?;

    Ok(WebResult {
        code: StatusCode::OK.as_u16(),
        ..WebResult::default()
    })
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetChallengeRequest {
    pub game_id: Option<i64>,
    pub challenge_id: Option<i64>,
    pub team_id: Option<i64>,
    pub is_enabled: Option<bool>,
}

pub async fn get_challenge(
    Extension(ext): Extension<Ext>, Query(params): Query<GetChallengeRequest>,
) -> Result<WebResult<Vec<crate::db::transfer::GameChallenge>>, WebError> {
    let _ = ext
        .operator
        .ok_or(WebError::Unauthorized(json!("")))?;

    let (game_challenges, _) = crate::db::transfer::game_challenge::find(
        params.game_id,
        params.challenge_id,
        params.is_enabled,
    )
    .await?;

    Ok(WebResult {
        code: StatusCode::OK.as_u16(),
        data: Some(game_challenges),
        ..WebResult::default()
    })
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateChallengeRequest {
    pub game_id: i64,
    pub challenge_id: i64,
    pub is_enabled: Option<bool>,
    pub difficulty: Option<i64>,
    pub max_pts: Option<i64>,
    pub min_pts: Option<i64>,
    pub first_blood_reward_ratio: Option<i64>,
    pub second_blood_reward_ratio: Option<i64>,
    pub third_blood_reward_ratio: Option<i64>,
}

pub async fn create_challenge(
    Extension(ext): Extension<Ext>, Json(body): Json<CreateChallengeRequest>,
) -> Result<WebResult<crate::db::transfer::GameChallenge>, WebError> {
    let operator = ext
        .operator
        .ok_or(WebError::Unauthorized(json!("")))?;
    if operator.group != Group::Admin {
        return Err(WebError::Forbidden(json!("")));
    }

    let game_challenge = crate::db::entity::game_challenge::ActiveModel {
        game_id: Set(body.game_id),
        challenge_id: Set(body.challenge_id),
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
    let game_challenge = crate::db::transfer::GameChallenge::from(game_challenge);

    Ok(WebResult {
        code: StatusCode::OK.as_u16(),
        data: Some(game_challenge),
        ..WebResult::default()
    })
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateChallengeRequest {
    pub game_id: Option<i64>,
    pub challenge_id: Option<i64>,
    pub is_enabled: Option<bool>,
    pub difficulty: Option<i64>,
    pub max_pts: Option<i64>,
    pub min_pts: Option<i64>,
    pub first_blood_reward_ratio: Option<i64>,
    pub second_blood_reward_ratio: Option<i64>,
    pub third_blood_reward_ratio: Option<i64>,
}

pub async fn update_challenge(
    Extension(ext): Extension<Ext>, Path((id, challenge_id)): Path<(i64, i64)>,
    Json(mut body): Json<UpdateChallengeRequest>,
) -> Result<WebResult<crate::db::transfer::GameChallenge>, WebError> {
    let operator = ext
        .operator
        .ok_or(WebError::Unauthorized(json!("")))?;
    if operator.group != Group::Admin {
        return Err(WebError::Forbidden(json!("")));
    }

    body.game_id = Some(id);
    body.challenge_id = Some(challenge_id);

    let game_challenge = crate::db::entity::game_challenge::ActiveModel {
        game_id: body.game_id.map_or(NotSet, Set),
        challenge_id: body.challenge_id.map_or(NotSet, Set),
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
    let game_challenge = crate::db::transfer::GameChallenge::from(game_challenge);

    Ok(WebResult {
        code: StatusCode::OK.as_u16(),
        data: Some(game_challenge),
        ..WebResult::default()
    })
}

pub async fn delete_challenge(
    Extension(ext): Extension<Ext>, Path((id, challenge_id)): Path<(i64, i64)>,
) -> Result<WebResult<()>, WebError> {
    let operator = ext
        .operator
        .ok_or(WebError::Unauthorized(json!("")))?;
    if operator.group != Group::Admin {
        return Err(WebError::Forbidden(json!("")));
    }

    let _ = crate::db::entity::game_challenge::Entity::delete_many()
        .filter(crate::db::entity::game_challenge::Column::GameId.eq(id))
        .filter(crate::db::entity::game_challenge::Column::ChallengeId.eq(challenge_id))
        .exec(get_db())
        .await?;

    Ok(WebResult {
        code: StatusCode::OK.as_u16(),
        ..WebResult::default()
    })
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetTeamRequest {
    pub game_id: Option<i64>,
    pub team_id: Option<i64>,
}

pub async fn get_team(
    Extension(ext): Extension<Ext>, Query(params): Query<GetTeamRequest>,
) -> Result<WebResult<Vec<crate::db::transfer::GameTeam>>, WebError> {
    let _ = ext
        .operator
        .ok_or(WebError::Unauthorized(json!("")))?;

    let (game_teams, total) =
        crate::db::transfer::game_team::find(params.game_id, params.team_id).await?;

    Ok(WebResult {
        code: StatusCode::OK.as_u16(),
        data: Some(game_teams),
        total: Some(total),
        ..WebResult::default()
    })
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateTeamRequest {
    pub game_id: i64,
    pub team_id: i64,
}

pub async fn create_team(
    Extension(ext): Extension<Ext>, Json(body): Json<CreateTeamRequest>,
) -> Result<WebResult<crate::db::transfer::GameTeam>, WebError> {
    let operator = ext
        .operator
        .ok_or(WebError::Unauthorized(json!("")))?;
    if operator.group != Group::Admin {
        return Err(WebError::Forbidden(json!("")));
    }

    let game_team = crate::db::entity::game_team::ActiveModel {
        game_id: Set(body.game_id),
        team_id: Set(body.team_id),

        ..Default::default()
    }
    .insert(get_db())
    .await?;
    let game_team = crate::db::transfer::GameTeam::from(game_team);

    Ok(WebResult {
        code: StatusCode::OK.as_u16(),
        data: Some(game_team),
        ..WebResult::default()
    })
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateTeamRequest {
    pub game_id: Option<i64>,
    pub team_id: Option<i64>,
    pub is_allowed: Option<bool>,
}

pub async fn update_team(
    Extension(ext): Extension<Ext>, Path((id, team_id)): Path<(i64, i64)>,
    Json(mut body): Json<UpdateTeamRequest>,
) -> Result<WebResult<crate::db::transfer::GameTeam>, WebError> {
    let operator = ext
        .operator
        .ok_or(WebError::Unauthorized(json!("")))?;
    if operator.group != Group::Admin {
        return Err(WebError::Forbidden(json!("")));
    }

    body.game_id = Some(id);
    body.team_id = Some(team_id);

    let game_team = crate::db::entity::game_team::ActiveModel {
        game_id: body.game_id.map_or(NotSet, Set),
        team_id: body.team_id.map_or(NotSet, Set),
        is_allowed: body.is_allowed.map_or(NotSet, Set),
        ..Default::default()
    }
    .update(get_db())
    .await?;
    let game_team = crate::db::transfer::GameTeam::from(game_team);

    Ok(WebResult {
        code: StatusCode::OK.as_u16(),
        data: Some(game_team),
        ..WebResult::default()
    })
}

pub async fn delete_team(
    Extension(ext): Extension<Ext>, Path((id, team_id)): Path<(i64, i64)>,
) -> Result<WebResult<()>, WebError> {
    let operator = ext
        .operator
        .ok_or(WebError::Unauthorized(json!("")))?;
    if operator.group != Group::Admin {
        return Err(WebError::Forbidden(json!("")));
    }

    let _ = crate::db::entity::game_team::Entity::delete_many()
        .filter(crate::db::entity::game_team::Column::GameId.eq(id))
        .filter(crate::db::entity::game_team::Column::TeamId.eq(team_id))
        .exec(get_db())
        .await?;

    Ok(WebResult {
        code: StatusCode::OK.as_u16(),
        ..WebResult::default()
    })
}

pub async fn get_notice() -> Result<impl IntoResponse, WebError> {
    Ok(todo!())
}

pub async fn create_notice() -> Result<impl IntoResponse, WebError> {
    Ok(todo!())
}

pub async fn update_notice() -> Result<impl IntoResponse, WebError> {
    Ok(todo!())
}

pub async fn delete_notice() -> Result<impl IntoResponse, WebError> {
    Ok(todo!())
}

pub async fn calculate(
    Extension(ext): Extension<Ext>, Path(id): Path<i64>,
) -> Result<WebResult<()>, WebError> {
    let operator = ext
        .operator
        .ok_or(WebError::Unauthorized(json!("")))?;
    if operator.group != Group::Admin {
        return Err(WebError::Forbidden(json!("")));
    }

    crate::queue::publish("calculator", calculator::Payload { game_id: Some(id) }).await?;

    Ok(WebResult {
        code: StatusCode::OK.as_u16(),
        ..WebResult::default()
    })
}

// pub async fn get_submission(
//     Path(id): Path<i64>, Query(params): Query<GetSubmissionRequest>,
// ) -> Result<impl IntoResponse, WebError> {
//     let submissions = crate::transfer::submission::get_with_pts(id,
// params.status).await?;

//     return Ok((
//         StatusCode::OK,
//         Json(GetSubmissionResponse {
//             code: StatusCode::OK.as_u16(),
//             data: submissions,
//         }),
//     ));
// }

// pub async fn get_scoreboard(Path(id): Path<i64>) -> Result<impl IntoResponse,
// WebError> {     pub struct TeamScoreRecord {}

//     let submissions =
//         crate::transfer::submission::get_with_pts(id,
// Some(crate::transfer::submission::Status::Correct))             .await;

//     let game_teams = crate::transfer::game_team::Entity::find()
//         .filter(
//             Condition::all()
//                 .add(crate::transfer::game_team::Column::GameId.eq(id))
//                 .add(crate::transfer::game_team::Column::IsAllowed.eq(true)),
//         )
//         .all(get_db())
//         .await?;

//     return Ok(());
// }

pub async fn get_poster(Path(id): Path<i64>) -> Result<impl IntoResponse, WebError> {
    let path = format!("games/{}/poster", id);
    match crate::media::scan_dir(path.clone()).await?.first() {
        Some((filename, _size)) => {
            let buffer = crate::media::get(path, filename.to_string()).await?;
            Ok(Response::builder().body(Body::from(buffer)).unwrap())
        }
        None => Err(WebError::NotFound(json!(""))),
    }
}

pub async fn get_poster_metadata(Path(id): Path<i64>) -> Result<WebResult<Metadata>, WebError> {
    let path = format!("games/{}/poster", id);
    match crate::media::scan_dir(path.clone()).await?.first() {
        Some((filename, size)) => Ok(WebResult {
            code: StatusCode::OK.as_u16(),
            data: Some(Metadata {
                filename: filename.to_string(),
                size: *size,
            }),
            ..WebResult::default()
        }),
        None => Err(WebError::NotFound(json!(""))),
    }
}

pub async fn save_poster(
    Extension(ext): Extension<Ext>, Path(id): Path<i64>, multipart: Multipart,
) -> Result<WebResult<()>, WebError> {
    let operator = ext
        .operator
        .ok_or(WebError::Unauthorized(json!("")))?;
    if operator.group != Group::Admin {
        return Err(WebError::Forbidden(json!("")));
    }

    let path = format!("games/{}/poster", id);
    let data = handle_image_multipart(multipart).await?;

    crate::media::delete_dir(path.clone()).await.unwrap();

    let data = crate::media::util::img_convert_to_webp(data).await?;
    let filename = format!("{}.webp", hash(data.clone()));

    crate::media::save(path, filename, data)
        .await
        .map_err(|_| WebError::InternalServerError(json!("")))?;

    Ok(WebResult {
        code: StatusCode::OK.as_u16(),
        ..WebResult::default()
    })
}

pub async fn delete_poster(
    Extension(ext): Extension<Ext>, Path(id): Path<i64>,
) -> Result<WebResult<()>, WebError> {
    let operator = ext
        .operator
        .ok_or(WebError::Unauthorized(json!("")))?;
    if operator.group != Group::Admin {
        return Err(WebError::Forbidden(json!("")));
    }

    let path = format!("games/{}/poster", id);

    crate::media::delete_dir(path)
        .await
        .map_err(|_| WebError::InternalServerError(json!("")))?;

    Ok(WebResult {
        code: StatusCode::OK.as_u16(),
        ..WebResult::default()
    })
}
