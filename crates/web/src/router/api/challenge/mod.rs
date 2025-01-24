use std::collections::HashMap;

use axum::{
    Router,
    body::Body,
    extract::{DefaultBodyLimit, Multipart},
    http::{Response, StatusCode, header},
    response::IntoResponse,
};
use cds_db::{
    entity::{submission::Status, user::Group},
    get_db,
};
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set},
    EntityTrait,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use validator::Validate;

use crate::{
    extract::{Extension, Json, Path, Query, VJson},
    model::Metadata,
    traits::{Ext, WebError, WebResponse},
};

pub fn router() -> Router {
    Router::new()
        .route("/", axum::routing::get(get))
        .route("/", axum::routing::post(create))
        .route("/status", axum::routing::post(get_status))
        .route("/{id}", axum::routing::put(update))
        .route("/{id}", axum::routing::delete(delete))
        .route("/{id}/attachment", axum::routing::get(get_attachment))
        .route(
            "/{id}/attachment/metadata",
            axum::routing::get(get_attachment_metadata),
        )
        .route(
            "/{id}/attachment",
            axum::routing::post(save_attachment)
                .layer(DefaultBodyLimit::max(512 * 1024 * 1024 /* MB */)),
        )
        .route("/{id}/attachment", axum::routing::delete(delete_attachment))
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetRequest {
    pub id: Option<uuid::Uuid>,
    pub title: Option<String>,
    pub category: Option<i32>,
    pub tags: Option<Vec<String>>,
    pub is_public: Option<bool>,
    pub is_dynamic: Option<bool>,
    pub is_detailed: Option<bool>,
    pub is_deleted: Option<bool>,
    pub page: Option<u64>,
    pub size: Option<u64>,
    pub sorts: Option<String>,
}

pub async fn get(
    Extension(ext): Extension<Ext>, Query(params): Query<GetRequest>,
) -> Result<WebResponse<Vec<cds_db::transfer::Challenge>>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    if operator.group != Group::Admin && params.is_detailed.unwrap_or(false) {
        return Err(WebError::Forbidden(json!("")));
    }

    let (mut challenges, total) = cds_db::transfer::challenge::find(
        params.id,
        params.title,
        params.category,
        params.is_public,
        params.is_dynamic,
        params.is_deleted,
        params.sorts,
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

    Ok(WebResponse {
        code: StatusCode::OK.as_u16(),
        data: Some(challenges),
        total: Some(total),
        ..WebResponse::default()
    })
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StatusRequest {
    pub challenge_ids: Vec<uuid::Uuid>,
    pub user_id: Option<i64>,
    pub team_id: Option<i64>,
    pub game_id: Option<i64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StatusResponse {
    pub is_solved: bool,
    pub solved_times: i64,
    pub pts: i64,
    pub bloods: Vec<cds_db::transfer::Submission>,
}

#[axum::debug_handler]
pub async fn get_status(
    Extension(ext): Extension<Ext>, Json(body): Json<StatusRequest>,
) -> Result<WebResponse<HashMap<uuid::Uuid, StatusResponse>>, WebError> {
    let _ = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;

    let mut submissions =
        cds_db::transfer::submission::get_by_challenge_ids(body.challenge_ids.clone()).await?;

    let mut result: HashMap<uuid::Uuid, StatusResponse> = HashMap::new();

    for challenge_id in body.challenge_ids {
        result
            .entry(challenge_id)
            .or_insert_with(|| StatusResponse {
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
            cds_db::transfer::game_challenge::find(Some(game_id), None, None).await?;

        for game_challenge in game_challenges {
            let status_response = result.get_mut(&game_challenge.challenge_id).unwrap();
            status_response.pts = game_challenge.pts;
        }
    }

    Ok(WebResponse {
        code: StatusCode::OK.as_u16(),
        data: Some(result),
        ..WebResponse::default()
    })
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateRequest {
    pub title: String,
    pub description: Option<String>,
    pub category: i32,
    pub tags: Option<Vec<String>>,
    pub is_public: Option<bool>,
    pub is_dynamic: Option<bool>,
    pub has_attachment: Option<bool>,
    pub image_name: Option<String>,
    pub env: Option<cds_db::entity::challenge::Env>,
    pub flags: Option<Vec<cds_db::entity::challenge::Flag>>,
}

pub async fn create(
    Extension(ext): Extension<Ext>, Json(body): Json<CreateRequest>,
) -> Result<WebResponse<cds_db::transfer::Challenge>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    if operator.group != Group::Admin {
        return Err(WebError::Forbidden(json!("")));
    }

    let challenge = cds_db::entity::challenge::ActiveModel {
        title: Set(body.title),
        description: Set(body.description),
        category: Set(body.category),
        tags: Set(body.tags.unwrap_or(vec![])),
        is_public: Set(body.is_public.unwrap_or(false)),
        is_dynamic: Set(body.is_dynamic.unwrap_or(false)),
        has_attachment: Set(body.has_attachment.unwrap_or(false)),
        env: Set(body.env),
        flags: Set(body.flags.unwrap_or(vec![])),
        ..Default::default()
    }
    .insert(get_db())
    .await?;
    let challenge = cds_db::transfer::Challenge::from(challenge);

    Ok(WebResponse {
        code: StatusCode::OK.as_u16(),
        data: Some(challenge),
        ..WebResponse::default()
    })
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateRequest {
    pub id: Option<uuid::Uuid>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub category: Option<i32>,
    pub tags: Option<Vec<String>>,
    pub is_public: Option<bool>,
    pub is_dynamic: Option<bool>,
    pub has_attachment: Option<bool>,
    pub env: Option<cds_db::entity::challenge::Env>,
    pub flags: Option<Vec<cds_db::entity::challenge::Flag>>,
}

pub async fn update(
    Extension(ext): Extension<Ext>, Path(id): Path<uuid::Uuid>,
    VJson(mut body): VJson<UpdateRequest>,
) -> Result<WebResponse<cds_db::transfer::Challenge>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    if operator.group != Group::Admin {
        return Err(WebError::Forbidden(json!("")));
    }

    body.id = Some(id);

    let challenge = cds_db::entity::challenge::ActiveModel {
        id: body.id.map_or(NotSet, Set),
        title: body.title.map_or(NotSet, Set),
        description: body.description.map_or(NotSet, |v| Set(Some(v))),
        tags: body.tags.map_or(NotSet, Set),
        category: body.category.map_or(NotSet, Set),
        is_public: body.is_public.map_or(NotSet, Set),
        is_dynamic: body.is_dynamic.map_or(NotSet, Set),
        has_attachment: body.has_attachment.map_or(NotSet, Set),
        env: body.env.map_or(NotSet, |v| Set(Some(v))),
        flags: body.flags.map_or(NotSet, Set),
        created_at: NotSet,
        ..Default::default()
    }
    .update(get_db())
    .await?;
    let challenge = cds_db::transfer::Challenge::from(challenge);

    Ok(WebResponse {
        code: StatusCode::OK.as_u16(),
        data: Some(challenge),
        ..WebResponse::default()
    })
}

pub async fn delete(
    Extension(ext): Extension<Ext>, Path(id): Path<uuid::Uuid>,
) -> Result<WebResponse<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    if operator.group != Group::Admin {
        return Err(WebError::Forbidden(json!("")));
    }

    let _ = cds_db::entity::challenge::ActiveModel {
        id: Set(id),
        is_deleted: Set(true),
        ..Default::default()
    }
    .update(get_db())
    .await?;

    Ok(WebResponse {
        code: StatusCode::OK.as_u16(),
        ..WebResponse::default()
    })
}

pub async fn get_attachment(
    Extension(ext): Extension<Ext>, Path(id): Path<uuid::Uuid>,
) -> Result<impl IntoResponse, WebError> {
    let _ = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    let path = format!("challenges/{}/attachment", id);
    match cds_media::scan_dir(path.clone()).await?.first() {
        Some((filename, _size)) => {
            let buffer = cds_media::get(path, filename.to_string()).await?;
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
    Extension(ext): Extension<Ext>, Path(id): Path<uuid::Uuid>,
) -> Result<WebResponse<Metadata>, WebError> {
    let _ = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;

    let path = format!("challenges/{}/attachment", id);
    match cds_media::scan_dir(path.clone()).await?.first() {
        Some((filename, size)) => Ok(WebResponse {
            code: StatusCode::OK.as_u16(),
            data: Some(Metadata {
                filename: filename.to_string(),
                size: *size,
            }),
            ..WebResponse::default()
        }),
        None => Err(WebError::NotFound(json!(""))),
    }
}

pub async fn save_attachment(
    Extension(ext): Extension<Ext>, Path(id): Path<uuid::Uuid>, mut multipart: Multipart,
) -> Result<WebResponse<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
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

    cds_media::delete_dir(path.clone()).await.unwrap();

    cds_media::save(path, filename, data)
        .await
        .map_err(|_| WebError::InternalServerError(json!("")))?;

    Ok(WebResponse {
        code: StatusCode::OK.as_u16(),
        ..WebResponse::default()
    })
}

pub async fn delete_attachment(
    Extension(ext): Extension<Ext>, Path(id): Path<uuid::Uuid>,
) -> Result<WebResponse<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    if operator.group != Group::Admin {
        return Err(WebError::Forbidden(json!("")));
    }

    let path = format!("challenges/{}/attachment", id);

    cds_media::delete_dir(path)
        .await
        .map_err(|_| WebError::InternalServerError(json!("")))?;

    Ok(WebResponse {
        code: StatusCode::OK.as_u16(),
        ..WebResponse::default()
    })
}
