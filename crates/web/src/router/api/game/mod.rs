mod game_id;

use std::str::FromStr;

use axum::{Router, http::StatusCode};
use cds_db::{
    get_db,
    sea_orm::{
        ColumnTrait, EntityTrait, Order, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect,
    },
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    extract::{Extension, Query},
    model::game::GameMini,
    traits::{Ext, WebError, WebResponse},
};

pub fn router() -> Router {
    Router::new()
        .route("/", axum::routing::get(get_game))
        .nest("/{game_id}", game_id::router())
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetGameRequest {
    pub id: Option<i64>,
    pub title: Option<String>,
    pub page: Option<u64>,
    pub size: Option<u64>,
    pub sorts: Option<String>,
}

/// Get games with given params.
pub async fn get_game(
    Extension(ext): Extension<Ext>,
    Query(params): Query<GetGameRequest>,
) -> Result<WebResponse<Vec<GameMini>>, WebError> {
    let _ = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;

    let mut sql = cds_db::entity::game::Entity::find();

    if let Some(id) = params.id {
        sql = sql.filter(cds_db::entity::game::Column::Id.eq(id));
    }

    if let Some(title) = params.title {
        sql = sql.filter(cds_db::entity::game::Column::Title.contains(title));
    }

    sql = sql.filter(cds_db::entity::game::Column::IsEnabled.eq(true));

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

    let games = sql.into_model::<GameMini>().all(get_db()).await?;

    Ok(WebResponse {
        code: StatusCode::OK,
        data: Some(games),
        total: Some(total),
        ..Default::default()
    })
}
