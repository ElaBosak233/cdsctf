//! Lean submission finalization.
//!
//! - `Incorrect` and `Failed` commit with a single lock-free `UPDATE`.
//! - `Correct` validates freeze/end boundaries against `submission.created_at`
//!   (fair even under queue backlog) and relies on PostgreSQL's partial unique
//!   indexes (`uq_submissions_correct_*`) to detect concurrent duplicate solves
//!   without any advisory locks.
//! - `Cheat` updates the submission and bans both teams atomically in one short
//!   transaction, with no cross-table score locks.

use anyhow::{Context as _, anyhow};
use cds_db::{
    DB,
    sea_orm::{AccessMode, IsolationLevel, TransactionTrait},
    submission::{ClaimedSubmission, FinishResult, Status},
    team::State,
};

/// Semantic result produced by the Lua checker before platform policy applies.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Verdict {
    Correct,
    Incorrect,
    Failed,
    Cheat { peer_team_id: i64 },
}

/// Outcome of attempting to commit a terminal verdict for `submission.claims`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum FinalizeOutcome {
    /// The terminal status committed. `score_game_id` is `Some` when the game's
    /// CQRS score epoch must be invalidated.
    Committed {
        status: Status,
        score_game_id: Option<i64>,
    },
    /// Another worker or admin superseded this `claims` generation.
    LeaseLost,
}

/// Persists `verdict` and any associated policy effects (auto-ban on cheat).
pub(crate) async fn finalize(
    db: &DB,
    submission: &ClaimedSubmission,
    verdict: Verdict,
) -> Result<FinalizeOutcome, anyhow::Error> {
    match verdict {
        Verdict::Incorrect => finish_simple(db, submission, Status::Incorrect).await,
        Verdict::Failed => finish_simple(db, submission, Status::Failed).await,
        Verdict::Correct => finish_correct(db, submission).await,
        Verdict::Cheat { peer_team_id } => match (submission.game_id, submission.team_id) {
            (Some(game_id), Some(team_id)) => {
                finish_cheat(db, submission, game_id, team_id, peer_team_id).await
            }
            // A cheat verdict has no enforceable platform meaning outside a
            // game/team context.
            _ => finish_simple(db, submission, Status::Incorrect).await,
        },
    }
}

async fn finish_simple(
    db: &DB,
    submission: &ClaimedSubmission,
    status: Status,
) -> Result<FinalizeOutcome, anyhow::Error> {
    match cds_db::submission::finish_processing(
        &db.conn,
        submission.id,
        submission.claims,
        status.clone(),
    )
    .await?
    {
        FinishResult::Committed => Ok(FinalizeOutcome::Committed {
            status,
            score_game_id: None,
        }),
        FinishResult::LeaseLost => Ok(FinalizeOutcome::LeaseLost),
        FinishResult::DuplicateCorrect => {
            Err(anyhow!("unexpected_unique_violation_for_{status:?}"))
        }
    }
}

async fn finish_correct(
    db: &DB,
    submission: &ClaimedSubmission,
) -> Result<FinalizeOutcome, anyhow::Error> {
    if let (Some(game_id), Some(_team_id)) = (submission.game_id, submission.team_id) {
        let window = cds_db::game::find_freeze_window(&db.conn, game_id)
            .await?
            .context("game_not_found")?;
        let challenge_frozen_at =
            cds_db::game_challenge::find_frozen_at(&db.conn, game_id, submission.challenge_id)
                .await?
                .context("game_challenge_not_found")?;

        // Evaluate deadlines against `created_at` (submission admission time),
        // so queue delay never wrongly expires an on-time solve.
        let submitted_at = submission.created_at;
        if submitted_at > window.frozen_at
            || submitted_at > window.ended_at
            || challenge_frozen_at.is_some_and(|frozen_at| submitted_at > frozen_at)
        {
            return finish_simple(db, submission, Status::Expired).await;
        }
    }

    match cds_db::submission::finish_processing(
        &db.conn,
        submission.id,
        submission.claims,
        Status::Correct,
    )
    .await?
    {
        FinishResult::Committed => Ok(FinalizeOutcome::Committed {
            status: Status::Correct,
            score_game_id: submission.game_id,
        }),
        FinishResult::LeaseLost => Ok(FinalizeOutcome::LeaseLost),
        // Partial unique index rejected a concurrent/repeat solve -> demote to
        // `duplicate` cleanly without advisory locks.
        FinishResult::DuplicateCorrect => finish_simple(db, submission, Status::Duplicate).await,
    }
}

