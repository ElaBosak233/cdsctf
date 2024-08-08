use axum::async_trait;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use crate::database::get_db;

use super::{game, team};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "game_teams")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub game_id: i64,
    #[sea_orm(primary_key)]
    pub team_id: i64,
    #[sea_orm(default_value = false)]
    pub is_allowed: bool,

    /// pts of the team in the game. (only controlled by daemons)
    #[sea_orm(default_value = 0)]
    pub pts: i64,
    /// rank of the team in the game. (only controlled by daemons)
    #[sea_orm(default_value = 0)]
    pub rank: i64,

    #[sea_orm(ignore)]
    pub game: Option<game::Model>,
    #[sea_orm(ignore)]
    pub team: Option<team::Model>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Game,
    Team,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Game => Entity::belongs_to(game::Entity)
                .from(Column::GameId)
                .to(game::Column::Id)
                .on_delete(ForeignKeyAction::Cascade)
                .into(),
            Self::Team => Entity::belongs_to(team::Entity)
                .from(Column::TeamId)
                .to(team::Column::Id)
                .on_delete(ForeignKeyAction::Cascade)
                .into(),
        }
    }
}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {}

async fn preload(
    mut game_teams: Vec<crate::model::game_team::Model>,
) -> Result<Vec<crate::model::game_team::Model>, DbErr> {
    let team_ids: Vec<i64> = game_teams
        .iter()
        .map(|game_team| game_team.team_id)
        .collect();

    let teams = crate::model::team::find_by_ids(team_ids).await?;

    for game_team in game_teams.iter_mut() {
        game_team.team = teams
            .iter()
            .find(|team| team.id == game_team.team_id)
            .cloned();
    }

    return Ok(game_teams);
}

pub async fn find(
    game_id: Option<i64>, team_id: Option<i64>,
) -> Result<(Vec<crate::model::game_team::Model>, u64), DbErr> {
    let mut query = crate::model::game_team::Entity::find();

    if let Some(game_id) = game_id {
        query = query.filter(crate::model::game_team::Column::GameId.eq(game_id));
    }

    if let Some(team_id) = team_id {
        query = query.filter(crate::model::game_team::Column::TeamId.eq(team_id));
    }

    let total = query.clone().count(&get_db()).await?;

    let mut game_teams = query.all(&get_db()).await?;

    game_teams = preload(game_teams).await?;

    Ok((game_teams, total))
}
