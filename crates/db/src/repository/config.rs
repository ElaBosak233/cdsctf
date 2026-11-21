//! Database access for `config` — SeaORM queries, updates, and DTOs.

use sea_orm::{
    ColumnTrait, ConnectionTrait, EntityTrait, PaginatorTrait, QueryFilter, Set,
    sea_query::{Expr, OnConflict},
};

pub use crate::entity::config::{ActiveModel, Config, Model, auth, captcha, email, meta};
pub(crate) use crate::entity::config::{Column, Entity};
use crate::traits::DbError;

/// Loads the singleton `configs` row and returns the embedded JSON
/// configuration DTO.
pub async fn get(conn: &impl ConnectionTrait) -> Result<Config, DbError> {
    Ok(Entity::find()
        .one(conn)
        .await?
        .ok_or_else(|| DbError::NotFound("config".to_string()))?
        .data)
}

/// Counts rows that match optional filters.
pub async fn count(conn: &impl ConnectionTrait) -> Result<u64, DbError> {
    Ok(Entity::find().count(conn).await?)
}

/// Saves configuration. An empty SMTP password keeps the stored password.
pub async fn save(conn: &impl ConnectionTrait, config: Config) -> Result<Config, DbError> {
    let previous = get(conn).await.ok();
    let mut on_conflict = OnConflict::column(Column::Id);
    if config.email.password.is_empty() {
        // Preserve the password from the row at write time, including
        // concurrent rotations.
        on_conflict.value(
            Column::Data,
            Expr::cust(
                r#"jsonb_set(EXCLUDED.data, '{email,password}', COALESCE(configs.data #> '{email,password}', '""'::jsonb))"#,
            ),
        );
    } else {
        on_conflict.update_column(Column::Data);
    }

    let saved = Entity::insert(ActiveModel {
        id: Set(true),
        data: Set(config),
    })
    .on_conflict(on_conflict)
    .exec_with_returning(conn)
    .await?
    .data;
    let changed_fields =
        changed_fields(&serde_json::json!(previous), &serde_json::json!(saved), "");
    let switches = serde_json::json!({
        "local_registration_enabled": saved.auth.local_registration_enabled,
        "email_enabled": saved.email.enabled,
    });
    tracing::info!(
        resource = "config",
        action = "save",
        changed_fields = ?changed_fields,
        switches = ?switches,
        "database step completed",
    );

    Ok(saved)
}

/// Updates only the logo reference in the singleton configuration.
pub async fn set_logo_hash(
    conn: &impl ConnectionTrait,
    hash: Option<String>,
) -> Result<(), DbError> {
    let result = Entity::update_many()
        .col_expr(
            Column::Data,
            Expr::cust_with_values(
                "jsonb_set(data, '{logo_hash}', $1)",
                [serde_json::json!(hash)],
            ),
        )
        .filter(Column::Id.eq(true))
        .exec(conn)
        .await?;

    if result.rows_affected == 0 {
        return Err(DbError::NotFound("config".to_owned()));
    }
    tracing::info!(
        resource = "config",
        action = "save",
        changed_fields = ?["logo_hash"],
        "database step completed",
    );

    Ok(())
}

/// Compare fixed configuration field paths without retaining their values.
/// SMTP/captcha credentials can change, but must never enter a log event.
fn changed_fields(
    before: &serde_json::Value,
    after: &serde_json::Value,
    prefix: &str,
) -> Vec<String> {
    if before == after {
        return Vec::new();
    }
    if let Some(after) = after.as_object() {
        after
            .iter()
            .flat_map(|(key, value)| {
                let path = if prefix.is_empty() {
                    key.clone()
                } else {
                    format!("{prefix}.{key}")
                };
                changed_fields(&before[key], value, &path)
            })
            .collect()
    } else {
        vec![prefix.to_owned()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn configuration_changes_record_paths_without_credential_values() {
        let before = serde_json::json!({"email": {"password": "old-secret", "enabled": true}});
        let after = serde_json::json!({"email": {"password": "new-secret", "enabled": true}});
        assert_eq!(changed_fields(&before, &after, ""), vec!["email.password"]);
        assert!(changed_fields(&after, &after, "").is_empty());
    }
}
