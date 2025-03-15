use axum::Router;
use cds_db::transfer::Team;
use nanoid::nanoid;
use sea_orm::{ActiveModelTrait, EntityTrait};

use crate::{
    extract::{Extension, Path},
    traits::{WebError, WebResponse},
};

pub fn router() -> Router {
    Router::new()
        .route("/", axum::routing::post(create_token))
        .route("/", axum::routing::get(get_token))
        .route("/", axum::routing::delete(delete_token))
}

/// Create an invitation token.
pub async fn create_token(
    Path((game_id, team_id)): Path<(i64, i64)>,
) -> Result<WebResponse<String>, WebError> {
    let team = crate::util::loader::prepare_team(game_id, team_id).await?;

    let token = nanoid!(16);
    cds_cache::set_ex(format!("team:{}:invite", team.id), token.clone(), 60 * 60).await?;

    Ok(WebResponse {
        data: Some(token),
        ..Default::default()
    })
}

/// Get invitation token.
pub async fn get_token(
    Path((game_id, team_id)): Path<(i64, i64)>,
) -> Result<WebResponse<String>, WebError> {
    let team = crate::util::loader::prepare_team(game_id, team_id).await?;
    let token = cds_cache::get::<String>(format!("team:{}:invite", team.id)).await?;

    Ok(WebResponse {
        data: token,
        ..Default::default()
    })
}

/// Delete invitation token.
pub async fn delete_token(
    Path((game_id, team_id)): Path<(i64, i64)>,
) -> Result<WebResponse<String>, WebError> {
    let team = crate::util::loader::prepare_team(game_id, team_id).await?;
    let token = cds_cache::get_del::<String>(format!("team:{}:invite", team.id)).await?;

    Ok(WebResponse {
        data: token,
        ..Default::default()
    })
}
