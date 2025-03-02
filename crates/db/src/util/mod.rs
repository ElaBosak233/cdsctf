use sea_orm::{
    ColumnTrait, DbErr, EntityTrait, JoinType, PaginatorTrait, QueryFilter, QuerySelect,
    RelationTrait,
};

use crate::{entity::user::Group, get_db};

pub fn can_user_modify_team(
    user: &crate::transfer::User, team: &crate::transfer::Team,
) -> bool {
    user.group == Group::Admin || team.users.iter().any(|t| t.id == user.id)
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
    user: &crate::transfer::User, game: &crate::transfer::Game, is_allowed: Option<bool>,
) -> Result<bool, DbErr> {
    let mut sql = crate::entity::team::Entity::find()
        .join(
            JoinType::InnerJoin,
            crate::entity::team_user::Relation::Team.def().rev(),
        )
        .filter(crate::entity::team_user::Column::UserId.eq(user.id))
        .filter(crate::entity::team::Column::GameId.eq(game.id));

    if let Some(is_allowed) = is_allowed {
        sql = sql.filter(crate::entity::team::Column::IsAllowed.eq(is_allowed));
    }

    Ok(sql.count(get_db()).await? > 0)
}
