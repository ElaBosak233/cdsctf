mod team_id;

use axum::{Router, http::StatusCode};
use cds_db::{
    sea_orm::ActiveValue::Set,
    team::{FindTeamOptions, State, Team},
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    extract::{Extension, Json, Path, Query},
    traits::{AuthPrincipal, WebError, WebResponse},
};

pub fn router() -> Router {
    Router::new()
        .route("/", axum::routing::get(get_team))
        .route("/", axum::routing::post(create_team))
        .nest("/{team_id}", team_id::router())
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetTeamRequest {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub state: Option<State>,
    pub user_id: Option<i64>,
    pub page: Option<u64>,
    pub size: Option<u64>,
    pub sorts: Option<String>,
}

/// Get game teams with given data.
pub async fn get_team(
    Extension(ext): Extension<AuthPrincipal>,
    Path(game_id): Path<i64>,
    Query(params): Query<GetTeamRequest>,
) -> Result<WebResponse<Vec<Team>>, WebError> {
    let _ = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;

    let (teams, total) = cds_db::team::find::<Team>(FindTeamOptions {
        id: params.id,
        name: params.name,
        state: params.state,
        game_id: Some(game_id),
        user_id: params.user_id,
        page: params.page,
        size: params.size,
        sorts: params.sorts,
    })
    .await?;

    Ok(WebResponse {
        code: StatusCode::OK,
        data: Some(teams),
        total: Some(total),
        ..Default::default()
    })
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateTeamRequest {
    pub name: String,
    pub email: Option<String>,
    pub slogan: Option<String>,
}

/// Add a team to a game with given path and data.
///
/// # Prerequisite
/// - Operator is admin.
pub async fn create_team(
    Path(game_id): Path<i64>,
    Json(body): Json<CreateTeamRequest>,
) -> Result<WebResponse<Team>, WebError> {
    let game = crate::util::loader::prepare_game(game_id).await?;

    let team = cds_db::team::create(cds_db::team::ActiveModel {
        name: Set(body.name),
        email: Set(body.email),
        slogan: Set(body.slogan),
        game_id: Set(game.id),
        state: Set(State::Preparing),
        ..Default::default()
    })
    .await?;

    Ok(WebResponse {
        code: StatusCode::OK,
        data: Some(team),
        ..Default::default()
    })
}
