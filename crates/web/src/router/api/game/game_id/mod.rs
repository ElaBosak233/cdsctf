mod challenge;
mod icon;
mod notice;
mod poster;
mod team;

use axum::{Router, http::StatusCode};
use cds_db::{
    entity::submission::Status,
    get_db,
    sea_orm::{
        ColumnTrait, EntityTrait, Order, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect,
    },
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use cds_db::entity::team::State;
use crate::{
    extract::{Extension, Path, Query},
    model::{game::Game, submission::Submission, team::Team},
    traits::{AuthPrincipal, WebError, WebResponse},
};

pub fn router() -> Router {
    Router::new()
        .route("/", axum::routing::get(get_game))
        .nest("/challenges", challenge::router())
        .nest("/teams", team::router())
        .nest("/notices", notice::router())
        .nest("/icon", icon::router())
        .nest("/poster", poster::router())
        .route("/scoreboard", axum::routing::get(get_game_scoreboard))
}

pub async fn get_game(
    Extension(ext): Extension<AuthPrincipal>,
    Path(game_id): Path<i64>,
) -> Result<WebResponse<Game>, WebError> {
    let _ = ext.operator.ok_or(WebError::Unauthorized(json!("")))?;

    let game = crate::util::loader::prepare_game(game_id).await?;

    Ok(WebResponse {
        code: StatusCode::OK,
        data: Some(game),
        ..Default::default()
    })
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetGameScoreboardRequest {
    pub size: Option<u64>,
    pub page: Option<u64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ScoreRecord {
    pub team: Team,
    pub submissions: Vec<Submission>,
}

pub async fn get_game_scoreboard(
    Path(game_id): Path<i64>,
    Query(params): Query<GetGameScoreboardRequest>,
) -> Result<WebResponse<Vec<ScoreRecord>>, WebError> {
    let game = crate::util::loader::prepare_game(game_id).await?;

    let mut sql = cds_db::entity::team::Entity::find()
        .filter(cds_db::entity::team::Column::GameId.eq(game.id))
        .filter(cds_db::entity::team::Column::State.eq(State::Passed))
        .order_by(cds_db::entity::team::Column::Rank, Order::Asc)
        .order_by(cds_db::entity::team::Column::Pts, Order::Desc);

    let total = sql.clone().count(get_db()).await?;

    if let (Some(page), Some(size)) = (params.page, params.size) {
        let offset = (page - 1) * size;
        sql = sql.offset(offset).limit(size);
    }

    let teams = sql.into_model::<Team>().all(get_db()).await?;

    let team_ids = teams.iter().map(|t| t.id).collect::<Vec<i64>>();

    let submissions = cds_db::entity::submission::Entity::base_find()
        .filter(cds_db::entity::submission::Column::Status.eq(Status::Correct))
        .filter(cds_db::entity::submission::Column::GameId.eq(game_id))
        .filter(cds_db::entity::submission::Column::TeamId.is_in(team_ids))
        .into_model::<Submission>()
        .all(get_db())
        .await?;

    let mut result: Vec<ScoreRecord> = Vec::new();

    for team in teams {
        let submissions = submissions
            .iter()
            .filter(|s| s.team_id.is_some_and(|t| t == team.id))
            .cloned()
            .collect::<Vec<Submission>>();

        result.push(ScoreRecord { team, submissions });
    }

    Ok(WebResponse {
        code: StatusCode::OK,
        data: Some(result),
        total: Some(total),
        ..Default::default()
    })
}