async fn finish_cheat(
    db: &DB,
    submission: &ClaimedSubmission,
    game_id: i64,
    team_id: i64,
    peer_team_id: i64,
) -> Result<FinalizeOutcome, anyhow::Error> {
    let transaction = db
        .conn
        .begin_with_config(
            Some(IsolationLevel::ReadCommitted),
            Some(AccessMode::ReadWrite),
        )
        .await?;

    match cds_db::submission::finish_processing(
        &transaction,
        submission.id,
        submission.claims,
        Status::Cheat,
    )
    .await?
    {
        FinishResult::Committed => {}
        FinishResult::LeaseLost => {
            transaction.rollback().await?;
            return Ok(FinalizeOutcome::LeaseLost);
        }
        FinishResult::DuplicateCorrect => {
            transaction.rollback().await?;
            return Err(anyhow!("unexpected_unique_violation_for_cheat"));
        }
    }

    let team =
        cds_db::team::find_by_id::<cds_db::team::Model>(&transaction, team_id, game_id).await?;
    let peer_team =
        cds_db::team::find_by_id::<cds_db::team::Model>(&transaction, peer_team_id, game_id)
            .await?;

    // Preserve the cross-game boundary: keep Cheat evidence on the submission,
    // but only ban teams when both belong to the submitting competition.
    let score_game_id = if team.is_some() && peer_team.is_some() {
        if !cds_db::team::set_state_in_game(&transaction, team_id, game_id, State::Banned).await? {
            transaction.rollback().await?;
            return Err(anyhow!("submitting_team_changed_during_finalization"));
        }
        if peer_team_id != team_id
            && !cds_db::team::set_state_in_game(&transaction, peer_team_id, game_id, State::Banned)
                .await?
        {
            transaction.rollback().await?;
            return Err(anyhow!("peer_team_changed_during_finalization"));
        }
        Some(game_id)
    } else {
        None
    };

    transaction.commit().await?;
    Ok(FinalizeOutcome::Committed {
        status: Status::Cheat,
        score_game_id,
    })
}

#[cfg(test)]
mod tests {
    use std::{
        sync::Arc,
        time::{SystemTime, UNIX_EPOCH},
    };

    use cds_db::sea_orm::{ConnectOptions, ConnectionTrait, Database, DbBackend, Statement};
    use tokio::sync::Barrier;

    use super::*;

    const GAME_ID: i64 = 7;

    async fn setup_schema() -> (cds_db::sea_orm::DatabaseConnection, DB, String) {
        let database_url = std::env::var("CDS_TEST_DATABASE_URL")
            .expect("CDS_TEST_DATABASE_URL must point to a migrated CdsCTF PostgreSQL database");
        let control = Database::connect(&database_url).await.unwrap();
        cds_migrator::run(&DB {
            conn: control.clone(),
        })
        .await
        .unwrap();
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let schema = format!("cds_finalizer_{}_{}", std::process::id(), nonce);

        control
            .execute_unprepared(&format!(
                r#"
                    CREATE SCHEMA {schema};
                    CREATE TABLE {schema}.users (LIKE public.users INCLUDING ALL);
                    CREATE TABLE {schema}.challenges (LIKE public.challenges INCLUDING ALL);
                    CREATE TABLE {schema}.games (LIKE public.games INCLUDING ALL);
                    CREATE TABLE {schema}.teams (LIKE public.teams INCLUDING ALL);
                    CREATE TABLE {schema}.game_challenges (LIKE public.game_challenges INCLUDING ALL);
                    CREATE TABLE {schema}.submissions (LIKE public.submissions INCLUDING ALL);

                    INSERT INTO {schema}.users (
                        id, name, username, description, banned, permissions,
                        hashed_password, avatar_hash, deleted_at, created_at, updated_at
                    ) VALUES
                        (1, 'Team user', 'team-user', NULL, FALSE, ARRAY[]::TEXT[], '', NULL, NULL, to_timestamp(0), to_timestamp(0)),
                        (2, 'Standalone user', 'standalone-user', NULL, FALSE, ARRAY[]::TEXT[], '', NULL, NULL, to_timestamp(0), to_timestamp(0));
                    INSERT INTO {schema}.challenges (
                        id, title, description, category, tags, has_instance,
                        has_attachment, has_writeup, public, instance, checker,
                        writeup, deleted_at, created_at, updated_at
                    ) VALUES (
                        10, 'Challenge', '', 0, ARRAY[]::TEXT[], FALSE, FALSE,
                        FALSE, TRUE, NULL, NULL, NULL, NULL, to_timestamp(0), to_timestamp(0)
                    );
                    INSERT INTO {schema}.games (
                        id, title, sketch, description, enabled, public, paused,
                        blacked_out, member_limit_min, member_limit_max,
                        writeup_required, timeslots, started_at, frozen_at,
                        ended_at, icon_hash, poster_hash, created_at
                    ) VALUES
                        (7, 'Game', NULL, NULL, TRUE, TRUE, FALSE, FALSE, 1, 5,
                         FALSE, '[]'::JSONB, to_timestamp(0), to_timestamp(4102444800),
                         to_timestamp(4102444800), NULL, NULL, to_timestamp(0)),
                        (8, 'Other game', NULL, NULL, TRUE, TRUE, FALSE, FALSE, 1, 5,
                         FALSE, '[]'::JSONB, to_timestamp(0), to_timestamp(4102444800),
                         to_timestamp(4102444800), NULL, NULL, to_timestamp(0));
                    INSERT INTO {schema}.teams (
                        id, game_id, name, email, slogan, avatar_hash, has_writeup, state
                    ) VALUES
                        (100, 7, 'Submitting', NULL, NULL, NULL, FALSE, 3),
                        (101, 7, 'Peer', NULL, NULL, NULL, FALSE, 3),
                        (102, 8, 'Other game peer', NULL, NULL, NULL, FALSE, 3);
                "#
            ))
            .await
            .unwrap();

        let mut options = ConnectOptions::new(database_url);
        options
            .max_connections(16)
            .min_connections(2)
            .set_schema_search_path(format!("{schema},public"));
        let db = DB {
            conn: Database::connect(options).await.unwrap(),
        };
        (control, db, schema)
    }

