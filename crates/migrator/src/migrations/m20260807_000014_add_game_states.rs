//! Adds independently controlled pause and blackout states to games.

use async_trait::async_trait;
use sea_orm::Statement;
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260807_000014_add_game_states"
    }
}

#[async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_raw(Statement::from_string(
                manager.get_database_backend(),
                r#"
                    ALTER TABLE "games"
                        ADD COLUMN IF NOT EXISTS "paused" BOOLEAN NOT NULL DEFAULT FALSE,
                        ADD COLUMN IF NOT EXISTS "blacked_out" BOOLEAN NOT NULL DEFAULT FALSE;
                "#
                .to_owned(),
            ))
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_raw(Statement::from_string(
                manager.get_database_backend(),
                r#"
                    ALTER TABLE "games"
                        DROP COLUMN IF EXISTS "blacked_out",
                        DROP COLUMN IF EXISTS "paused";
                "#
                .to_owned(),
            ))
            .await?;

        Ok(())
    }
}
