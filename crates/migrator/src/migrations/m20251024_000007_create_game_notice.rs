use async_trait::async_trait;
use sea_orm::Statement;
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20251024_000007_create_game_notice"
    }
}

#[async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"
                CREATE TABLE IF NOT EXISTS "game_notices" (
                    "id" BIGSERIAL PRIMARY KEY,
                    "game_id" BIGINT NOT NULL,
                    "title" VARCHAR NOT NULL,
                    "content" TEXT NOT NULL,
                    "created_at" BIGINT NOT NULL,
                
                    CONSTRAINT "fk_game_notices_game_id"
                        FOREIGN KEY ("game_id") REFERENCES "games" ("id")
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
                DROP TABLE IF EXISTS "game_notices";
            "#
            .to_owned(),
        ))
        .await?;

        Ok(())
    }
}
