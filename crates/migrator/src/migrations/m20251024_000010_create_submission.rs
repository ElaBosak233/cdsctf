use async_trait::async_trait;
use sea_orm::Statement;
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20251024_000010_create_submission"
    }
}

#[async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"
                CREATE TABLE IF NOT EXISTS "submissions" (
                    "id" BIGSERIAL PRIMARY KEY,
                    "content" TEXT NOT NULL,
                    "status" INTEGER NOT NULL DEFAULT 0,
                    "challenge_id" BIGINT NOT NULL,
                    "user_id" BIGINT NOT NULL,
                    "team_id" BIGINT,
                    "game_id" BIGINT,
                    "created_at" BIGINT NOT NULL,
                    "pts" BIGINT NOT NULL DEFAULT 0,
                    "rank" BIGINT NOT NULL DEFAULT 0,
                
                    CONSTRAINT fk_submissions_challenge FOREIGN KEY ("challenge_id")
                        REFERENCES challenges ("id") ON DELETE CASCADE,
                    CONSTRAINT fk_submissions_user FOREIGN KEY ("user_id")
                        REFERENCES users ("id") ON DELETE CASCADE,
                    CONSTRAINT fk_submissions_team FOREIGN KEY ("team_id")
                        REFERENCES teams ("id") ON DELETE CASCADE,
                    CONSTRAINT fk_submissions_game FOREIGN KEY ("game_id")
                        REFERENCES games ("id") ON DELETE CASCADE,
                
                    CONSTRAINT fk_submissions_game_challenge FOREIGN KEY ("game_id", "challenge_id")
                        REFERENCES game_challenges ("game_id", "challenge_id") ON DELETE CASCADE
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
                DROP TABLE IF EXISTS "submissions";
            "#
            .to_owned(),
        ))
        .await?;

        Ok(())
    }
}
