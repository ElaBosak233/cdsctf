//! SeaORM migration `m20260806_000004_create_game` — applies forward/backward
//! schema changes.

use async_trait::async_trait;
use sea_orm::Statement;
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    /// Stable migration name string for SeaORM.
    fn name(&self) -> &str {
        "m20260806_000004_create_game"
    }
}

#[async_trait]
impl MigrationTrait for Migration {
    /// Applies forward DDL/DML for this migration.
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        db.execute_raw(Statement::from_string(
            manager.get_database_backend(),
            r#"
                CREATE TABLE IF NOT EXISTS "games" (
                    "id" BIGSERIAL PRIMARY KEY,
                    "title" VARCHAR NOT NULL,
                    "sketch" TEXT,
                    "description" TEXT,
                    "enabled" BOOLEAN NOT NULL,
                    "public" BOOLEAN NOT NULL,
                    "paused" BOOLEAN NOT NULL DEFAULT FALSE,
                    "blacked_out" BOOLEAN NOT NULL DEFAULT FALSE,
                    "member_limit_min" BIGINT NOT NULL DEFAULT 3,
                    "member_limit_max" BIGINT NOT NULL DEFAULT 3,
                    "writeup_required" BOOLEAN NOT NULL DEFAULT FALSE,
                    "timeslots" JSONB NOT NULL,
                    "started_at" BIGINT NOT NULL,
                    "frozen_at" BIGINT NOT NULL,
                    "ended_at" BIGINT NOT NULL,
                    "icon_hash" VARCHAR,
                    "poster_hash" VARCHAR,
                    "score_revision" BIGINT NOT NULL DEFAULT 0,
                    "created_at" BIGINT NOT NULL
                );
            "#
            .to_owned(),
        ))
        .await?;

        Ok(())
    }

    /// Rolls back this migration (reverse DDL/DML).
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        db.execute_raw(Statement::from_string(
            manager.get_database_backend(),
            r#"
                DROP TABLE IF EXISTS "games";
            "#
            .to_owned(),
        ))
        .await?;

        Ok(())
    }
}
