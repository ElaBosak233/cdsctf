pub mod calculator;

use axum::{
    body::Body,
    extract::{DefaultBodyLimit, Multipart, Path, Query},
    http::{Response, StatusCode},
    response::IntoResponse,
    Extension, Json, Router,
};
use mime::Mime;
use sea_orm::{ActiveModelTrait, ActiveValue::NotSet, ColumnTrait, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    database::get_db,
    model::user::group::Group,
    web::{
        model::Metadata,
        traits::{Ext, WebError, WebResult},
    },
};

pub async fn router() -> Router {
    calculator::init().await;

    return Router::new()
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
        .route("/:id/poster", axum::routing::delete(delete_poster));
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
) -> Result<WebResult<Vec<crate::model::game::Model>>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(String::new()))?;
    if operator.group != Group::Admin && !params.is_enabled.unwrap_or(true) {
        return Err(WebError::Forbidden(String::new()));
    }

    let (games, total) = crate::model::game::find(
        params.id,
        params.title,
        params.is_enabled,
        params.page,
        params.size,
    )
    .await?;

    return Ok(WebResult {
        code: StatusCode::OK.as_u16(),
        data: Some(games),
        total: Some(total),
        ..WebResult::default()
    });
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
pub struct CreateRequest {
    pub title: String,
    pub started_at: i64,
    pub ended_at: i64,

    pub bio: Option<String>,
    pub description: Option<String>,
    pub is_enabled: Option<bool>,
    pub is_public: Option<bool>,
    pub member_limit_min: Option<i64>,
    pub member_limit_max: Option<i64>,
    pub parallel_container_limit: Option<i64>,
    pub is_need_write_up: Option<bool>,
}

pub async fn create(
    Extension(ext): Extension<Ext>, Json(body): Json<CreateRequest>,
) -> Result<WebResult<crate::model::game::Model>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(String::new()))?;
    if operator.group != Group::Admin {
        return Err(WebError::Forbidden(String::new()));
    }

    let game = crate::model::game::ActiveModel {
        title: Set(body.title),
        bio: Set(body.bio),
        description: Set(body.description),
        started_at: Set(body.started_at),
        ended_at: Set(body.ended_at),
        frozed_at: Set(body.ended_at),

        is_enabled: Set(body.is_enabled.unwrap_or(false)),
        is_public: Set(body.is_public.unwrap_or(false)),

        member_limit_min: body.member_limit_min.map_or(NotSet, |v| Set(v)),
        member_limit_max: body.member_limit_max.map_or(NotSet, |v| Set(v)),
        parallel_container_limit: body.parallel_container_limit.map_or(NotSet, |v| Set(v)),

        is_need_write_up: Set(body.is_need_write_up.unwrap_or(false)),

        ..Default::default()
    }
    .insert(&get_db())
    .await?;

    return Ok(WebResult {
        code: StatusCode::OK.as_u16(),
        data: Some(game),
        ..WebResult::default()
    });
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
pub struct UpdateRequest {
    pub id: Option<i64>,
    pub title: Option<String>,
    pub bio: Option<String>,
    pub description: Option<String>,
    pub is_enabled: Option<bool>,
    pub is_public: Option<bool>,
    pub member_limit_min: Option<i64>,
    pub member_limit_max: Option<i64>,
    pub parallel_container_limit: Option<i64>,
    pub is_need_write_up: Option<bool>,
    pub started_at: Option<i64>,
    pub ended_at: Option<i64>,
    pub frozed_at: Option<i64>,
}

