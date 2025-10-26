use async_trait::async_trait;
use sea_orm::Statement;
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20251024_000002_create_user"
    }
}

#[async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"
                CREATE TABLE IF NOT EXISTS "users" (
                    "id" BIGSERIAL PRIMARY KEY,
                    "name" VARCHAR NOT NULL,
                    "username" VARCHAR UNIQUE NOT NULL,
                    "description" TEXT,
                    "group" INTEGER NOT NULL,
                    "hashed_password" VARCHAR NOT NULL,
                    "has_avatar" BOOLEAN NOT NULL DEFAULT FALSE,
                    "deleted_at" BIGINT,
                    "created_at" BIGINT NOT NULL,
                    "updated_at" BIGINT NOT NULL
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
                DROP TABLE IF EXISTS "users";
            "#
            .to_owned(),
        ))
        .await?;

        Ok(())
    }
}
