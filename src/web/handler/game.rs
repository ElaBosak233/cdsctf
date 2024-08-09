use axum::body::Body;
use axum::{
    extract::{Multipart, Path, Query},
    http::{Response, StatusCode},
    response::IntoResponse,
    Extension, Json,
};
use mime::Mime;
use sea_orm::ActiveValue::NotSet;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;
use sea_orm::{ActiveModelTrait, Set};
use sea_orm::{ColumnTrait, Condition};

use crate::database::get_db;
use crate::web::model::{game::*, Metadata};
use crate::web::traits::Ext;
use crate::web::traits::WebError;

pub async fn get(
    Extension(ext): Extension<Ext>, Query(params): Query<GetRequest>,
) -> Result<impl IntoResponse, WebError> {
    let operator = ext.operator.unwrap();
    if operator.group != "admin" && !params.is_enabled.unwrap_or(true) {
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

    return Ok((
        StatusCode::OK,
        Json(GetResponse {
            code: StatusCode::OK.as_u16(),
            data: games,
            total: total,
        }),
    ));
}

pub async fn create(Json(body): Json<CreateRequest>) -> Result<impl IntoResponse, WebError> {
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

    return Ok((
        StatusCode::OK,
        Json(CreateResponse {
            code: StatusCode::OK.as_u16(),
            data: game,
        }),
    ));
}

pub async fn update(
    Path(id): Path<i64>, Json(mut body): Json<UpdateRequest>,
) -> Result<impl IntoResponse, WebError> {
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

    return Ok((
        StatusCode::OK,
        Json(UpdateResponse {
            code: StatusCode::OK.as_u16(),
            data: game,
        }),
    ));
}

pub async fn delete(Path(id): Path<i64>) -> Result<impl IntoResponse, WebError> {
    let _ = crate::model::game::Entity::delete_by_id(id)
        .exec(&get_db())
        .await?;

    return Ok((
        StatusCode::OK,
        Json(DeleteResponse {
            code: StatusCode::OK.as_u16(),
        }),
    ));
}

pub async fn get_challenge(
    Query(params): Query<GetChallengeRequest>,
) -> Result<impl IntoResponse, WebError> {
    let (game_challenges, _) =
        crate::model::game_challenge::find(params.game_id, params.challenge_id, params.is_enabled)
            .await?;

    return Ok((
        StatusCode::OK,
        Json(GetChallengeResponse {
            code: StatusCode::OK.as_u16(),
            data: game_challenges,
        }),
    ));
}

pub async fn create_challenge(
    Json(body): Json<CreateChallengeRequest>,
) -> Result<impl IntoResponse, WebError> {
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

    return Ok((
        StatusCode::OK,
        Json(CreateChallengeResponse {
            code: StatusCode::OK.as_u16(),
            data: game_challenge,
        }),
    ));
}

pub async fn update_challenge(
    Path((id, challenge_id)): Path<(i64, i64)>, Json(mut body): Json<UpdateChallengeRequest>,
) -> Result<impl IntoResponse, WebError> {
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

    return Ok((
        StatusCode::OK,
        Json(UpdateChallengeResponse {
            code: StatusCode::OK.as_u16(),
            data: game_challenge,
        }),
    ));
}

pub async fn delete_challenge(
    Path((id, challenge_id)): Path<(i64, i64)>,
) -> Result<impl IntoResponse, WebError> {
    let _ = crate::model::game_challenge::Entity::delete_many()
        .filter(crate::model::game_challenge::Column::GameId.eq(id))
        .filter(crate::model::game_challenge::Column::ChallengeId.eq(challenge_id))
        .exec(&get_db())
        .await?;

    return Ok((
        StatusCode::OK,
        Json(DeleteChallengeResponse {
            code: StatusCode::OK.as_u16(),
        }),
    ));
}

pub async fn get_team(Query(params): Query<GetTeamRequest>) -> Result<impl IntoResponse, WebError> {
    let (game_teams, total) = crate::model::game_team::find(params.game_id, params.team_id).await?;

    return Ok((
        StatusCode::OK,
        Json(GetTeamResponse {
            code: StatusCode::OK.as_u16(),
            data: game_teams,
            total: total,
        }),
    ));
}

pub async fn create_team(
    Json(body): Json<CreateTeamRequest>,
) -> Result<impl IntoResponse, WebError> {
    let game_team = crate::model::game_team::ActiveModel {
        game_id: Set(body.game_id),
        team_id: Set(body.team_id),

        ..Default::default()
    }
    .insert(&get_db())
    .await?;

    return Ok((
        StatusCode::OK,
        Json(CreateTeamResponse {
            code: StatusCode::OK.as_u16(),
            data: game_team,
        }),
    ));
}

pub async fn update_team(
    Path((id, team_id)): Path<(i64, i64)>, Json(mut body): Json<UpdateTeamRequest>,
) -> Result<impl IntoResponse, WebError> {
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

    return Ok((
        StatusCode::OK,
        Json(UpdateTeamResponse {
            code: StatusCode::OK.as_u16(),
            data: game_team,
        }),
    ));
}

pub async fn delete_team(
    Path((id, team_id)): Path<(i64, i64)>,
) -> Result<impl IntoResponse, WebError> {
    let _ = crate::model::game_team::Entity::delete_many()
        .filter(crate::model::game_team::Column::GameId.eq(id))
        .filter(crate::model::game_team::Column::TeamId.eq(team_id))
        .exec(&get_db())
        .await?;

    return Ok((
        StatusCode::OK,
        Json(DeleteTeamResponse {
            code: StatusCode::OK.as_u16(),
        }),
    ));
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

// pub async fn get_submission(
//     Path(id): Path<i64>, Query(params): Query<GetSubmissionRequest>,
// ) -> Result<impl IntoResponse, WebError> {
//     let submissions = crate::model::submission::get_with_pts(id, params.status).await?;

//     return Ok((
//         StatusCode::OK,
//         Json(GetSubmissionResponse {
//             code: StatusCode::OK.as_u16(),
//             data: submissions,
//         }),
//     ));
// }

// pub async fn get_scoreboard(Path(id): Path<i64>) -> Result<impl IntoResponse, WebError> {
//     pub struct TeamScoreRecord {}

//     let submissions =
//         crate::model::submission::get_with_pts(id, Some(crate::model::submission::Status::Correct))
//             .await;

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

pub async fn get_poster_metadata(Path(id): Path<i64>) -> Result<impl IntoResponse, WebError> {
    let path = format!("games/{}/poster", id);
    match crate::media::scan_dir(path.clone()).await.unwrap().first() {
        Some((filename, size)) => {
            return Ok((
                StatusCode::OK,
                Json(GetPosterMetadataResponse {
                    code: StatusCode::OK.as_u16(),
                    data: Metadata {
                        filename: filename.to_string(),
                        size: *size,
                    },
                }),
            ));
        }
        None => {
            return Err(WebError::NotFound(String::new()));
        }
    }
}

pub async fn save_poster(
    Path(id): Path<i64>, mut multipart: Multipart,
) -> Result<impl IntoResponse, WebError> {
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

    return Ok((
        StatusCode::OK,
        Json(SavePosterResponse {
            code: StatusCode::OK.as_u16(),
        }),
    ));
}

pub async fn delete_poster(Path(id): Path<i64>) -> Result<impl IntoResponse, WebError> {
    let path = format!("games/{}/poster", id);

    let _ = crate::media::delete(path)
        .await
        .map_err(|_| WebError::InternalServerError(String::new()))?;

    return Ok((
        StatusCode::OK,
        Json(DeletePosterResponse {
            code: StatusCode::OK.as_u16(),
        }),
    ));
}
