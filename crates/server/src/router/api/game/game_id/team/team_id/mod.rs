mod avatar;

use std::sync::Arc;

use axum::{Router, extract::State, http::StatusCode};
use cds_db::{TeamUser, UserMini, sea_orm::ActiveValue::Set, team::State as TState};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    extract::{Extension, Json, Path},
    traits::{AppState, AuthPrincipal, WebError, WebResponse},
};

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .nest("/avatar", avatar::router())
        .route("/members", axum::routing::get(get_team_members))
        .route("/join", axum::routing::post(join_team))
}

pub async fn get_team_members(
    State(ref s): State<Arc<AppState>>,

    Path((_game_id, team_id)): Path<(i64, i64)>,
) -> Result<WebResponse<Vec<UserMini>>, WebError> {
    let users = cds_db::user::find_by_team_id(&s.db.conn, team_id).await?;

    Ok(WebResponse {
        code: StatusCode::OK,
        data: Some(users),
        ..Default::default()
    })
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JoinTeamRequest {
    pub team_id: i64,
    pub token: String,
}

pub async fn join_team(
    State(ref s): State<Arc<AppState>>,

    Extension(ext): Extension<AuthPrincipal>,
    Path((game_id, team_id)): Path<(i64, i64)>,
    Json(body): Json<JoinTeamRequest>,
) -> Result<WebResponse<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    let game = crate::util::loader::prepare_game(&s.db.conn, game_id).await?;
    let team = crate::util::loader::prepare_team(&s.db.conn, game_id, team_id).await?;

    if team.state != TState::Preparing {
        return Err(WebError::BadRequest(json!("team_not_preparing")));
    }

    if cds_db::util::is_user_in_game(&s.db.conn, operator.id, game.id, None).await? {
        return Err(WebError::BadRequest(json!("user_already_in_game")));
    }

    if body.team_id != team.id {
        return Err(WebError::BadRequest(json!("invalid_team")));
    }

    let criteria = s
        .cache
        .get::<String>(format!("team:{}:invite", team.id))
        .await?
        .ok_or(WebError::BadRequest(json!("no_invite_token")))?;

    if criteria != body.token {
        return Err(WebError::BadRequest(json!("invalid_invite_token")));
    }

    let _ = cds_db::team_user::create::<TeamUser>(
        &s.db.conn,
        cds_db::team_user::ActiveModel {
            team_id: Set(team.id),
            user_id: Set(operator.id),
        },
    )
    .await?;

    Ok(WebResponse {
        code: StatusCode::OK,
        ..Default::default()
    })
}