pub async fn update(
    Extension(ext): Extension<Ext>, Path(id): Path<i64>, Json(mut body): Json<UpdateRequest>,
) -> Result<WebResult<crate::model::game::Model>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(String::new()))?;
    if operator.group != Group::Admin {
        return Err(WebError::Forbidden(String::new()));
    }

    body.id = Some(id);

    let game = crate::model::game::ActiveModel {
        id: body.id.map_or(NotSet, |v| Set(v)),
        title: body.title.map_or(NotSet, |v| Set(v)),
        bio: body.bio.map_or(NotSet, |v| Set(Some(v))),
        description: body.description.map_or(NotSet, |v| Set(Some(v))),
        is_enabled: body.is_enabled.map_or(NotSet, |v| Set(v)),
        is_public: body.is_public.map_or(NotSet, |v| Set(v)),

        member_limit_min: body.member_limit_min.map_or(NotSet, |v| Set(v)),
        member_limit_max: body.member_limit_max.map_or(NotSet, |v| Set(v)),
        parallel_container_limit: body.parallel_container_limit.map_or(NotSet, |v| Set(v)),

        is_need_write_up: body.is_need_write_up.map_or(NotSet, |v| Set(v)),
        started_at: body.started_at.map_or(NotSet, |v| Set(v)),
        ended_at: body.ended_at.map_or(NotSet, |v| Set(v)),
        frozed_at: body.frozed_at.map_or(NotSet, |v| Set(v)),
        ..Default::default()
    }
    .update(&get_db())
    .await?;

    return Ok(WebResult {
        code: StatusCode::OK.as_u16(),
        data: Some(game),
        ..WebResult::default()
    });
}

pub async fn delete(
    Extension(ext): Extension<Ext>, Path(id): Path<i64>,
) -> Result<WebResult<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(String::new()))?;
    if operator.group != Group::Admin {
        return Err(WebError::Forbidden(String::new()));
    }

    let _ = crate::model::game::Entity::delete_by_id(id)
        .exec(&get_db())
        .await?;

    return Ok(WebResult {
        code: StatusCode::OK.as_u16(),
        ..WebResult::default()
    });
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
) -> Result<WebResult<Vec<crate::model::game_challenge::Model>>, WebError> {
    let _ = ext.operator.ok_or(WebError::Unauthorized(String::new()))?;

    let (game_challenges, _) =
        crate::model::game_challenge::find(params.game_id, params.challenge_id, params.is_enabled)
            .await?;

    return Ok(WebResult {
        code: StatusCode::OK.as_u16(),
        data: Some(game_challenges),
        ..WebResult::default()
    });
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
) -> Result<WebResult<crate::model::game_challenge::Model>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(String::new()))?;
    if operator.group != Group::Admin {
        return Err(WebError::Forbidden(String::new()));
    }

    let game_challenge = crate::model::game_challenge::ActiveModel {
        game_id: Set(body.game_id),
        challenge_id: Set(body.challenge_id),
        difficulty: body.difficulty.map_or(NotSet, |v| Set(v)),
        is_enabled: body.is_enabled.map_or(NotSet, |v| Set(v)),
        max_pts: body.max_pts.map_or(NotSet, |v| Set(v)),
        min_pts: body.min_pts.map_or(NotSet, |v| Set(v)),
        first_blood_reward_ratio: body.first_blood_reward_ratio.map_or(NotSet, |v| Set(v)),
        second_blood_reward_ratio: body.second_blood_reward_ratio.map_or(NotSet, |v| Set(v)),
        third_blood_reward_ratio: body.third_blood_reward_ratio.map_or(NotSet, |v| Set(v)),
        ..Default::default()
    }
    .insert(&get_db())
    .await?;

    return Ok(WebResult {
        code: StatusCode::OK.as_u16(),
        data: Some(game_challenge),
        ..WebResult::default()
    });
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
) -> Result<WebResult<crate::model::game_challenge::Model>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(String::new()))?;
    if operator.group != Group::Admin {
        return Err(WebError::Forbidden(String::new()));
    }

    body.game_id = Some(id);
    body.challenge_id = Some(challenge_id);

    let game_challenge = crate::model::game_challenge::ActiveModel {
        game_id: body.game_id.map_or(NotSet, |v| Set(v)),
        challenge_id: body.challenge_id.map_or(NotSet, |v| Set(v)),
        difficulty: body.difficulty.map_or(NotSet, |v| Set(v)),
        is_enabled: body.is_enabled.map_or(NotSet, |v| Set(v)),
        max_pts: body.max_pts.map_or(NotSet, |v| Set(v)),
        min_pts: body.min_pts.map_or(NotSet, |v| Set(v)),
        first_blood_reward_ratio: body.first_blood_reward_ratio.map_or(NotSet, |v| Set(v)),
        second_blood_reward_ratio: body.second_blood_reward_ratio.map_or(NotSet, |v| Set(v)),
        third_blood_reward_ratio: body.third_blood_reward_ratio.map_or(NotSet, |v| Set(v)),
        ..Default::default()
    }
    .update(&get_db())
    .await?;

    return Ok(WebResult {
        code: StatusCode::OK.as_u16(),
        data: Some(game_challenge),
        ..WebResult::default()
    });
}

