use std::sync::Arc;

use axum::{Router, extract::State, http::StatusCode};
use cds_db::{TeamUser, team::State as TState, team_user::FindTeamUserOptions};
use serde_json::json;

use crate::{
    extract::{Extension, Path},
    traits::{AppState, AuthPrincipal, WebError, WebResponse},
};

pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/leave", axum::routing::delete(leave_team))
}

pub async fn leave_team(
    State(ref s): State<Arc<AppState>>,

    Extension(ext): Extension<AuthPrincipal>,
    Path(game_id): Path<i64>,
) -> Result<WebResponse<()>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;
    let team = crate::util::loader::prepare_self_team(&s.db.conn, game_id, operator.id).await?;

    if team.state != TState::Preparing {
        return Err(WebError::BadRequest(json!("team_not_preparing")));
    }

    let (_, count) = cds_db::team_user::find::<TeamUser>(
        &s.db.conn,
        FindTeamUserOptions {
            team_id: Some(team.id),
            user_id: Some(operator.id),
        },
    )
    .await?;

    if count <= 1 {
        return Err(WebError::BadRequest(json!("team_has_no_other_member")));
    }

    cds_db::team_user::delete(&s.db.conn, team.id, operator.id).await?;

    Ok(WebResponse {
        code: StatusCode::OK,
        ..Default::default()
    })
}
