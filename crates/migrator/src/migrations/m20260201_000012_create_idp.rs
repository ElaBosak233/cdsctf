//! SeaORM migration `m20260201_000012_create_idp` — creates Rune-backed IdP
//! and user binding tables.

use async_trait::async_trait;
use sea_orm::Statement;
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260201_000012_create_idp"
    }
}

#[async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"
                CREATE TABLE IF NOT EXISTS "idps" (
                    "id" BIGSERIAL PRIMARY KEY,
                    "name" VARCHAR(127) NOT NULL,
                    "enabled" BOOLEAN NOT NULL DEFAULT TRUE,
                    "has_avatar" BOOLEAN NOT NULL DEFAULT FALSE,
                    "portal" VARCHAR(255),
                    "script" TEXT NOT NULL,
                    "created_at" BIGINT NOT NULL,
                    "updated_at" BIGINT NOT NULL
                );
            "#
            .to_owned(),
        ))
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"
                DROP TABLE IF EXISTS "idps";
            "#
            .to_owned(),
        ))
        .await?;

        Ok(())
    }
}
