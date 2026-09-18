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
    pub paused: bool,
    pub blacked_out: bool,
    pub writeup_required: bool,
    pub member_limit_min: i64,
    pub member_limit_max: i64,
    pub timeslots: Vec<Timeslot>,
    #[serde(with = "crate::time_format")]
    pub started_at: time::OffsetDateTime,
    #[serde(with = "crate::time_format")]
    pub frozen_at: time::OffsetDateTime,
    #[serde(with = "crate::time_format")]
    pub ended_at: time::OffsetDateTime,
    pub icon_hash: Option<String>,
    pub poster_hash: Option<String>,
    #[serde(with = "crate::time_format")]
    pub created_at: time::OffsetDateTime,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, utoipa::ToSchema)]
pub struct GameView {
    pub id: i64,
    pub title: String,
    pub sketch: Option<String>,
    pub description: Option<String>,
    pub writeup_required: bool,
    pub paused: bool,
    pub blacked_out: bool,
    #[serde(with = "crate::time_format")]
    pub started_at: time::OffsetDateTime,
    #[serde(with = "crate::time_format")]
    pub frozen_at: time::OffsetDateTime,
    #[serde(with = "crate::time_format")]
    pub ended_at: time::OffsetDateTime,
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
            paused: game.paused,
            blacked_out: game.blacked_out,
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
    #[serde(with = "crate::time_format")]
    pub started_at: time::OffsetDateTime,
    #[serde(with = "crate::time_format")]
    pub frozen_at: time::OffsetDateTime,
    #[serde(with = "crate::time_format")]
    pub ended_at: time::OffsetDateTime,
    pub icon_hash: Option<String>,
    pub poster_hash: Option<String>,
}
