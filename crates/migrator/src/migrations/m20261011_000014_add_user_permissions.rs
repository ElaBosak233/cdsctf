//! Adds explicit account ban state and coarse-grained permissions.

use async_trait::async_trait;
use sea_orm::ConnectionTrait;
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20261011_000014_add_user_permissions"
    }
}

#[async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared(
            r#"
                ALTER TABLE "users"
                    ADD COLUMN IF NOT EXISTS "banned" BOOLEAN NOT NULL DEFAULT FALSE,
                    ADD COLUMN IF NOT EXISTS "permissions" JSONB NOT NULL DEFAULT '[]'::jsonb;

                UPDATE "users"
                SET "banned" = TRUE
                WHERE "group" = 1;

                UPDATE "users"
                SET "permissions" = '["management:access"]'::jsonb
                WHERE "group" = 3 AND "permissions" = '[]'::jsonb;

                ALTER TABLE "users"
                    DROP COLUMN IF EXISTS "group";
            "#,
        )
        .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared(
            r#"
                ALTER TABLE "users"
                    ADD COLUMN IF NOT EXISTS "group" INTEGER NOT NULL DEFAULT 2;

                UPDATE "users"
                SET "group" = 1
                WHERE "banned";

                UPDATE "users"
                SET "group" = 3
                WHERE "permissions" @> '["management:access"]'::jsonb;

                ALTER TABLE "users"
                    DROP COLUMN IF EXISTS "permissions",
                    DROP COLUMN IF EXISTS "banned";
            "#,
        )
        .await?;
        Ok(())
    }
}
