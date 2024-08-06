use std::collections::HashMap;

use anyhow::anyhow;
use axum::{
    body::Body,
    extract::{Multipart, Path, Query},
    http::{header, Response, StatusCode},
    response::IntoResponse,
    Extension, Json,
};
use sea_orm::{ActiveModelTrait, EntityTrait};
use serde_json::json;

use crate::database::get_db;
use crate::{model::submission::Status, web::traits::Ext};
use crate::{util::validate, web::traits::WebError};

pub async fn get(
    Extension(ext): Extension<Ext>,
    Query(params): Query<crate::model::challenge::request::FindRequest>,
) -> Result<impl IntoResponse, WebError> {
    let operator = ext.operator.unwrap();
    if operator.group != "admin" && params.is_detailed.unwrap_or(false) {
        return Err(WebError::Forbidden(String::new()));
    }

    let (mut challenges, total) = crate::model::challenge::find(
        params.id,
        params.title,
        params.category_id,
        params.is_practicable,
        params.is_dynamic,
        params.page,
        params.size,
    )
    .await
    .map_err(|err| WebError::DatabaseError(err))?;

    for challenge in challenges.iter_mut() {
        let is_detailed = params.is_detailed.unwrap_or(false);
        if !is_detailed {
            challenge.flags.clear();
        }
    }

    return Ok((
        StatusCode::OK,
        Json(json!({
            "code": StatusCode::OK.as_u16(),
            "data": json!(challenges),
            "total": total,
        })),
    ));
}

pub async fn get_status(
    Json(body): Json<crate::model::challenge::request::StatusRequest>,
) -> Result<impl IntoResponse, WebError> {
    let mut submissions = crate::model::submission::find_by_challenge_ids(body.cids.clone())
        .await
        .unwrap();

    let mut result: HashMap<i64, crate::model::challenge::response::StatusResponse> =
        HashMap::new();

    for cid in body.cids {
        result
            .entry(cid)
            .or_insert_with(|| crate::model::challenge::response::StatusResponse {
                is_solved: false,
                solved_times: 0,
                pts: 0,
                bloods: Vec::new(),
            });
    }

    for submission in submissions.iter_mut() {
        submission.simplify();
        submission.challenge = None;

        if body.game_id.is_some() {
            submission.game = None;
            if submission.game_id != body.game_id {
                continue;
            }
        }

        if submission.status != Status::Correct {
            continue;
        }

        let status_response = result.get_mut(&submission.challenge_id).unwrap();

        if let Some(user_id) = body.user_id {
            if submission.user_id == user_id {
                status_response.is_solved = true;
            }
        }

        if let Some(team_id) = body.team_id {
            if let Some(game_id) = body.game_id {
                if submission.team_id == Some(team_id) && submission.game_id == Some(game_id) {
                    status_response.is_solved = true;
                }
            }
        }

        status_response.solved_times += 1;
        if status_response.bloods.len() < 3 {
            status_response.bloods.push(submission.clone());
            status_response
                .bloods
                .sort_by(|a, b| a.created_at.cmp(&b.created_at));
        } else {
            let last_submission = status_response.bloods.last().unwrap();
            if submission.created_at < last_submission.created_at {
                status_response.bloods.pop();
                status_response.bloods.push(submission.clone());
                status_response
                    .bloods
                    .sort_by(|a, b| a.created_at.cmp(&b.created_at));
            }
        }
    }

    if let Some(game_id) = body.game_id {
        let (game_challenges, _) = crate::model::game_challenge::find(Some(game_id), None, None)
            .await
            .unwrap();

        for game_challenge in game_challenges {
            let status_response = result.get_mut(&game_challenge.challenge_id).unwrap();
            status_response.pts = crate::util::math::curve(
                game_challenge.max_pts,
                game_challenge.min_pts,
                game_challenge.difficulty,
                status_response.solved_times,
            );
            status_response.pts = match status_response.solved_times {
                0 => status_response.pts * (100 + game_challenge.first_blood_reward_ratio) / 100,
                1 => status_response.pts * (100 + game_challenge.second_blood_reward_ratio) / 100,
                2 => status_response.pts * (100 + game_challenge.third_blood_reward_ratio) / 100,
                _ => status_response.pts,
            }
        }
    }

    return Ok((
        StatusCode::OK,
        Json(json!({
            "code": StatusCode::OK.as_u16(),
            "data": json!(result),
        })),
    ));
}

