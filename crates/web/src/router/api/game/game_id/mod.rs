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
    transfer::{Submission, Team},
};
use serde::{Deserialize, Serialize};
use cds_db::traits::EagerLoading;
use crate::{
    extract::{Path, Query},
    traits::{WebError, WebResponse},
};

pub fn router() -> Router {
    Router::new()
        .nest("/challenges", challenge::router())
        .nest("/teams", team::router())
        .nest("/notices", notice::router())
        .nest("/icon", icon::router())
        .nest("/poster", poster::router())
        .route("/scoreboard", axum::routing::get(get_game_scoreboard))
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
        .order_by(cds_db::entity::team::Column::Pts, Order::Desc);

    let total = sql.clone().count(get_db()).await?;

    if let (Some(page), Some(size)) = (params.page, params.size) {
        let offset = (page - 1) * size;
        sql = sql.offset(offset).limit(size);
    }

    let teams = sql.all(get_db()).await?.eager_load(get_db()).await?;

    let team_ids = teams.iter().map(|t| t.id).collect::<Vec<i64>>();

    let submissions = cds_db::transfer::submission::get_by_game_id_and_team_ids(
        game.id,
        team_ids,
        Some(Status::Correct),
    )
    .await?;

    let mut result: Vec<ScoreRecord> = Vec::new();

    for team in teams {
        let mut submissions = submissions
            .iter()
            .filter(|s| s.team_id.unwrap() == team.id)
            .cloned()
            .collect::<Vec<Submission>>();
        for submission in submissions.iter_mut() {
            *submission = submission.desensitize();
            submission.team = None;
            submission.game = None;
        }

        result.push(ScoreRecord { team, submissions });
    }

    Ok(WebResponse {
        code: StatusCode::OK,
        data: Some(result),
        total: Some(total),
        ..Default::default()
    })
}
