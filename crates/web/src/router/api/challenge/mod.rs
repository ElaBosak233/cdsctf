mod challenge_id;

use std::{collections::HashMap, str::FromStr};

use axum::{http::StatusCode, response::IntoResponse, Router};
use cds_db::{
    entity::{submission::Status, user::Group},
    get_db,
    transfer::Challenge,
};
use sea_orm::{
    sea_query::Expr, ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityName, EntityTrait, Iden,
    IdenStatic, Order, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use validator::Validate;

use crate::{
    extract::{Extension, Json, Query},
    traits::{Ext, WebError, WebResponse},
};

pub fn router() -> Router {
    Router::new()
        .route("/", axum::routing::get(get_challenge))
        .route("/", axum::routing::post(create_challenge))
        .route("/status", axum::routing::post(get_challenge_status))
        .nest("/{challenge_id}", challenge_id::router())
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetChallengeRequest {
    pub id: Option<uuid::Uuid>,
    pub title: Option<String>,
    pub category: Option<i32>,
    pub tags: Option<String>,
    pub is_public: Option<bool>,
    pub is_dynamic: Option<bool>,
    pub page: Option<u64>,
    pub size: Option<u64>,
    pub sorts: Option<String>,

    /// Whether the expected challenges are desensitized.
    /// If you are not an admin, this must be true,
    /// or you will be forbidden.
    pub is_desensitized: Option<bool>,
}

pub async fn get_challenge(
    Extension(ext): Extension<Ext>, Query(params): Query<GetChallengeRequest>,
) -> Result<WebResponse<Vec<Challenge>>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    let is_desensitized = params.is_desensitized.unwrap_or(true);

    if operator.group != Group::Admin && (!is_desensitized || !params.is_public.unwrap_or(false)) {
        return Err(WebError::Forbidden(json!("")));
    }

    let mut sql = cds_db::entity::challenge::Entity::find();

    if let Some(id) = params.id {
        sql = sql.filter(cds_db::entity::challenge::Column::Id.eq(id));
    }

    if let Some(title) = params.title {
        sql = sql.filter(cds_db::entity::challenge::Column::Title.contains(title));
    }

    if let Some(category) = params.category {
        sql = sql.filter(cds_db::entity::challenge::Column::Category.eq(category));
    }

    if let Some(tags) = params.tags {
        let tags = tags
            .split(",")
            .map(|s| s.to_owned())
            .collect::<Vec<String>>();

        sql = sql.filter(Expr::cust_with_expr(
            format!(
                "\"{}\".\"{}\" @> $1::varchar[]",
                cds_db::entity::challenge::Entity.table_name(),
                cds_db::entity::challenge::Column::Tags.to_string()
            )
            .as_str(),
            tags,
        ))
    }

    if let Some(is_public) = params.is_public {
        sql = sql.filter(cds_db::entity::challenge::Column::IsPublic.eq(is_public));
    }

    if let Some(is_dynamic) = params.is_dynamic {
        sql = sql.filter(cds_db::entity::challenge::Column::IsDynamic.eq(is_dynamic));
    }

    sql = sql.filter(cds_db::entity::challenge::Column::DeletedAt.is_null());

    let total = sql.clone().count(get_db()).await?;

    if let Some(sorts) = params.sorts {
        let sorts = sorts.split(",").collect::<Vec<&str>>();
        for sort in sorts {
            let col =
                match cds_db::entity::challenge::Column::from_str(sort.replace("-", "").as_str()) {
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

    let mut challenges = sql
        .all(get_db())
        .await?
        .into_iter()
        .map(Challenge::from)
        .collect::<Vec<Challenge>>();

    if is_desensitized {
        for challenge in challenges.iter_mut() {
            challenge.desensitize();
        }
    }

    Ok(WebResponse {
        code: StatusCode::OK,
        data: Some(challenges),
        total: Some(total),
        ..Default::default()
    })
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetChallengeStatusRequest {
    pub challenge_ids: Vec<uuid::Uuid>,
    pub user_id: Option<i64>,
    pub game_team_id: Option<i64>,
    pub game_id: Option<i64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChallengeStatusResponse {
    pub is_solved: bool,
    pub solved_times: i64,
    pub pts: i64,
    pub bloods: Vec<cds_db::transfer::Submission>,
}

pub async fn get_challenge_status(
    Extension(ext): Extension<Ext>, Json(body): Json<GetChallengeStatusRequest>,
) -> Result<WebResponse<HashMap<uuid::Uuid, ChallengeStatusResponse>>, WebError> {
    let _ = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;

    let mut submissions =
        cds_db::transfer::submission::get_by_challenge_ids(body.challenge_ids.clone()).await?;

    let mut result: HashMap<uuid::Uuid, ChallengeStatusResponse> = HashMap::new();

    for challenge_id in body.challenge_ids {
        result
            .entry(challenge_id)
            .or_insert_with(|| ChallengeStatusResponse {
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

        if let Some(game_team_id) = body.game_team_id {
            if let Some(game_id) = body.game_id {
                if submission.team_id == Some(game_team_id) && submission.game_id == Some(game_id) {
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
        let game_challenges = cds_db::entity::game_challenge::Entity::find()
            .filter(cds_db::entity::game_challenge::Column::GameId.eq(game_id))
            .all(get_db())
            .await?;

        for game_challenge in game_challenges {
            if let Some(status_response) = result.get_mut(&game_challenge.challenge_id) {
                status_response.pts = game_challenge.pts;
            }
        }
    }

    Ok(WebResponse {
        code: StatusCode::OK,
        data: Some(result),
        ..Default::default()
    })
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateChallengeRequest {
    pub title: String,
    pub description: String,
    pub category: i32,
    pub tags: Option<Vec<String>>,
    pub is_public: Option<bool>,
    pub is_dynamic: Option<bool>,
    pub has_attachment: Option<bool>,
    pub image_name: Option<String>,
    pub env: Option<cds_db::entity::challenge::Env>,
    pub checker: Option<String>,
}

pub async fn create_challenge(
    Extension(ext): Extension<Ext>, Json(body): Json<CreateChallengeRequest>,
) -> Result<WebResponse<Challenge>, WebError> {
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
        checker: Set(body.checker),
        ..Default::default()
    }
    .insert(get_db())
    .await?;
    let challenge = cds_db::transfer::Challenge::from(challenge);

    Ok(WebResponse {
        code: StatusCode::OK,
        data: Some(challenge),
        ..Default::default()
    })
}