pub async fn delete_challenge(
    Extension(ext): Extension<Ext>, Path((id, challenge_id)): Path<(i64, i64)>,
) -> Result<WebResult<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(String::new()))?;
    if operator.group != Group::Admin {
        return Err(WebError::Forbidden(String::new()));
    }

    let _ = crate::model::game_challenge::Entity::delete_many()
        .filter(crate::model::game_challenge::Column::GameId.eq(id))
        .filter(crate::model::game_challenge::Column::ChallengeId.eq(challenge_id))
        .exec(&get_db())
        .await?;

    return Ok(WebResult {
        code: StatusCode::OK.as_u16(),
        ..WebResult::default()
    });
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetTeamRequest {
    pub game_id: Option<i64>,
    pub team_id: Option<i64>,
}

pub async fn get_team(
    Extension(ext): Extension<Ext>, Query(params): Query<GetTeamRequest>,
) -> Result<WebResult<Vec<crate::model::game_team::Model>>, WebError> {
    let _ = ext.operator.ok_or(WebError::Unauthorized(String::new()))?;

    let (game_teams, total) = crate::model::game_team::find(params.game_id, params.team_id).await?;

    return Ok(WebResult {
        code: StatusCode::OK.as_u16(),
        data: Some(game_teams),
        total: Some(total),
        ..WebResult::default()
    });
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateTeamRequest {
    pub game_id: i64,
    pub team_id: i64,
}

pub async fn create_team(
    Extension(ext): Extension<Ext>, Json(body): Json<CreateTeamRequest>,
) -> Result<WebResult<crate::model::game_team::Model>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(String::new()))?;
    if operator.group != Group::Admin {
        return Err(WebError::Forbidden(String::new()));
    }

    let game_team = crate::model::game_team::ActiveModel {
        game_id: Set(body.game_id),
        team_id: Set(body.team_id),

        ..Default::default()
    }
    .insert(&get_db())
    .await?;

    return Ok(WebResult {
        code: StatusCode::OK.as_u16(),
        data: Some(game_team),
        ..WebResult::default()
    });
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
) -> Result<WebResult<crate::model::game_team::Model>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(String::new()))?;
    if operator.group != Group::Admin {
        return Err(WebError::Forbidden(String::new()));
    }

    body.game_id = Some(id);
    body.team_id = Some(team_id);

    let game_team = crate::model::game_team::ActiveModel {
        game_id: body.game_id.map_or(NotSet, |v| Set(v)),
        team_id: body.team_id.map_or(NotSet, |v| Set(v)),
        is_allowed: body.is_allowed.map_or(NotSet, |v| Set(v)),
        ..Default::default()
    }
    .update(&get_db())
    .await?;

    return Ok(WebResult {
        code: StatusCode::OK.as_u16(),
        data: Some(game_team),
        ..WebResult::default()
    });
}

pub async fn delete_team(
    Extension(ext): Extension<Ext>, Path((id, team_id)): Path<(i64, i64)>,
) -> Result<WebResult<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(String::new()))?;
    if operator.group != Group::Admin {
        return Err(WebError::Forbidden(String::new()));
    }

    let _ = crate::model::game_team::Entity::delete_many()
        .filter(crate::model::game_team::Column::GameId.eq(id))
        .filter(crate::model::game_team::Column::TeamId.eq(team_id))
        .exec(&get_db())
        .await?;

    return Ok(WebResult {
        code: StatusCode::OK.as_u16(),
        ..WebResult::default()
    });
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
    let operator = ext.operator.ok_or(WebError::Unauthorized(String::new()))?;
    if operator.group != Group::Admin {
        return Err(WebError::Forbidden(String::new()));
    }

    crate::queue::publish("calculator", calculator::Payload { game_id: Some(id) }).await?;

    return Ok(WebResult {
        code: StatusCode::OK.as_u16(),
        ..WebResult::default()
    });
}

