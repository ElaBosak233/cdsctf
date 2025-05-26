use async_trait::async_trait;
use sea_orm::Statement;
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20250501_000008_create_team_user"
    }
}

#[async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"
                CREATE TABLE IF NOT EXISTS "team_users" (
                    "team_id" BIGINT NOT NULL,
                    "user_id" BIGINT NOT NULL,
                    
                    PRIMARY KEY ("team_id", "user_id"),
                    CONSTRAINT "fk_team_users_team_id"
                        FOREIGN KEY ("team_id") REFERENCES "teams" ("id")
                            ON DELETE CASCADE,
                    CONSTRAINT "fk_team_users_user_id"
                        FOREIGN KEY ("user_id") REFERENCES "users" ("id")
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
                DROP TABLE IF EXISTS "team_users";
            "#
            .to_owned(),
        ))
        .await?;

        Ok(())
    }
}
