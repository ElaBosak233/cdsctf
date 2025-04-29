mod challenge_id;

use axum::Router;
use cds_db::{
    entity::team::State,
    get_db,
    sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QuerySelect},
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

use crate::{
    extract::{Extension, Path, Query},
    model::game_challenge::GameChallengeMini,
    traits::{Ext, WebError, WebResponse},
};

pub fn router() -> Router {
    Router::new().route("/", axum::routing::get(get_game_challenge))
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetGameChallengeRequest {
    pub game_id: Option<i64>,
    pub challenge_id: Option<Uuid>,
    pub category: Option<i32>,
}

/// Get challenges by given params.
/// - Operating time is between related game's `started_at` and `ended_at`.
pub async fn get_game_challenge(
    Extension(ext): Extension<Ext>,
    Path(game_id): Path<i64>,
    Query(params): Query<GetGameChallengeRequest>,
) -> Result<WebResponse<Vec<GameChallengeMini>>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;

    let game = crate::util::loader::prepare_game(game_id).await?;

    let now = chrono::Utc::now().timestamp();
    let in_game = cds_db::util::is_user_in_game(operator.id, game.id, Some(State::Passed)).await?;

    if !in_game || !(game.started_at..=game.ended_at).contains(&now) {
        return Err(WebError::Forbidden(json!("")));
    }

    let mut sql = cds_db::entity::game_challenge::Entity::base_find();

    sql = sql.filter(cds_db::entity::game_challenge::Column::GameId.eq(game_id));

    if let Some(challenge_id) = params.challenge_id {
        sql = sql.filter(cds_db::entity::game_challenge::Column::ChallengeId.eq(challenge_id));
    }

    sql = sql.filter(cds_db::entity::game_challenge::Column::IsEnabled.eq(true));

    if let Some(category) = params.category {
        sql = sql.filter(cds_db::entity::challenge::Column::Category.eq(category));
    }

    let total = sql.clone().count(get_db()).await?;

    let game_challenges = sql.into_model::<GameChallengeMini>().all(get_db()).await?;

    Ok(WebResponse {
        data: Some(game_challenges),
        total: Some(total),
        ..Default::default()
    })
}