// pub async fn get_submission(
//     Path(id): Path<i64>, Query(params): Query<GetSubmissionRequest>,
// ) -> Result<impl IntoResponse, WebError> {
//     let submissions = crate::model::submission::get_with_pts(id,
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
//         crate::model::submission::get_with_pts(id,
// Some(crate::model::submission::Status::Correct))             .await;

//     let game_teams = crate::model::game_team::Entity::find()
//         .filter(
//             Condition::all()
//                 .add(crate::model::game_team::Column::GameId.eq(id))
//                 .add(crate::model::game_team::Column::IsAllowed.eq(true)),
//         )
//         .all(&get_db())
//         .await?;

//     return Ok(());
// }

pub async fn get_poster(Path(id): Path<i64>) -> Result<impl IntoResponse, WebError> {
    let path = format!("games/{}/poster", id);
    match crate::media::scan_dir(path.clone()).await.unwrap().first() {
        Some((filename, _size)) => {
            let buffer = crate::media::get(path, filename.to_string()).await.unwrap();
            return Ok(Response::builder().body(Body::from(buffer)).unwrap());
        }
        None => return Err(WebError::NotFound(String::new())),
    }
}

pub async fn get_poster_metadata(Path(id): Path<i64>) -> Result<WebResult<Metadata>, WebError> {
    let path = format!("games/{}/poster", id);
    match crate::media::scan_dir(path.clone()).await.unwrap().first() {
        Some((filename, size)) => {
            return Ok(WebResult {
                code: StatusCode::OK.as_u16(),
                data: Some(Metadata {
                    filename: filename.to_string(),
                    size: *size,
                }),
                ..WebResult::default()
            });
        }
        None => {
            return Err(WebError::NotFound(String::new()));
        }
    }
}

pub async fn save_poster(
    Extension(ext): Extension<Ext>, Path(id): Path<i64>, mut multipart: Multipart,
) -> Result<WebResult<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(String::new()))?;
    if operator.group != Group::Admin {
        return Err(WebError::Forbidden(String::new()));
    }

    let path = format!("games/{}/poster", id);
    let mut filename = String::new();
    let mut data = Vec::<u8>::new();
    while let Some(field) = multipart.next_field().await.unwrap() {
        if field.name() == Some("file") {
            filename = field.file_name().unwrap().to_string();
            let content_type = field.content_type().unwrap().to_string();
            let mime: Mime = content_type.parse().unwrap();
            if mime.type_() != mime::IMAGE {
                return Err(WebError::BadRequest(String::from("forbidden_file_type")));
            }
            data = match field.bytes().await {
                Ok(bytes) => bytes.to_vec(),
                Err(_err) => {
                    return Err(WebError::BadRequest(String::from("size_too_large")));
                }
            };
        }
    }

    crate::media::delete(path.clone()).await.unwrap();

    let _ = crate::media::save(path, filename, data)
        .await
        .map_err(|_| WebError::InternalServerError(String::new()))?;

    return Ok(WebResult {
        code: StatusCode::OK.as_u16(),
        ..WebResult::default()
    });
}

pub async fn delete_poster(
    Extension(ext): Extension<Ext>, Path(id): Path<i64>,
) -> Result<WebResult<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(String::new()))?;
    if operator.group != Group::Admin {
        return Err(WebError::Forbidden(String::new()));
    }

    let path = format!("games/{}/poster", id);

    let _ = crate::media::delete(path)
        .await
        .map_err(|_| WebError::InternalServerError(String::new()))?;

    return Ok(WebResult {
        code: StatusCode::OK.as_u16(),
        ..WebResult::default()
    });
}
