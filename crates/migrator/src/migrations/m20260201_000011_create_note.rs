use async_trait::async_trait;
use sea_orm::Statement;
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260201_000011_create_note"
    }
}

#[async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"
                CREATE TABLE IF NOT EXISTS "notes" (
                    "id" BIGSERIAL PRIMARY KEY,
                    "content" TEXT NOT NULL,
                    "public" BOOLEAN NOT NULL DEFAULT FALSE,
                    "user_id" BIGINT NOT NULL,
                    "challenge_id" BIGINT NOT NULL,
                    "created_at" BIGINT NOT NULL,
                    "updated_at" BIGINT NOT NULL,
                
                    CONSTRAINT fk_notes_challenge FOREIGN KEY ("challenge_id")
                        REFERENCES challenges ("id") ON DELETE CASCADE,
                    CONSTRAINT fk_notes_user FOREIGN KEY ("user_id")
                        REFERENCES users ("id") ON DELETE CASCADE,
                    CONSTRAINT uq_notes_user_challenge UNIQUE ("user_id", "challenge_id")
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
                DROP TABLE IF EXISTS "notes";
            "#
            .to_owned(),
        ))
        .await?;

        Ok(())
    }
}
