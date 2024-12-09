use std::collections::HashMap;

use axum::{
    body::Body,
    extract::{DefaultBodyLimit, Multipart, Path, Query},
    http::{header, Response, StatusCode},
    response::IntoResponse,
    Router,
};
use sea_orm::{ActiveModelTrait, ActiveValue::NotSet, EntityTrait, Set};
use serde::{Deserialize, Serialize};
use serde_json::json;
use validator::Validate;

use crate::{
    db::{
        entity::{submission::Status, user::Group},
        get_db,
    },
    web::{
        extract::{Extension, Json},
        model::Metadata,
        traits::{Ext, WebError, WebResult},
    },
};
use crate::web::extract::VJson;

pub fn router() -> Router {
    Router::new()
        .route("/", axum::routing::get(get))
        .route("/", axum::routing::post(create))
        .route("/status", axum::routing::post(get_status))
        .route("/:id", axum::routing::put(update))
        .route("/:id", axum::routing::delete(delete))
        .route("/:id/attachment", axum::routing::get(get_attachment))
        .route(
            "/:id/attachment/metadata",
            axum::routing::get(get_attachment_metadata),
        )
        .route(
            "/:id/attachment",
            axum::routing::post(save_attachment)
                .layer(DefaultBodyLimit::max(512 * 1024 * 1024 /* MB */)),
        )
        .route("/:id/attachment", axum::routing::delete(delete_attachment))
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetRequest {
    pub id: Option<i64>,
    pub title: Option<String>,
    pub category: Option<i32>,
    pub tags: Option<Vec<String>>,
    pub is_practicable: Option<bool>,
    pub is_dynamic: Option<bool>,
    pub is_detailed: Option<bool>,
    pub page: Option<u64>,
    pub size: Option<u64>,
}

pub async fn get(
    Extension(ext): Extension<Ext>, Query(params): Query<GetRequest>,
) -> Result<WebResult<Vec<crate::db::transfer::Challenge>>, WebError> {
    let operator = ext
        .operator
        .ok_or(WebError::Unauthorized(serde_json::json!("")))?;
    if operator.group != Group::Admin && params.is_detailed.unwrap_or(false) {
        return Err(WebError::Forbidden(serde_json::json!("")));
    }

    let (mut challenges, total) = crate::db::transfer::challenge::find(
        params.id,
        params.title,
        params.category,
        params.is_practicable,
        params.is_dynamic,
        params.page,
        params.size,
    )
    .await?;

    for challenge in challenges.iter_mut() {
        let is_detailed = params.is_detailed.unwrap_or(false);
        if !is_detailed {
            challenge.flags.clear();
        }
    }

    Ok(WebResult {
        code: StatusCode::OK.as_u16(),
        data: Some(challenges),
        total: Some(total),
        ..WebResult::default()
    })
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StatusRequest {
    pub cids: Vec<i64>,
    pub user_id: Option<i64>,
    pub team_id: Option<i64>,
    pub game_id: Option<i64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StatusResult {
    pub is_solved: bool,
    pub solved_times: i64,
    pub pts: i64,
    pub bloods: Vec<crate::db::transfer::Submission>,
}

#[axum::debug_handler]
pub async fn get_status(
    Extension(ext): Extension<Ext>, Json(body): Json<StatusRequest>,
) -> Result<WebResult<HashMap<i64, StatusResult>>, WebError> {
    let _ = ext
        .operator
        .ok_or(WebError::Unauthorized(serde_json::json!("")))?;

    let mut submissions =
        crate::db::transfer::submission::get_by_challenge_ids(body.cids.clone()).await?;

    let mut result: HashMap<i64, StatusResult> = HashMap::new();

    for cid in body.cids {
        result.entry(cid).or_insert_with(|| StatusResult {
            is_solved: false,
            solved_times: 0,
            pts: 0,
            bloods: Vec::new(),
        });
    }

    for submission in submissions.iter_mut() {
        submission.desensitize();
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
        }
    }

    if let Some(game_id) = body.game_id {
        let (game_challenges, _) =
            crate::db::transfer::game_challenge::find(Some(game_id), None, None).await?;

        for game_challenge in game_challenges {
            let status_response = result.get_mut(&game_challenge.challenge_id).unwrap();
            status_response.pts = game_challenge.pts;
        }
    }

    Ok(WebResult {
        code: StatusCode::OK.as_u16(),
        data: Some(result),
        ..WebResult::default()
    })
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateRequest {
    pub title: String,
    pub description: String,
    pub category: i32,
    pub tags: Option<Vec<String>>,
    pub is_practicable: Option<bool>,
    pub is_dynamic: Option<bool>,
    pub has_attachment: Option<bool>,
    pub difficulty: Option<i64>,
    pub image_name: Option<String>,
    pub cpu_limit: Option<i64>,
    pub memory_limit: Option<i64>,
    pub duration: Option<i64>,
    pub ports: Option<Vec<i32>>,
    pub envs: Option<Vec<crate::db::entity::challenge::Env>>,
    pub flags: Option<Vec<crate::db::entity::challenge::Flag>>,
}

pub async fn create(
    Extension(ext): Extension<Ext>, Json(body): Json<CreateRequest>,
) -> Result<WebResult<crate::db::transfer::Challenge>, WebError> {
    let operator = ext
        .operator
        .ok_or(WebError::Unauthorized(serde_json::json!("")))?;
    if operator.group != Group::Admin {
        return Err(WebError::Forbidden(serde_json::json!("")));
    }

    let challenge = crate::db::entity::challenge::ActiveModel {
        title: Set(body.title),
        description: Set(Some(body.description)),
        category: Set(body.category),
        tags: Set(body.tags.unwrap_or(vec![])),
        is_practicable: Set(body.is_practicable.unwrap_or(false)),
        is_dynamic: Set(body.is_dynamic.unwrap_or(false)),
        has_attachment: Set(body.has_attachment.unwrap_or(false)),
        image_name: Set(body.image_name),
        cpu_limit: Set(body.cpu_limit.unwrap_or(0)),
        memory_limit: Set(body.memory_limit.unwrap_or(0)),
        duration: Set(body.duration.unwrap_or(1800)),
        ports: Set(body.ports.unwrap_or(vec![])),
        envs: Set(body.envs.unwrap_or(vec![])),
        flags: Set(body.flags.unwrap_or(vec![])),
        ..Default::default()
    }
    .insert(get_db())
    .await?;
    let challenge = crate::db::transfer::Challenge::from(challenge);

    Ok(WebResult {
        code: StatusCode::OK.as_u16(),
        data: Some(challenge),
        ..WebResult::default()
    })
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateRequest {
    pub id: Option<i64>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub category: Option<i32>,
    pub tags: Option<Vec<String>>,
    pub is_practicable: Option<bool>,
    pub is_dynamic: Option<bool>,
    pub has_attachment: Option<bool>,
    pub difficulty: Option<i64>,
    pub image_name: Option<String>,
    pub cpu_limit: Option<i64>,
    pub memory_limit: Option<i64>,
    pub duration: Option<i64>,
    pub ports: Option<Vec<i32>>,
    pub envs: Option<Vec<crate::db::entity::challenge::Env>>,
    pub flags: Option<Vec<crate::db::entity::challenge::Flag>>,
}

pub async fn update(
    Extension(ext): Extension<Ext>, Path(id): Path<i64>, VJson(mut body): VJson<UpdateRequest>,
) -> Result<WebResult<crate::db::transfer::Challenge>, WebError> {
    let operator = ext
        .operator
        .ok_or(WebError::Unauthorized(json!("")))?;
    if operator.group != Group::Admin {
        return Err(WebError::Forbidden(json!("")));
    }

    body.id = Some(id);

    let challenge = crate::db::entity::challenge::ActiveModel {
        id: body.id.map_or(NotSet, Set),
        title: body.title.map_or(NotSet, Set),
        description: body.description.map_or(NotSet, |v| Set(Some(v))),
        tags: body.tags.map_or(NotSet, Set),
        category: body.category.map_or(NotSet, Set),
        is_practicable: body.is_practicable.map_or(NotSet, Set),
        is_dynamic: body.is_dynamic.map_or(NotSet, Set),
        has_attachment: body.has_attachment.map_or(NotSet, Set),
        image_name: body.image_name.map_or(NotSet, |v| Set(Some(v))),
        cpu_limit: body.cpu_limit.map_or(NotSet, Set),
        memory_limit: body.memory_limit.map_or(NotSet, Set),
        duration: body.duration.map_or(NotSet, Set),
        ports: body.ports.map_or(NotSet, Set),
        envs: body.envs.map_or(NotSet, Set),
        flags: body.flags.map_or(NotSet, Set),
        ..Default::default()
    }
    .update(get_db())
    .await?;
    let challenge = crate::db::transfer::Challenge::from(challenge);

    Ok(WebResult {
        code: StatusCode::OK.as_u16(),
        data: Some(challenge),
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

    let _ = crate::db::entity::challenge::Entity::delete_by_id(id)
        .exec(get_db())
        .await?;

    Ok(WebResult {
        code: StatusCode::OK.as_u16(),
        ..WebResult::default()
    })
}

pub async fn get_attachment(
    Extension(ext): Extension<Ext>, Path(id): Path<i64>,
) -> Result<impl IntoResponse, WebError> {
    let _ = ext
        .operator
        .ok_or(WebError::Unauthorized(json!("")))?;
    let path = format!("challenges/{}/attachment", id);
    match crate::media::scan_dir(path.clone()).await?.first() {
        Some((filename, _size)) => {
            let buffer = crate::media::get(path, filename.to_string()).await?;
            Ok(Response::builder()
                .header(header::CONTENT_TYPE, "application/octet-stream")
                .header(
                    header::CONTENT_DISPOSITION,
                    format!("attachment; filename=\"{}\"", filename),
                )
                .body(Body::from(buffer))
                .unwrap())
        }
        None => Err(WebError::NotFound(json!(""))),
    }
}

pub async fn get_attachment_metadata(
    Extension(ext): Extension<Ext>, Path(id): Path<i64>,
) -> Result<WebResult<Metadata>, WebError> {
    let _ = ext
        .operator
        .ok_or(WebError::Unauthorized(json!("")))?;

    let path = format!("challenges/{}/attachment", id);
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

pub async fn save_attachment(
    Extension(ext): Extension<Ext>, Path(id): Path<i64>, mut multipart: Multipart,
) -> Result<WebResult<()>, WebError> {
    let operator = ext
        .operator
        .ok_or(WebError::Unauthorized(json!("")))?;
    if operator.group != Group::Admin {
        return Err(WebError::Forbidden(json!("")));
    }

    let path = format!("challenges/{}/attachment", id);
    let mut filename = String::new();
    let mut data = Vec::<u8>::new();
    while let Some(field) = multipart.next_field().await.unwrap() {
        if field.name() == Some("file") {
            filename = field.file_name().unwrap().to_string();
            data = match field.bytes().await {
                Ok(bytes) => bytes.to_vec(),
                Err(_err) => {
                    return Err(WebError::BadRequest(json!("size_too_large")));
                }
            };
        }
    }

    crate::media::delete_dir(path.clone()).await.unwrap();

    crate::media::save(path, filename, data)
        .await
        .map_err(|_| WebError::InternalServerError(json!("")))?;

    Ok(WebResult {
        code: StatusCode::OK.as_u16(),
        ..WebResult::default()
    })
}

pub async fn delete_attachment(
    Extension(ext): Extension<Ext>, Path(id): Path<i64>,
) -> Result<WebResult<()>, WebError> {
    let operator = ext
        .operator
        .ok_or(WebError::Unauthorized(json!("")))?;
    if operator.group != Group::Admin {
        return Err(WebError::Forbidden(json!("")));
    }

    let path = format!("challenges/{}/attachment", id);

    crate::media::delete_dir(path)
        .await
        .map_err(|_| WebError::InternalServerError(json!("")))?;

    Ok(WebResult {
        code: StatusCode::OK.as_u16(),
        ..WebResult::default()
    })
}
