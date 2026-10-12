//! Creates user-bound personal access tokens.

use async_trait::async_trait;
use sea_orm::Statement;
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20261011_000014_create_personal_access_token"
    }
}

#[async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_raw(Statement::from_string(
                manager.get_database_backend(),
                r#"
                    CREATE TABLE IF NOT EXISTS "personal_access_tokens" (
                        "id" BIGSERIAL PRIMARY KEY,
                        "user_id" BIGINT NOT NULL,
                        "name" VARCHAR(64) NOT NULL,
                        "token_id" VARCHAR(32) UNIQUE NOT NULL,
                        "token_hash" VARCHAR(64) NOT NULL,
                        "expires_at" TIMESTAMPTZ NOT NULL,
                        "last_used_at" TIMESTAMPTZ,
                        "revoked_at" TIMESTAMPTZ,
                        "created_at" TIMESTAMPTZ NOT NULL,

                        CONSTRAINT "fk_personal_access_tokens_user_id"
                            FOREIGN KEY ("user_id") REFERENCES "users" ("id")
                            ON DELETE CASCADE
                    );
                "#
                .to_owned(),
            ))
            .await?;
        manager
            .get_connection()
            .execute_raw(Statement::from_string(
                manager.get_database_backend(),
                r#"
                    CREATE INDEX IF NOT EXISTS "idx_personal_access_tokens_user_id"
                        ON "personal_access_tokens" ("user_id");
                "#
                .to_owned(),
            ))
            .await?;
        Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}