    fn claimed(
        id: i64,
        user_id: i64,
        team_id: Option<i64>,
        game_id: Option<i64>,
        claims: i64,
    ) -> ClaimedSubmission {
        ClaimedSubmission {
            id,
            content: "flag".to_owned(),
            challenge_id: 10,
            user_id,
            team_id,
            game_id,
            created_at: time::OffsetDateTime::from_unix_timestamp(1_000).unwrap(),
            claims,
        }
    }

    #[tokio::test]
    #[ignore = "requires CDS_TEST_DATABASE_URL pointing to migrated PostgreSQL"]
    async fn concurrent_correct_finalization_enforces_uniqueness_via_partial_index() {
        let (control, db, schema) = setup_schema().await;
        db.conn
            .execute_unprepared(
                r#"
                    INSERT INTO game_challenges (game_id, challenge_id, difficulty, max_pts, min_pts, bonus_ratios, enabled, frozen_at)
                    VALUES (7, 10, 10, 1000, 100, ARRAY[10, 0]::BIGINT[], TRUE, NULL);
                    INSERT INTO submissions (id, content, status, challenge_id, user_id, team_id, game_id, created_at, processing_at, claims)
                    SELECT id, 'flag', 'processing', 10, 1, 100, 7, to_timestamp(1000), to_timestamp(1700000000), 1
                    FROM generate_series(1000, 1009) AS id;
                    INSERT INTO submissions (id, content, status, challenge_id, user_id, team_id, game_id, created_at, processing_at, claims)
                    SELECT id, 'flag', 'processing', 10, 2, NULL, NULL, to_timestamp(1000), to_timestamp(1700000000), 1
                    FROM generate_series(2000, 2009) AS id;
                "#,
            )
            .await
            .unwrap();

        let barrier = Arc::new(Barrier::new(20));
        let mut tasks = Vec::new();
        for id in 1000..1010 {
            let db = db.clone();
            let barrier = Arc::clone(&barrier);
            tasks.push(tokio::spawn(async move {
                barrier.wait().await;
                finalize(
                    &db,
                    &claimed(id, 1, Some(100), Some(GAME_ID), 1),
                    Verdict::Correct,
                )
                .await
            }));
        }
        for id in 2000..2010 {
            let db = db.clone();
            let barrier = Arc::clone(&barrier);
            tasks.push(tokio::spawn(async move {
                barrier.wait().await;
                finalize(&db, &claimed(id, 2, None, None, 1), Verdict::Correct).await
            }));
        }

        for task in tasks {
            assert!(matches!(
                task.await.unwrap().unwrap(),
                FinalizeOutcome::Committed { .. }
            ));
        }

        let row = db
            .conn
            .query_one_raw(Statement::from_string(
                DbBackend::Postgres,
                r#"
                    SELECT
                        COUNT(*) FILTER (WHERE status = 'correct' AND game_id = 7) AS game_correct,
                        COUNT(*) FILTER (WHERE status = 'duplicate' AND game_id = 7) AS game_dup,
                        COUNT(*) FILTER (WHERE status = 'correct' AND game_id IS NULL) AS user_correct,
                        COUNT(*) FILTER (WHERE status = 'duplicate' AND game_id IS NULL) AS user_dup
                    FROM submissions
                "#,
            ))
            .await
            .unwrap()
            .unwrap();

        assert_eq!(row.try_get::<i64>("", "game_correct").unwrap(), 1);
        assert_eq!(row.try_get::<i64>("", "game_dup").unwrap(), 9);
        assert_eq!(row.try_get::<i64>("", "user_correct").unwrap(), 1);
        assert_eq!(row.try_get::<i64>("", "user_dup").unwrap(), 9);

        db.conn.close().await.unwrap();
        control
            .execute_unprepared(&format!("DROP SCHEMA {schema} CASCADE"))
            .await
            .unwrap();
    }
}
