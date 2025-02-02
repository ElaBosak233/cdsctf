use sea_orm::{
    ColumnTrait, DbErr, EntityTrait, JoinType, PaginatorTrait, QueryFilter, QuerySelect,
    RelationTrait,
};

use crate::{entity::user::Group, get_db};

pub fn can_user_modify_team(user: &crate::transfer::User, team: &crate::transfer::Team) -> bool {
    user.group == Group::Admin || user.teams.iter().any(|t| t.id == team.id)
}

/// Check whether a user is in a game.
///
/// # Params
/// - `is_allowed`: Whether the user is allowed to access into the game.
///
/// ```sql
///  SELECT u.id AS user_id, gt.game_id, gt.is_allowed
///  FROM users u
///     JOIN team_users tu ON u.id = tu.user_id
///     JOIN game_teams gt ON ut.team_id = gt.team_id
///  u.id = ? AND gt.game_id = ? AND gt.is_allowed = true;
/// ```
pub async fn is_user_in_game(
    user: &crate::transfer::User, game: &crate::transfer::Game, is_allowed: Option<bool>,
) -> Result<bool, DbErr> {
    let mut sql = crate::entity::user::Entity::find()
        .join(
            JoinType::InnerJoin,
            crate::entity::team_user::Relation::User.def().rev(),
        )
        .join(
            JoinType::InnerJoin,
            crate::entity::team_user::Relation::Team.def(),
        )
        .join(
            JoinType::InnerJoin,
            crate::entity::game_team::Relation::Team.def().rev(),
        )
        .filter(crate::entity::user::Column::Id.eq(user.id))
        .filter(crate::entity::game_team::Column::GameId.eq(game.id));

    if let Some(is_allowed) = is_allowed {
        sql = sql.filter(crate::entity::game_team::Column::IsAllowed.eq(is_allowed));
    }

    Ok(sql.count(get_db()).await? > 0)
}
