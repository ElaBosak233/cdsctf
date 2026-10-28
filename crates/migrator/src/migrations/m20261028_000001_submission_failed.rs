//! Distinguish checker failures from incorrect answers.

use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared(
                "ALTER TABLE submissions DROP CONSTRAINT chk_submissions_status,
             ADD CONSTRAINT chk_submissions_status CHECK (status IN (
                 'queued', 'processing', 'correct', 'incorrect',
                 'cheat', 'expired', 'duplicate', 'failed'
             ))",
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Preserve failed submissions for rechecking after a rollback.
        manager
            .get_connection()
            .execute_unprepared(
                "UPDATE submissions SET status = 'queued', processing_at = NULL,
             checked_at = NULL WHERE status = 'failed'",
            )
            .await?;
        manager
            .get_connection()
            .execute_unprepared(
                "ALTER TABLE submissions DROP CONSTRAINT chk_submissions_status,
             ADD CONSTRAINT chk_submissions_status CHECK (status IN (
                 'queued', 'processing', 'correct', 'incorrect',
                 'cheat', 'expired', 'duplicate'
             ))",
            )
            .await?;
        Ok(())
    }
}
