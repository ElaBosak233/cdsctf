//! Normalizes the coarse-grained user permissions.

use async_trait::async_trait;
use sea_orm::ConnectionTrait;
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20261011_000016_normalize_permissions"
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
                    SET "permissions" = array_remove(
                        array_remove("permissions", 'management:instance'),
                        'management:submission'
                    );

                    UPDATE "users"
                    SET "permissions" = array_append("permissions", 'basic')
                    WHERE NOT ('basic' = ANY("permissions"));
                "#,
            )
            .await?;
        Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}
