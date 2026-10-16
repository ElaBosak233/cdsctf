//! HTTP routing for public user statistics.

use std::sync::Arc;

use axum::{Json, Router, extract::State};
use cds_db::UserAccountView;
use serde::Serialize;
use serde_json::json;
use utoipa_axum::{router::{OpenApiRouter, UtoipaMethodRouterExt}, routes};

use crate::{
    extract::Path,
    traits::{AppState, WebError},
};

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct UserCategoryStatistics {
    pub category: i32,
    pub solved: u64,
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct UserStatistics {
    pub solved: u64,
    pub solved_by_category: Vec<UserCategoryStatistics>,
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct UserStatisticsResponse {
    pub statistics: UserStatistics,
}

pub fn router(state: Arc<AppState>) -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::from(Router::new().with_state(state.clone()))
        .routes(routes!(get_user_statistics).with_state(state))
}

/// Returns the number of distinct successful solves grouped by category.
#[utoipa::path(
    get,
    path = "/",
    tag = "user",
    params(
        ("user_id" = i64, Path, description = "User id"),
    ),
    responses(
        (status = 200, description = "User solve statistics", body = UserStatisticsResponse),
        (status = 404, description = "Not found", body = crate::traits::ErrorResponse),
        (status = 500, description = "Server error", body = crate::traits::ErrorResponse),
    )
)]
#[tracing::instrument(skip_all, fields(handler = "get_user_statistics"))]
pub async fn get_user_statistics(
    State(s): State<Arc<AppState>>,
    Path(user_id): Path<i64>,
) -> Result<Json<UserStatisticsResponse>, WebError> {
    cds_db::user::find_by_id::<UserAccountView>(&s.db.conn, user_id)
        .await?
        .ok_or(WebError::NotFound(json!("not_found")))?;

    let solved_by_category = cds_db::submission::count_correct_by_user_and_category(
        &s.db.conn,
        user_id,
    )
    .await?
    .into_iter()
    .map(|(category, solved)| UserCategoryStatistics { category, solved })
    .collect::<Vec<_>>();

    let solved = solved_by_category.iter().map(|item| item.solved).sum();

    Ok(Json(UserStatisticsResponse {
        statistics: UserStatistics {
            solved,
            solved_by_category,
        },
    }))
}
