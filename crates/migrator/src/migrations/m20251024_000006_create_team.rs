use async_trait::async_trait;
use sea_orm::Statement;
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20251024_000006_create_team"
    }
}

#[async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"
                CREATE TABLE IF NOT EXISTS "teams" (
                    "id" BIGSERIAL PRIMARY KEY,
                    "game_id" BIGINT NOT NULL,
                    "name" VARCHAR NOT NULL,
                    "email" VARCHAR,
                    "slogan" VARCHAR,
                    "has_avatar" BOOLEAN NOT NULL DEFAULT FALSE,
                    "state" INT NOT NULL,
                    "pts" BIGINT NOT NULL DEFAULT 0,
                    "rank" BIGINT NOT NULL DEFAULT 0,
                    
                    CONSTRAINT "fk_teams_game_id"
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
                DROP TABLE IF EXISTS "teams";
            "#
            .to_owned(),
        ))
        .await?;

        Ok(())
    }
}
