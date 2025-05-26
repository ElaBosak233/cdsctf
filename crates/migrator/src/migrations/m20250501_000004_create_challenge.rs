use async_trait::async_trait;
use sea_orm::Statement;
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20250501_000004_create_challenge"
    }
}

#[async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"
                CREATE TABLE IF NOT EXISTS "challenges" (
                    "id" UUID PRIMARY KEY,
                    "title" VARCHAR NOT NULL,
                    "description" TEXT NOT NULL,
                    "category" INTEGER NOT NULL,
                    "tags" TEXT[] NOT NULL,
                    "is_dynamic" BOOLEAN NOT NULL,
                    "has_attachment" BOOLEAN NOT NULL,
                    "is_public" BOOLEAN NOT NULL,
                    "env" JSONB,
                    "checker" TEXT,
                    "deleted_at" BIGINT,
                    "created_at" BIGINT NOT NULL,
                    "updated_at" BIGINT NOT NULL
                );
            "#
            .to_owned(),
        ))
        .await?;

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"
                CREATE INDEX IF NOT EXISTS "idx_challenges_category"
                    ON "challenges" ("category");
            "#
            .to_owned(),
        ))
        .await?;

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"
                CREATE EXTENSION IF NOT EXISTS pg_trgm;
            "#
            .to_owned(),
        ))
        .await?;

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"
                CREATE INDEX IF NOT EXISTS "idx_challenges_title"
                    ON "challenges"
                    USING GIN ("title" gin_trgm_ops);
            "#
            .to_owned(),
        ))
        .await?;

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"
                CREATE INDEX IF NOT EXISTS "idx_challenges_tags"
                    ON "challenges"
                    USING GIN ("tags");
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
                DROP TABLE IF EXISTS "challenges";
            "#
            .to_owned(),
        ))
        .await?;

        Ok(())
    }
}
