//! Track creator and last editor on `challenges`:
//!
//! 1. Add nullable `created_by` and `updated_by` columns referencing
//!    `users(id)` with `ON DELETE SET NULL`.
//! 2. Backfill existing challenges whose author columns are NULL using the
//!    lowest-ID non-deleted user with `management:challenge` permission.

use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        db.execute_unprepared(
            r#"
                ALTER TABLE "challenges"
                    ADD COLUMN IF NOT EXISTS "created_by" BIGINT NULL,
                    ADD COLUMN IF NOT EXISTS "updated_by" BIGINT NULL;

                DO $$
                BEGIN
                    IF NOT EXISTS (
                        SELECT 1 FROM pg_constraint
                        WHERE conname = 'fk_challenges_created_by'
                          AND conrelid = 'challenges'::regclass
                    ) THEN
                        ALTER TABLE "challenges"
                            ADD CONSTRAINT "fk_challenges_created_by"
                                FOREIGN KEY ("created_by")
                                REFERENCES "users" ("id")
                                ON DELETE SET NULL;
                    END IF;

                    IF NOT EXISTS (
                        SELECT 1 FROM pg_constraint
                        WHERE conname = 'fk_challenges_updated_by'
                          AND conrelid = 'challenges'::regclass
                    ) THEN
                        ALTER TABLE "challenges"
                            ADD CONSTRAINT "fk_challenges_updated_by"
                                FOREIGN KEY ("updated_by")
                                REFERENCES "users" ("id")
                                ON DELETE SET NULL;
                    END IF;
                END
                $$;

                WITH first_challenge_manager AS (
                    SELECT "id"
                    FROM "users"
                    WHERE "deleted_at" IS NULL
                      AND 'management:challenge' = ANY("permissions")
                    ORDER BY "id" ASC
                    LIMIT 1
                )
                UPDATE "challenges"
                SET
                    "created_by" = COALESCE("created_by", (SELECT "id" FROM first_challenge_manager)),
                    "updated_by" = COALESCE("updated_by", (SELECT "id" FROM first_challenge_manager))
                WHERE "created_by" IS NULL OR "updated_by" IS NULL;
            "#,
        )
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        db.execute_unprepared(
            r#"
                ALTER TABLE "challenges"
                    DROP CONSTRAINT IF EXISTS "fk_challenges_updated_by",
                    DROP CONSTRAINT IF EXISTS "fk_challenges_created_by",
                    DROP COLUMN IF EXISTS "updated_by",
                    DROP COLUMN IF EXISTS "created_by";
            "#,
        )
        .await?;

        Ok(())
    }
}
