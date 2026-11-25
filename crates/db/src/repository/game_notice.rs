//! Database access for `game_notice` — SeaORM queries, updates, and DTOs.

use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, FromQueryResult, QueryFilter,
    Statement,
};

pub(crate) use crate::entity::game_notice::{Column, Entity};
use crate::traits::DbError;
pub use crate::{
    dto::game_notice::{AdminGameNoticeView, GameNoticeView},
    entity::game_notice::{ActiveModel, Model},
};

const PLAYER_VIEW_SELECT: &str = r#"
    SELECT
        gn.id,
        gn.game_id,
        gn.challenge_id,
        c.title AS challenge_title,
        c.category AS challenge_category,
        gn.title,
        gn.content,
        gn.created_at,
        gn.updated_at
    FROM game_notices gn
    LEFT JOIN game_challenges gc
        ON gc.game_id = gn.game_id AND gc.challenge_id = gn.challenge_id
    LEFT JOIN challenges c
        ON c.id = gn.challenge_id
    WHERE gn.game_id = $1
      AND (gn.challenge_id IS NULL OR gc.enabled = TRUE)
    ORDER BY gn.created_at DESC, gn.id DESC
"#;

const ADMIN_VIEW_LIST_SELECT: &str = r#"
    SELECT
        gn.id,
        gn.game_id,
        gn.challenge_id,
        c.title AS challenge_title,
        c.category AS challenge_category,
        gn.title,
        gn.content,
        gn.created_by,
        COALESCE(NULLIF(creator.name, ''), creator.username) AS created_by_name,
        creator.avatar_hash AS created_by_avatar_hash,
        gn.updated_by,
        COALESCE(NULLIF(editor.name, ''), editor.username) AS updated_by_name,
        editor.avatar_hash AS updated_by_avatar_hash,
        gn.created_at,
        gn.updated_at
    FROM game_notices gn
    LEFT JOIN challenges c
        ON c.id = gn.challenge_id
    LEFT JOIN users creator
        ON creator.id = gn.created_by
    LEFT JOIN users editor
        ON editor.id = gn.updated_by
    WHERE gn.game_id = $1
    ORDER BY gn.created_at DESC, gn.id DESC
"#;

const ADMIN_VIEW_BY_ID_SELECT: &str = r#"
    SELECT
        gn.id,
        gn.game_id,
        gn.challenge_id,
        c.title AS challenge_title,
        c.category AS challenge_category,
        gn.title,
        gn.content,
        gn.created_by,
        COALESCE(NULLIF(creator.name, ''), creator.username) AS created_by_name,
        creator.avatar_hash AS created_by_avatar_hash,
        gn.updated_by,
        COALESCE(NULLIF(editor.name, ''), editor.username) AS updated_by_name,
        editor.avatar_hash AS updated_by_avatar_hash,
        gn.created_at,
        gn.updated_at
    FROM game_notices gn
    LEFT JOIN challenges c
        ON c.id = gn.challenge_id
    LEFT JOIN users creator
        ON creator.id = gn.created_by
    LEFT JOIN users editor
        ON editor.id = gn.updated_by
    WHERE gn.id = $1
      AND gn.game_id = $2
    LIMIT 1
"#;

/// Looks up the raw entity model for a notice scoped to a game.
pub async fn find_model_by_id(
    conn: &impl ConnectionTrait,
    notice_id: i64,
    game_id: i64,
) -> Result<Option<Model>, DbError> {
    Ok(Entity::find_by_id(notice_id)
        .filter(Column::GameId.eq(game_id))
        .one(conn)
        .await?)
}

/// Lists player-visible notices for a game in reverse chronological order.
///
/// Notices bound to disabled challenges are excluded so unpublished challenges
/// never leak through announcements.
pub async fn find_player_views_by_game_id(
    conn: &impl ConnectionTrait,
    game_id: i64,
) -> Result<Vec<GameNoticeView>, DbError> {
    let stmt = Statement::from_sql_and_values(
        conn.get_database_backend(),
        PLAYER_VIEW_SELECT,
        [game_id.into()],
    );
    Ok(GameNoticeView::find_by_statement(stmt).all(conn).await?)
}

/// Lists all notices for a game (with author & last-editor usernames) in
/// reverse chronological order for administrators.
pub async fn find_admin_views_by_game_id(
    conn: &impl ConnectionTrait,
    game_id: i64,
) -> Result<Vec<AdminGameNoticeView>, DbError> {
    let stmt = Statement::from_sql_and_values(
        conn.get_database_backend(),
        ADMIN_VIEW_LIST_SELECT,
        [game_id.into()],
    );
    Ok(AdminGameNoticeView::find_by_statement(stmt)
        .all(conn)
        .await?)
}

/// Looks up one admin notice view by id and game id.
pub async fn find_admin_view_by_id(
    conn: &impl ConnectionTrait,
    notice_id: i64,
    game_id: i64,
) -> Result<Option<AdminGameNoticeView>, DbError> {
    let stmt = Statement::from_sql_and_values(
        conn.get_database_backend(),
        ADMIN_VIEW_BY_ID_SELECT,
        [notice_id.into(), game_id.into()],
    );
    Ok(AdminGameNoticeView::find_by_statement(stmt)
        .one(conn)
        .await?)
}

/// Inserts a new notice and returns the hydrated admin view.
pub async fn create(
    conn: &impl ConnectionTrait,
    model: ActiveModel,
) -> Result<AdminGameNoticeView, DbError> {
    let game_notice = model.insert(conn).await?;
    tracing::info!(
        resource = "game_notice",
        action = "create",
        notice_id = game_notice.id,
        game_id = game_notice.game_id,
        challenge_id = ?game_notice.challenge_id,
        created_by = ?game_notice.created_by,
        title = ?game_notice.title,
        "database step completed",
    );

    find_admin_view_by_id(conn, game_notice.id, game_notice.game_id)
        .await?
        .ok_or_else(|| DbError::NotFound(format!("game_notice_{}", game_notice.id)))
}

/// Updates an existing notice and returns the hydrated admin view.
pub async fn update(
    conn: &impl ConnectionTrait,
    model: ActiveModel,
) -> Result<AdminGameNoticeView, DbError> {
    let game_notice = model.update(conn).await?;
    tracing::info!(
        resource = "game_notice",
        action = "update",
        notice_id = game_notice.id,
        game_id = game_notice.game_id,
        challenge_id = ?game_notice.challenge_id,
        updated_by = ?game_notice.updated_by,
        title = ?game_notice.title,
        "database step completed",
    );

    find_admin_view_by_id(conn, game_notice.id, game_notice.game_id)
        .await?
        .ok_or_else(|| DbError::NotFound(format!("game_notice_{}", game_notice.id)))
}

/// Deletes a notice scoped to a game.
pub async fn delete(
    conn: &impl ConnectionTrait,
    notice_id: i64,
    game_id: i64,
) -> Result<(), DbError> {
    Entity::delete_many()
        .filter(Column::Id.eq(notice_id))
        .filter(Column::GameId.eq(game_id))
        .exec(conn)
        .await?;
    tracing::info!(
        resource = "game_notice",
        action = "delete",
        notice_id = notice_id,
        game_id = game_id,
        "resource step completed"
    );

    Ok(())
}
