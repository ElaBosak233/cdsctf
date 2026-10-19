//! Database access for `config` — SeaORM queries, updates, and DTOs.

use sea_orm::{ConnectionTrait, EntityTrait, PaginatorTrait, Set, sea_query::OnConflict};

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

/// Upserts the singleton `configs` row with a new JSON payload (insert or
/// on-conflict update).
pub async fn save(conn: &impl ConnectionTrait, config: Config) -> Result<Config, DbError> {
    let previous = get(conn).await.ok();
    let changed_fields =
        changed_fields(&serde_json::json!(previous), &serde_json::json!(config), "");
    let switches = serde_json::json!({
        "local_registration_enabled": config.auth.local_registration_enabled,
        "email_enabled": config.email.enabled,
    });
    let model = ActiveModel {
        id: Set(true),
        data: Set(config),
    };

    Entity::insert(model.clone())
        .on_conflict(
            OnConflict::column(Column::Id)
                .update_column(Column::Data)
                .to_owned(),
        )
        .exec(conn)
        .await?;
    tracing::info!(
        resource = "config",
        action = "save",
        changed_fields = ?changed_fields,
        switches = ?switches,
        "database step completed",
    );

    Ok(get(conn).await?)
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
