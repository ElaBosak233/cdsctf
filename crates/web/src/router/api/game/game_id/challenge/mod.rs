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

    pub page: Option<u64>,
    pub size: Option<u64>,
}

/// Get challenges by given params.
///
/// # Prerequisite
/// - If the operator is admin, there is no prerequisite.
/// - Operating time is between related game's `started_at` and `ended_at`.
pub async fn get_game_challenge(
    Extension(ext): Extension<Ext>,
    Path(game_id): Path<i64>,
    Query(params): Query<GetGameChallengeRequest>,
) -> Result<WebResponse<Vec<cds_db::transfer::GameChallenge>>, WebError> {
    let operator = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;

    let game = cds_db::entity::game::Entity::find_by_id(game_id)
        .one(get_db())
        .await?
        .map(cds_db::transfer::Game::from)
        .ok_or(WebError::BadRequest(json!("game_not_found")))?;

    let now = chrono::Utc::now().timestamp();
    let in_game = cds_db::util::is_user_in_game(&operator, &game, Some(State::Passed)).await?;

    if !in_game || !(game.started_at..=game.ended_at).contains(&now) {
        return Err(WebError::Forbidden(json!("")));
    }

    // Using inner join to access fields in related tables.
    let mut sql = cds_db::entity::game_challenge::Entity::find()
        .inner_join(cds_db::entity::challenge::Entity)
        .inner_join(cds_db::entity::game::Entity);

    sql = sql.filter(cds_db::entity::game_challenge::Column::GameId.eq(game_id));

    if let Some(challenge_id) = params.challenge_id {
        sql = sql.filter(cds_db::entity::game_challenge::Column::ChallengeId.eq(challenge_id));
    }

    sql = sql.filter(cds_db::entity::game_challenge::Column::IsEnabled.eq(true));

    if let Some(category) = params.category {
        sql = sql.filter(cds_db::entity::challenge::Column::Category.eq(category));
    }

    let total = sql.clone().count(get_db()).await?;

    if let (Some(page), Some(size)) = (params.page, params.size) {
        let offset = (page - 1) * size;
        sql = sql.offset(offset).limit(size);
    }

    let mut game_challenges =
        cds_db::transfer::game_challenge::preload(sql.all(get_db()).await?).await?;

    for game_challenge in game_challenges.iter_mut() {
        *game_challenge = game_challenge.desensitize();
    }

    Ok(WebResponse {
        data: Some(game_challenges),
        total: Some(total),
        ..Default::default()
    })
}
