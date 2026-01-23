use std::sync::Arc;

use axum::{Router, extract::State};
use cds_db::{
    DB, GameChallengeMini, game_challenge::FindGameChallengeOptions, team::State as TState,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    extract::{Extension, Path, Query},
    traits::{AppState, AuthPrincipal, WebError, WebResponse},
};

pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/", axum::routing::get(get_game_challenge))
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetGameChallengeRequest {
    pub game_id: Option<i64>,
    pub challenge_id: Option<i64>,
    pub category: Option<i32>,
}

/// Get challenges by given params.
/// - Operating time is between related game's `started_at` and `ended_at`.
pub async fn get_game_challenge(
    State(ref s): State<Arc<AppState>>,

    Extension(ext): Extension<AuthPrincipal>,
    Path(game_id): Path<i64>,
    Query(params): Query<GetGameChallengeRequest>,
) -> Result<WebResponse<Vec<GameChallengeMini>>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;

    let game = crate::util::loader::prepare_game(&s.db.conn, game_id).await?;

    let now = time::OffsetDateTime::now_utc().unix_timestamp();
    let in_game =
        cds_db::util::is_user_in_game(&s.db.conn, operator.id, game.id, Some(TState::Passed))
            .await?;

    if !in_game || !(game.started_at..=game.ended_at).contains(&now) {
        return Err(WebError::Forbidden(json!("")));
    }

    let (game_challenges, total) = cds_db::game_challenge::find(
        &s.db.conn,
        FindGameChallengeOptions {
            game_id: Some(game.id),
            challenge_id: params.challenge_id,
            is_enabled: Some(true),
            category: params.category,
        },
    )
    .await?;

    Ok(WebResponse {
        data: Some(game_challenges),
        total: Some(total),
        ..Default::default()
    })
}
