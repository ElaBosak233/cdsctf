//! SeaORM migration `m20260201_000003_create_email` — applies forward/backward
//! schema changes.

use async_trait::async_trait;
use sea_orm::Statement;
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    /// Stable migration name string for SeaORM.
    fn name(&self) -> &str {
        "m20260201_000003_create_email"
    }
}

#[async_trait]
impl MigrationTrait for Migration {
    /// Applies forward DDL/DML for this migration.
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"
                CREATE TABLE IF NOT EXISTS "emails" (
                    "email" VARCHAR UNIQUE NOT NULL PRIMARY KEY,
                    "user_id" BIGINT NOT NULL,
                    "verified" BOOLEAN NOT NULL DEFAULT FALSE,

                    CONSTRAINT "fk_emails_user_id"
                        FOREIGN KEY ("user_id") REFERENCES "users" ("id")
                            ON DELETE CASCADE
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

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"
                DROP TABLE IF EXISTS "emails";
            "#
            .to_owned(),
        ))
        .await?;

        Ok(())
    }
}
