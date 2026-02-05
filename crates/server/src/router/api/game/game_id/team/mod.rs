mod team_id;
mod us;

use std::sync::Arc;

use axum::{Router, extract::State, http::StatusCode};
use cds_db::{
    TeamUser,
    sea_orm::ActiveValue::Set,
    team::{State as TState, Team},
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    extract::{Extension, Json, Path},
    traits::{AppState, AuthPrincipal, WebError, WebResponse},
};

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/register", axum::routing::post(team_register))
        .nest("/us", us::router())
        .nest("/{team_id}", team_id::router())
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TeamRegisterRequest {
    pub name: String,
    pub email: Option<String>,
    pub slogan: Option<String>,
    pub description: Option<String>,
}

/// Add a team to a game with given path and data.
///
/// # Prerequisite
/// - No user in the team is already in the game.
pub async fn team_register(
    State(s): State<Arc<AppState>>,

    Extension(ext): Extension<AuthPrincipal>,
    Path(game_id): Path<i64>,
    Json(body): Json<TeamRegisterRequest>,
) -> Result<WebResponse<Team>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;

    let game = crate::util::loader::prepare_game(&s.db.conn, game_id).await?;

    if cds_db::util::is_user_in_game(&s.db.conn, operator.id, game.id, None).await? {
        return Err(WebError::BadRequest(json!("user_already_in_game")));
    }

    let team = cds_db::team::create::<Team>(
        &s.db.conn,
        cds_db::team::ActiveModel {
            name: Set(body.name),
            email: Set(body.email),
            slogan: Set(body.slogan),
            game_id: Set(game.id),
            state: Set(TState::Preparing),
            ..Default::default()
        },
    )
    .await?;

    let _ = cds_db::team_user::create::<TeamUser>(
        &s.db.conn,
        cds_db::team_user::ActiveModel {
            team_id: Set(team.id),
            user_id: Set(operator.id),
        },
    )
    .await?;

    let team = cds_db::team::find_by_id(&s.db.conn, team.id, team.game_id).await?;

    Ok(WebResponse {
        code: StatusCode::OK,
        data: team,
        ..Default::default()
    })
}
