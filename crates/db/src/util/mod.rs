use sea_orm::{
    ColumnTrait, DbErr, EntityTrait, JoinType, PaginatorTrait, QueryFilter, QuerySelect,
    RelationTrait, prelude::Expr, sea_query::Func,
};

use crate::{entity::team::State, get_db};

pub async fn is_user_in_team(user_id: i64, team_id: i64) -> Result<bool, DbErr> {
    Ok(crate::entity::team_user::Entity::find()
        .filter(crate::entity::team_user::Column::UserId.eq(user_id))
        .filter(crate::entity::team_user::Column::TeamId.eq(team_id))
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
///     JOIN team_users tu ON t.id = tu.profile
///  WHERE u.id = ? AND t.game_id = ? AND t.is_allowed = true;
/// ```
pub async fn is_user_in_game(
    user_id: i64,
    game_id: i64,
    state: Option<State>,
) -> Result<bool, DbErr> {
    let mut sql = crate::entity::team::Entity::find()
        .join(
            JoinType::InnerJoin,
            crate::entity::team_user::Relation::Team.def().rev(),
        )
        .filter(crate::entity::team_user::Column::UserId.eq(user_id))
        .filter(crate::entity::team::Column::GameId.eq(game_id));

    if let Some(state) = state {
        sql = sql.filter(crate::entity::team::Column::State.eq(state));
    }

    Ok(sql.count(get_db()).await? > 0)
}

pub async fn is_user_email_unique(user_id: i64, email: &str) -> Result<bool, DbErr> {
    let user = crate::entity::user::Entity::find()
        .filter(
            Expr::expr(Func::lower(Expr::col(crate::entity::user::Column::Email)))
                .eq(email.to_lowercase()),
        )
        .one(get_db())
        .await?;

    Ok(user.map(|u| u.id == user_id).unwrap_or(true))
}

pub async fn is_user_username_unique(user_id: i64, username: &str) -> Result<bool, DbErr> {
    let user = crate::entity::user::Entity::find()
        .filter(
            Expr::expr(Func::lower(Expr::col(
                crate::entity::user::Column::Username,
            )))
            .eq(username.to_lowercase()),
        )
        .one(get_db())
        .await?;

    Ok(user.map(|u| u.id == user_id).unwrap_or(true))
}

pub async fn can_user_access_challenge(
    user_id: i64,
    challenge_id: uuid::Uuid,
) -> Result<bool, DbErr> {
    let Some(challenge) = crate::entity::challenge::Entity::find_by_id(challenge_id)
        .one(get_db())
        .await?
    else {
        return Ok(false);
    };

    if challenge.is_public {
        return Ok(true);
    }

    let now = chrono::Utc::now().timestamp();

    if crate::entity::game::Entity::find()
        .join(
            JoinType::InnerJoin,
            crate::entity::game_challenge::Relation::Game.def().rev(),
        )
        .join(
            JoinType::InnerJoin,
            crate::entity::team::Relation::Game.def().rev(),
        )
        .join(
            JoinType::InnerJoin,
            crate::entity::team_user::Relation::Team.def().rev(),
        )
        .filter(crate::entity::game_challenge::Column::ChallengeId.eq(challenge_id))
        .filter(crate::entity::team_user::Column::UserId.eq(user_id))
        .filter(crate::entity::team::Column::State.eq(State::Passed))
        .filter(crate::entity::game::Column::StartedAt.lte(now))
        .filter(crate::entity::game::Column::EndedAt.gte(now))
        .count(get_db())
        .await?
        > 0
    {
        return Ok(true);
    }

    Ok(false)
}
