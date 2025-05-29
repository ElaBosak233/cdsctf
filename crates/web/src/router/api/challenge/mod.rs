mod challenge_id;

use std::{collections::HashMap, str::FromStr};

use axum::Router;
use cds_db::{
    entity::submission::Status,
    get_db,
    sea_orm::{
        ColumnTrait, EntityName, EntityTrait, Iden, Order, PaginatorTrait, QueryFilter, QueryOrder,
        QuerySelect, sea_query::Expr,
    },
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    extract::{Extension, Json, Query},
    model::{challenge::ChallengeMini, submission::Submission},
    traits::{AuthPrincipal, WebError, WebResponse},
};

pub fn router() -> Router {
    Router::new()
        .route("/playground", axum::routing::get(get_challenge))
        .route("/status", axum::routing::post(get_challenge_status))
        .nest("/{challenge_id}", challenge_id::router())
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetChallengeRequest {
    pub id: Option<uuid::Uuid>,
    pub title: Option<String>,
    pub category: Option<i32>,
    pub tag: Option<String>,
    pub page: Option<u64>,
    pub size: Option<u64>,
    pub sorts: Option<String>,
}

pub async fn get_challenge(
    Extension(ext): Extension<AuthPrincipal>,
    Query(params): Query<GetChallengeRequest>,
) -> Result<WebResponse<Vec<ChallengeMini>>, WebError> {
    let _ = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;

    let page = params.page.unwrap_or(1);
    let size = params.size.unwrap_or(10).min(100);

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

    if let Some(tag) = params.tag {
        sql = sql.filter(Expr::cust_with_expr(
            format!(
                "\"{}\".\"{}\" @> $1::text[]",
                cds_db::entity::challenge::Entity.table_name(),
                cds_db::entity::challenge::Column::Tags.to_string()
            )
            .as_str(),
            vec![tag],
        ))
    }

    sql = sql.filter(cds_db::entity::challenge::Column::IsPublic.eq(true));
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

    let offset = (page - 1) * size;
    sql = sql.offset(offset).limit(size);

    let challenges = sql.into_model::<ChallengeMini>().all(get_db()).await?;

    Ok(WebResponse {
        data: Some(challenges),
        total: Some(total),
        ..Default::default()
    })
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetChallengeStatusRequest {
    pub challenge_ids: Vec<uuid::Uuid>,
    pub user_id: Option<i64>,
    pub team_id: Option<i64>,
    pub game_id: Option<i64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChallengeStatusResponse {
    pub is_solved: bool,
    pub solved_times: i64,
    pub pts: i64,
    pub bloods: Vec<Submission>,
}

pub async fn get_challenge_status(
    Extension(ext): Extension<AuthPrincipal>,
    Json(body): Json<GetChallengeStatusRequest>,
) -> Result<WebResponse<HashMap<uuid::Uuid, ChallengeStatusResponse>>, WebError> {
    let _ = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;

    if body.user_id.is_some() && (body.team_id.is_some() || body.game_id.is_some()) {
        return Err(WebError::BadRequest(json!("either_user_or_team")));
    }

    let mut submissions = cds_db::entity::submission::Entity::base_find()
        .filter(
            cds_db::entity::submission::Column::ChallengeId.is_in(body.challenge_ids.to_owned()),
        )
        .order_by_asc(cds_db::entity::submission::Column::CreatedAt)
        .into_model::<Submission>()
        .all(get_db())
        .await?;

    let mut result: HashMap<uuid::Uuid, ChallengeStatusResponse> = HashMap::new();

    for challenge_id in body.challenge_ids.iter() {
        result.insert(
            *challenge_id,
            ChallengeStatusResponse {
                is_solved: false,
                solved_times: 0,
                pts: 0,
                bloods: Vec::new(),
            },
        );
    }

    for submission in submissions.iter_mut() {
        *submission = submission.desensitize();

        let valid = if let Some(user_id) = body.user_id {
            submission.user_id == user_id
                && submission.team_id.is_none()
                && submission.game_id.is_none()
        } else if let (Some(team_id), Some(game_id)) = (body.team_id, body.game_id) {
            submission.team_id == Some(team_id) && submission.game_id == Some(game_id)
        } else {
            false
        };

        if !valid || submission.status != Status::Correct {
            continue;
        }

        if let Some(status_response) = result.get_mut(&submission.challenge_id) {
            status_response.is_solved = true;
            status_response.solved_times += 1;

            if status_response.bloods.len() < 3 {
                status_response.bloods.push(submission.clone());
            }
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
        data: Some(result),
        ..Default::default()
    })
}
