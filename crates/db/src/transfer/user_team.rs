use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{entity, get_db};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserTeam {
    pub user_id: i64,
    pub team_id: i64,
}

impl From<entity::user_team::Model> for UserTeam {
    fn from(entity: entity::user_team::Model) -> Self {
        Self {
            user_id: entity.user_id,
            team_id: entity.team_id,
        }
    }
}

pub async fn find(
    user_id: Option<i64>, team_id: Option<i64>,
) -> Result<(Vec<UserTeam>, u64), DbErr> {
    let mut sql = entity::user_team::Entity::find();

    if let Some(user_id) = user_id {
        sql = sql.filter(entity::user_team::Column::UserId.eq(user_id));
    }

    if let Some(team_id) = team_id {
        sql = sql.filter(entity::user_team::Column::TeamId.eq(team_id));
    }

    let total = sql.clone().count(get_db()).await?;

    let user_teams = sql.all(get_db()).await?;
    let user_teams = user_teams
        .into_iter()
        .map(UserTeam::from)
        .collect::<Vec<UserTeam>>();

    Ok((user_teams, total))
}
