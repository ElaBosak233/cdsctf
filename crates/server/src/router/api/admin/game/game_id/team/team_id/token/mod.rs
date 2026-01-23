use std::sync::Arc;

use axum::{Router, extract::State};
use nanoid::nanoid;

use crate::{
    extract::Path,
    traits::{AppState, WebError, WebResponse},
};

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", axum::routing::post(create_token))
        .route("/", axum::routing::get(get_token))
        .route("/", axum::routing::delete(delete_token))
}

/// Create an invitation token.
pub async fn create_token(
    State(s): State<Arc<AppState>>,

    Path((game_id, team_id)): Path<(i64, i64)>,
) -> Result<WebResponse<String>, WebError> {
    let team = crate::util::loader::prepare_team(&s.db.conn, game_id, team_id).await?;

    let token = nanoid!(16);
    s.cache
        .set_ex(format!("team:{}:invite", team.id), token.clone(), 60 * 60)
        .await?;

    Ok(WebResponse {
        data: Some(token),
        ..Default::default()
    })
}

/// Get invitation token.
pub async fn get_token(
    State(s): State<Arc<AppState>>,

    Path((game_id, team_id)): Path<(i64, i64)>,
) -> Result<WebResponse<String>, WebError> {
    let team = crate::util::loader::prepare_team(&s.db.conn, game_id, team_id).await?;
    let token = s
        .cache
        .get::<String>(format!("team:{}:invite", team.id))
        .await?;

    Ok(WebResponse {
        data: token,
        ..Default::default()
    })
}

/// Delete invitation token.
pub async fn delete_token(
    State(s): State<Arc<AppState>>,

    Path((game_id, team_id)): Path<(i64, i64)>,
) -> Result<WebResponse<String>, WebError> {
    let team = crate::util::loader::prepare_team(&s.db.conn, game_id, team_id).await?;
    let token = s
        .cache
        .get_del::<String>(format!("team:{}:invite", team.id))
        .await?;

    Ok(WebResponse {
        data: token,
        ..Default::default()
    })
}
