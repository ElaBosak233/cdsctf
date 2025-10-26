use async_trait::async_trait;
use sea_orm::Statement;
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20251024_000004_create_game"
    }
}

#[async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"
                CREATE TABLE IF NOT EXISTS "games" (
                    "id" BIGSERIAL PRIMARY KEY,
                    "title" VARCHAR NOT NULL,
                    "sketch" TEXT,
                    "description" TEXT,
                    "is_enabled" BOOLEAN NOT NULL,
                    "is_public" BOOLEAN NOT NULL,
                    "member_limit_min" BIGINT NOT NULL DEFAULT 3,
                    "member_limit_max" BIGINT NOT NULL DEFAULT 3,
                    "is_need_write_up" BOOLEAN NOT NULL DEFAULT FALSE,
                    "timeslots" JSONB NOT NULL,
                    "started_at" BIGINT NOT NULL,
                    "frozen_at" BIGINT NOT NULL,
                    "ended_at" BIGINT NOT NULL,
                    "has_icon" BOOLEAN NOT NULL DEFAULT FALSE,
                    "has_poster" BOOLEAN NOT NULL DEFAULT FALSE,
                    "created_at" BIGINT NOT NULL
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
                DROP TABLE IF EXISTS "games";
            "#
            .to_owned(),
        ))
        .await?;

        Ok(())
    }
}
