use axum::async_trait;
use sea_orm::{entity::prelude::*, TryIntoModel};
use serde::{Deserialize, Serialize};

use crate::{calculator::traits::CalculatorPayload, database::get_db};

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

    #[sea_orm(default_value = 0)]
    pub pts: i64,
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
impl ActiveModelBehavior for ActiveModel {
    async fn after_delete<C>(self, _db: &C) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        let game_team = self.clone().try_into_model()?;
        crate::queue::publish(
            "calculator",
            CalculatorPayload {
                game_id: Some(game_team.game_id),
                team_id: None,
            },
        )
        .await
        .unwrap();

        return Ok(self);
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExPtsModel {
    pub game_id: i64,
    pub team_id: i64,
    pub is_allowed: bool,
    pub pts: i64,
    pub rank: i64,
    pub team: Option<team::Model>,
}

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
