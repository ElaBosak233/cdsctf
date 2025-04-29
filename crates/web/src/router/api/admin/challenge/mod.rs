mod challenge_id;

use std::str::FromStr;

use axum::{Router, http::StatusCode};
use cds_db::{
    entity::user::Group,
    get_db,
    sea_orm::{
        ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityName, EntityTrait, Iden, IdenStatic,
        Order, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, sea_query::Expr,
    },
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    extract::{Extension, Json, Query},
    model::challenge::Challenge,
    traits::{Ext, WebError, WebResponse},
};

pub fn router() -> Router {
    Router::new()
        .route("/", axum::routing::get(get_challenges))
        .route("/", axum::routing::post(create_challenge))
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
}

pub async fn get_challenges(
    Query(params): Query<GetChallengeRequest>,
) -> Result<WebResponse<Vec<Challenge>>, WebError> {
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

    let offset = (page - 1) * size;
    sql = sql.offset(offset).limit(size);

    let challenges = sql.into_model::<Challenge>().all(get_db()).await?;

    Ok(WebResponse {
        code: StatusCode::OK,
        data: Some(challenges),
        total: Some(total),
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
    Json(body): Json<CreateChallengeRequest>,
) -> Result<WebResponse<Challenge>, WebError> {
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

    let challenge = cds_db::entity::challenge::Entity::find_by_id(challenge.id)
        .into_model::<Challenge>()
        .one(get_db())
        .await?;

    Ok(WebResponse {
        code: StatusCode::OK,
        data: challenge,
        ..Default::default()
    })
}