pub async fn create(
    Json(body): Json<crate::model::challenge::request::CreateRequest>,
) -> Result<impl IntoResponse, WebError> {
    let challenge = crate::model::challenge::ActiveModel::from(body)
        .insert(&get_db())
        .await
        .map_err(|err| WebError::DatabaseError(err))?;

    return Ok((
        StatusCode::OK,
        Json(json!({
            "code": StatusCode::OK.as_u16(),
            "data": json!(challenge),
        })),
    ));
}

pub async fn update(
    Path(id): Path<i64>,
    validate::Json(mut body): validate::Json<crate::model::challenge::request::UpdateRequest>,
) -> Result<impl IntoResponse, WebError> {
    body.id = Some(id);

    let challenge = crate::model::challenge::ActiveModel::from(body)
        .update(&get_db())
        .await
        .map_err(|err| WebError::DatabaseError(err))?;

    return Ok((
        StatusCode::OK,
        Json(json!({
            "code": StatusCode::OK.as_u16(),
            "data": json!(challenge),
        })),
    ));
}

pub async fn delete(Path(id): Path<i64>) -> Result<impl IntoResponse, WebError> {
    let _ = crate::model::challenge::Entity::delete_by_id(id)
        .exec(&get_db())
        .await
        .map_err(|err| WebError::DatabaseError(err))?;

    return Ok((
        StatusCode::OK,
        Json(json!({
            "code": StatusCode::OK.as_u16(),
        })),
    ));
}

pub async fn get_attachment(Path(id): Path<i64>) -> Result<impl IntoResponse, WebError> {
    let path = format!("challenges/{}/attachment", id);
    match crate::media::scan_dir(path.clone()).await.unwrap().first() {
        Some((filename, _size)) => {
            let buffer = crate::media::get(path, filename.to_string()).await.unwrap();
            return Ok(Response::builder()
                .header(header::CONTENT_TYPE, "application/octet-stream")
                .header(
                    header::CONTENT_DISPOSITION,
                    format!("attachment; filename=\"{}\"", filename),
                )
                .body(Body::from(buffer))
                .unwrap());
        }
        None => return Err(WebError::NotFound(String::new())),
    }
}

pub async fn get_attachment_metadata(Path(id): Path<i64>) -> Result<impl IntoResponse, WebError> {
    let path = format!("challenges/{}/attachment", id);
    match crate::media::scan_dir(path.clone()).await.unwrap().first() {
        Some((filename, size)) => {
            return Ok((
                StatusCode::OK,
                Json(json!({
                    "code": StatusCode::OK.as_u16(),
                    "data": {
                        "filename": filename,
                        "size": size,
                    },
                })),
            ));
        }
        None => return Err(WebError::NotFound(String::new())),
    }
}

pub async fn save_attachment(
    Path(id): Path<i64>, mut multipart: Multipart,
) -> Result<impl IntoResponse, WebError> {
    let path = format!("challenges/{}/attachment", id);
    let mut filename = String::new();
    let mut data = Vec::<u8>::new();
    while let Some(field) = multipart.next_field().await.unwrap() {
        if field.name() == Some("file") {
            filename = field.file_name().unwrap().to_string();
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
        Json(json!({
            "code": StatusCode::OK.as_u16(),
        })),
    ));
}

pub async fn delete_attachment(Path(id): Path<i64>) -> Result<impl IntoResponse, WebError> {
    let path = format!("challenges/{}/attachment", id);

    let _ = crate::media::delete(path)
        .await
        .map_err(|_| WebError::InternalServerError(String::new()))?;

    return Ok((
        StatusCode::OK,
        Json(json!({
            "code": StatusCode::OK.as_u16(),
        })),
    ));
}
