use serde::{Deserialize, Serialize};

use crate::{entity, entity::game::Timeslot};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Game {
    pub id: i64,
    pub title: String,
    pub sketch: Option<String>,
    pub description: Option<String>,
    pub is_enabled: bool,
    pub is_public: bool,
    pub is_need_write_up: bool,
    pub member_limit_min: i64,
    pub member_limit_max: i64,
    pub timeslots: Vec<Timeslot>,
    pub started_at: i64,
    pub frozen_at: i64,
    pub ended_at: i64,
    pub created_at: i64,
    pub updated_at: i64,
}

impl From<entity::game::Model> for Game {
    fn from(model: entity::game::Model) -> Self {
        Self {
            id: model.id,
            title: model.title,
            sketch: model.sketch,
            description: model.description,
            is_enabled: model.is_enabled,
            is_public: model.is_public,
            is_need_write_up: model.is_need_write_up,
            member_limit_min: model.member_limit_min,
            member_limit_max: model.member_limit_max,
            timeslots: model.timeslots,
            started_at: model.started_at,
            frozen_at: model.frozen_at,
            ended_at: model.ended_at,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}
