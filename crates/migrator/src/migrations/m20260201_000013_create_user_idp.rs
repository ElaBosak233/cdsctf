//! SeaORM migration `m20260201_000013_create_user_idp` — creates user IdP
//! binding table.

use async_trait::async_trait;
use sea_orm::Statement;
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260201_000013_create_user_idp"
    }
}

#[async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"
                CREATE TABLE IF NOT EXISTS "user_idps" (
                    "id" BIGSERIAL PRIMARY KEY,
                    "user_id" BIGINT NOT NULL,
                    "idp_id" BIGINT NOT NULL,
                    "auth_key" VARCHAR(255) NOT NULL,
                    "data" JSONB,
                    "created_at" BIGINT NOT NULL,
                    "updated_at" BIGINT NOT NULL,

                    CONSTRAINT "fk_user_idps_user_id"
                        FOREIGN KEY ("user_id") REFERENCES "users" ("id")
                            ON DELETE CASCADE,
                    CONSTRAINT "fk_user_idps_idp_id"
                        FOREIGN KEY ("idp_id") REFERENCES "idps" ("id")
                            ON DELETE CASCADE,
                    CONSTRAINT "uq_user_idps_idp_id_auth_key"
                        UNIQUE ("idp_id", "auth_key"),
                    CONSTRAINT "uq_user_idps_user_id_idp_id"
                        UNIQUE ("user_id", "idp_id")
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
                DROP TABLE IF EXISTS "user_idps";
            "#
            .to_owned(),
        ))
        .await?;

        Ok(())
    }
}
