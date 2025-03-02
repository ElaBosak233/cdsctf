use sea_orm::{
    ColumnTrait, DbErr, EntityTrait, JoinType, PaginatorTrait, QueryFilter, QuerySelect,
    RelationTrait,
};

use crate::{entity::team::State, get_db};

pub async fn is_user_in_team(user_id: i64, team_id: i64) -> Result<bool, DbErr> {
    Ok(crate::entity::team_user::Entity::find()
        .join(
            JoinType::InnerJoin,
            crate::entity::team_user::Relation::Team.def().rev(),
        )
        .filter(crate::entity::team_user::Column::UserId.eq(user_id))
        .filter(crate::entity::team_user::Column::TeamId.eq(team_id))
        .filter(crate::entity::team::Column::DeletedAt.is_null())
        .count(get_db())
        .await?
        > 0)
}

/// Check whether a user is in a game.
///
/// # Params
/// - `is_allowed`: Whether the user is allowed to access into the game.
///
/// ```sql
///  SELECT u.id AS user_id, t.game_id, t.is_allowed
///  FROM teams t
///     JOIN team_users tu ON t.id = tu.team_id
///  WHERE u.id = ? AND t.game_id = ? AND t.is_allowed = true;
/// ```
pub async fn is_user_in_game(
    user: &crate::transfer::User, game: &crate::transfer::Game, state: Option<State>,
) -> Result<bool, DbErr> {
    let mut sql = crate::entity::team::Entity::find()
        .join(
            JoinType::InnerJoin,
            crate::entity::team_user::Relation::Team.def().rev(),
        )
        .filter(crate::entity::team_user::Column::UserId.eq(user.id))
        .filter(crate::entity::team::Column::GameId.eq(game.id))
        .filter(crate::entity::team::Column::DeletedAt.is_null());

    if let Some(state) = state {
        sql = sql.filter(crate::entity::team::Column::State.eq(state));
    }

    Ok(sql.count(get_db()).await? > 0)
}
