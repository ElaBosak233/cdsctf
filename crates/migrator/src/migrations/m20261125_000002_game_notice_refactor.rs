//! Refactor `game_notices` for ordered feeds, optional `game_challenges`
//! binding (cascading on challenge removal), edit history, and admin author
//! tracking:
//!
//! 1. Add `challenge_id`, `created_by`, `updated_by`, and `updated_at` columns.
//! 2. Backfill `updated_at` from `created_at` on existing rows.
//! 3. Add composite foreign key `(game_id, challenge_id) -> game_challenges`
//!    (`ON DELETE CASCADE`), plus `ON DELETE SET NULL` foreign keys on
//!    `created_by` and `updated_by` referencing `users(id)`.
//! 4. Add covering sort index `(game_id, created_at DESC, id DESC)`.

use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        db.execute_unprepared(
            r#"
                ALTER TABLE "game_notices"
                    ADD COLUMN IF NOT EXISTS "challenge_id" BIGINT NULL,
                    ADD COLUMN IF NOT EXISTS "created_by" BIGINT NULL,
                    ADD COLUMN IF NOT EXISTS "updated_by" BIGINT NULL,
                    ADD COLUMN IF NOT EXISTS "updated_at" TIMESTAMPTZ;

                UPDATE "game_notices"
                SET "updated_at" = "created_at"
                WHERE "updated_at" IS NULL;

                ALTER TABLE "game_notices"
                    ALTER COLUMN "updated_at" SET NOT NULL;

                DO $$
                BEGIN
                    IF NOT EXISTS (
                        SELECT 1 FROM pg_constraint
                        WHERE conname = 'fk_game_notices_game_challenge'
                          AND conrelid = 'game_notices'::regclass
                    ) THEN
                        ALTER TABLE "game_notices"
                            ADD CONSTRAINT "fk_game_notices_game_challenge"
                                FOREIGN KEY ("game_id", "challenge_id")
                                REFERENCES "game_challenges" ("game_id", "challenge_id")
                                ON DELETE CASCADE;
                    END IF;

                    IF NOT EXISTS (
                        SELECT 1 FROM pg_constraint
                        WHERE conname = 'fk_game_notices_created_by'
                          AND conrelid = 'game_notices'::regclass
                    ) THEN
                        ALTER TABLE "game_notices"
                            ADD CONSTRAINT "fk_game_notices_created_by"
                                FOREIGN KEY ("created_by")
                                REFERENCES "users" ("id")
                                ON DELETE SET NULL;
                    END IF;

                    IF NOT EXISTS (
                        SELECT 1 FROM pg_constraint
                        WHERE conname = 'fk_game_notices_updated_by'
                          AND conrelid = 'game_notices'::regclass
                    ) THEN
                        ALTER TABLE "game_notices"
                            ADD CONSTRAINT "fk_game_notices_updated_by"
                                FOREIGN KEY ("updated_by")
                                REFERENCES "users" ("id")
                                ON DELETE SET NULL;
                    END IF;
                END
                $$;

                CREATE INDEX IF NOT EXISTS "idx_game_notices_game_created"
                ON "game_notices" ("game_id", "created_at" DESC, "id" DESC);
            "#,
        )
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        db.execute_unprepared(
            r#"
                DROP INDEX IF EXISTS "idx_game_notices_game_created";

                ALTER TABLE "game_notices"
                    DROP CONSTRAINT IF EXISTS "fk_game_notices_updated_by",
                    DROP CONSTRAINT IF EXISTS "fk_game_notices_created_by",
                    DROP CONSTRAINT IF EXISTS "fk_game_notices_game_challenge",
                    DROP COLUMN IF EXISTS "updated_at",
                    DROP COLUMN IF EXISTS "updated_by",
                    DROP COLUMN IF EXISTS "created_by",
                    DROP COLUMN IF EXISTS "challenge_id";
            "#,
        )
        .await?;

        Ok(())
    }
}
