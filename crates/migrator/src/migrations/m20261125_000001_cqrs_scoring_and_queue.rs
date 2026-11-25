//! Simplify judging and scoring schema for single-source-of-truth queueing and
//! CQRS read-model scoring:
//!
//! 1. Add partial work-queue index `idx_submissions_work_queue` so workers can
//!    claim `queued` and expired `processing` rows via `FOR UPDATE SKIP
//!    LOCKED`.
//! 2. Add partial unique indexes `uq_submissions_correct_game` and
//!    `uq_submissions_correct_standalone` so PostgreSQL enforces at-most-one
//!    `correct` solve per scope without application advisory locks.
//! 3. Upgrade `idx_submissions_scoring` with `INCLUDE (team_id)` and add
//!    `idx_teams_game_state` for fast index-only snapshot reads.
//! 4. Drop derived score columns (`pts`, `rank`) and `games.score_revision`
//!    that are now served from the Valkey CQRS score snapshot.

use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        // Defensively demote any legacy duplicate `correct` rows (keeping the
        // earliest `(created_at, id)` per scope) before enforcing uniqueness.
        db.execute_unprepared(
            r#"
                WITH ranked_game AS (
                    SELECT
                        id,
                        ROW_NUMBER() OVER (
                            PARTITION BY game_id, team_id, challenge_id
                            ORDER BY created_at ASC, id ASC
                        ) AS rn
                    FROM submissions
                    WHERE status = 'correct'
                      AND game_id IS NOT NULL
                      AND team_id IS NOT NULL
                )
                UPDATE submissions
                SET status = 'duplicate'
                WHERE id IN (SELECT id FROM ranked_game WHERE rn > 1);

                WITH ranked_standalone AS (
                    SELECT
                        id,
                        ROW_NUMBER() OVER (
                            PARTITION BY user_id, challenge_id
                            ORDER BY created_at ASC, id ASC
                        ) AS rn
                    FROM submissions
                    WHERE status = 'correct'
                      AND game_id IS NULL
                      AND team_id IS NULL
                )
                UPDATE submissions
                SET status = 'duplicate'
                WHERE id IN (SELECT id FROM ranked_standalone WHERE rn > 1);
            "#,
        )
        .await?;

        // Partial work-queue index: keeps only active queued/processing rows,
        // enabling fast `FOR UPDATE SKIP LOCKED` claims in FIFO order.
        db.execute_unprepared(
            r#"
                CREATE INDEX IF NOT EXISTS idx_submissions_work_queue
                ON submissions (created_at, id)
                WHERE status IN ('queued', 'processing');
            "#,
        )
        .await?;

        // Partial unique indexes replace manual `pg_advisory_xact_lock` checks
        // and guarantee at most one `correct` submission per solve scope.
        db.execute_unprepared(
            r#"
                CREATE UNIQUE INDEX IF NOT EXISTS uq_submissions_correct_game
                ON submissions (game_id, team_id, challenge_id)
                WHERE status = 'correct' AND game_id IS NOT NULL AND team_id IS NOT NULL;

                CREATE UNIQUE INDEX IF NOT EXISTS uq_submissions_correct_standalone
                ON submissions (user_id, challenge_id)
                WHERE status = 'correct' AND game_id IS NULL AND team_id IS NULL;
            "#,
        )
        .await?;

        // Covering index for index-only scoring snapshot queries, plus a team
        // state index so `SELECT id FROM teams WHERE game_id = $1 AND state =
        // 3` also runs as an index-only scan.
        db.execute_unprepared(
            r#"
                DROP INDEX IF EXISTS idx_submissions_scoring;
                CREATE INDEX idx_submissions_scoring
                ON submissions (game_id, challenge_id, created_at, id)
                INCLUDE (team_id)
                WHERE status = 'correct' AND game_id IS NOT NULL;

                CREATE INDEX IF NOT EXISTS idx_teams_game_state
                ON teams (game_id, state, id);
            "#,
        )
        .await?;

        // Drop materialized derived score columns and dirty revision counter.
        db.execute_unprepared(
            r#"
                ALTER TABLE games DROP COLUMN IF EXISTS score_revision;
                ALTER TABLE game_challenges DROP COLUMN IF EXISTS pts;
                ALTER TABLE teams
                    DROP COLUMN IF EXISTS pts,
                    DROP COLUMN IF EXISTS rank;
                ALTER TABLE submissions
                    DROP COLUMN IF EXISTS pts,
                    DROP COLUMN IF EXISTS rank;
            "#,
        )
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        db.execute_unprepared(
            r#"
                ALTER TABLE games
                    ADD COLUMN IF NOT EXISTS score_revision BIGINT NOT NULL DEFAULT 0;
                ALTER TABLE game_challenges
                    ADD COLUMN IF NOT EXISTS pts BIGINT NOT NULL DEFAULT 0;
                ALTER TABLE teams
                    ADD COLUMN IF NOT EXISTS pts BIGINT NOT NULL DEFAULT 0,
                    ADD COLUMN IF NOT EXISTS rank BIGINT NOT NULL DEFAULT 0;
                ALTER TABLE submissions
                    ADD COLUMN IF NOT EXISTS pts BIGINT NOT NULL DEFAULT 0,
                    ADD COLUMN IF NOT EXISTS rank BIGINT NOT NULL DEFAULT 0;

                DROP INDEX IF EXISTS idx_teams_game_state;
                DROP INDEX IF EXISTS uq_submissions_correct_standalone;
                DROP INDEX IF EXISTS uq_submissions_correct_game;
                DROP INDEX IF EXISTS idx_submissions_work_queue;
                DROP INDEX IF EXISTS idx_submissions_scoring;

                CREATE INDEX IF NOT EXISTS idx_submissions_scoring
                ON submissions (game_id, challenge_id, created_at, id)
                WHERE status = 'correct';
            "#,
        )
        .await?;

        Ok(())
    }
}
