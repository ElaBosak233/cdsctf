use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};

use crate::entity::game::Timeslot;

#[derive(
    Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromQueryResult, utoipa::ToSchema,
)]
pub struct GameDetail {
    pub id: i64,
    pub title: String,
    pub sketch: Option<String>,
    pub description: Option<String>,
    pub enabled: bool,
    pub public: bool,
    pub writeup_required: bool,
    pub member_limit_min: i64,
    pub member_limit_max: i64,
    pub timeslots: Vec<Timeslot>,
    pub started_at: i64,
    pub frozen_at: i64,
    pub ended_at: i64,
    pub icon_hash: Option<String>,
    pub poster_hash: Option<String>,
    pub created_at: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, utoipa::ToSchema)]
pub struct GameView {
    pub id: i64,
    pub title: String,
    pub sketch: Option<String>,
    pub description: Option<String>,
    pub writeup_required: bool,
    pub started_at: i64,
    pub frozen_at: i64,
    pub ended_at: i64,
    pub icon_hash: Option<String>,
    pub poster_hash: Option<String>,
}

impl From<&GameDetail> for GameView {
    fn from(game: &GameDetail) -> Self {
        Self {
            id: game.id,
            title: game.title.clone(),
            sketch: game.sketch.clone(),
            description: game.description.clone(),
            writeup_required: game.writeup_required,
            started_at: game.started_at,
            frozen_at: game.frozen_at,
            ended_at: game.ended_at,
            icon_hash: game.icon_hash.clone(),
            poster_hash: game.poster_hash.clone(),
        }
    }
}

#[derive(
    Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromQueryResult, utoipa::ToSchema,
)]
pub struct GameSummary {
    pub id: i64,
    pub title: String,
    pub sketch: Option<String>,
    pub started_at: i64,
    pub frozen_at: i64,
    pub ended_at: i64,
    pub icon_hash: Option<String>,
    pub poster_hash: Option<String>,
}
