pub mod request;

use axum::async_trait;
use sea_orm::{entity::prelude::*, TryIntoModel};
use serde::{Deserialize, Serialize};

use crate::database::get_db;

use super::{team, user};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "user_teams")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub user_id: i64,
    #[sea_orm(primary_key)]
    pub team_id: i64,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    User,
    Team,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::User => Entity::belongs_to(user::Entity)
                .from(Column::UserId)
                .to(user::Column::Id)
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

pub async fn find(
    user_id: Option<i64>, team_id: Option<i64>,
) -> Result<(Vec<crate::model::user_team::Model>, u64), DbErr> {
    let mut query = crate::model::user_team::Entity::find();

    if let Some(user_id) = user_id {
        query = query.filter(crate::model::user_team::Column::UserId.eq(user_id));
    }

    if let Some(team_id) = team_id {
        query = query.filter(crate::model::user_team::Column::TeamId.eq(team_id));
    }

    let total = query.clone().count(&get_db().await).await?;

    let user_teams = query.all(&get_db().await).await?;

    Ok((user_teams, total))
}

pub async fn create(
    user_team: crate::model::user_team::ActiveModel,
) -> Result<crate::model::user_team::Model, DbErr> {
    user_team.insert(&get_db().await).await?.try_into_model()
}

pub async fn update(
    user_team: crate::model::user_team::ActiveModel,
) -> Result<crate::model::user_team::Model, DbErr> {
    user_team.update(&get_db().await).await?.try_into_model()
}

pub async fn delete(user_id: i64, team_id: i64) -> Result<(), DbErr> {
    let _result: sea_orm::DeleteResult = crate::model::user_team::Entity::delete_many()
        .filter(crate::model::user_team::Column::UserId.eq(user_id))
        .filter(crate::model::user_team::Column::TeamId.eq(team_id))
        .exec(&get_db().await)
        .await?;

    return Ok(());
}
