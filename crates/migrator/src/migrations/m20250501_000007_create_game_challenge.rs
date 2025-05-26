use async_trait::async_trait;
use sea_orm::Statement;
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20250501_000007_create_game_challenge"
    }
}

#[async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"
                CREATE TABLE IF NOT EXISTS "game_challenges" (
                    "game_id" BIGINT NOT NULL,
                    "challenge_id" UUID NOT NULL,
                    "difficulty" BIGINT NOT NULL DEFAULT 1,
                    "max_pts" BIGINT NOT NULL DEFAULT 2000,
                    "min_pts" BIGINT NOT NULL DEFAULT 200,
                    "bonus_ratios" BIGINT[] NOT NULL,
                    "is_enabled" BOOLEAN NOT NULL DEFAULT FALSE,
                    "frozen_at" BIGINT,
                    "pts" BIGINT NOT NULL DEFAULT 0,

                    PRIMARY KEY ("game_id", "challenge_id"),
                    CONSTRAINT "fk_game_challenges_game_id"
                        FOREIGN KEY ("game_id") REFERENCES "games" ("id")
                        ON DELETE CASCADE,
                    CONSTRAINT "fk_game_challenges_challenge_id"
                        FOREIGN KEY ("challenge_id") REFERENCES "challenges" ("id")
                        ON DELETE CASCADE
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
                DROP TABLE IF EXISTS "game_challenges";
            "#
            .to_owned(),
        ))
        .await?;

        Ok(())
    }
}
