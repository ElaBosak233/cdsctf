//! Expands legacy wildcard permissions into the explicit permission set.

use async_trait::async_trait;
use sea_orm::ConnectionTrait;
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20261011_000017_remove_wildcard_permissions"
    }
}

#[async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared(
                r#"
                    UPDATE "users"
                    SET "permissions" = ARRAY(
                        SELECT DISTINCT permission
                        FROM unnest(
                            array_cat(
                                array_remove(array_remove("permissions", '*'), 'management:*'),
                                ARRAY[
                                    'basic', 'management:config', 'management:challenge',
                                    'management:game', 'management:idp', 'management:user'
                                ]::TEXT[]
                            )
                        ) AS permission
                    )::TEXT[]
                    WHERE '*' = ANY("permissions")
                       OR 'management:*' = ANY("permissions");
                "#,
            )
            .await?;
        Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}
