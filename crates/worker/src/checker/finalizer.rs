//! Transactional submission finalization.
//!
//! Checker execution stays outside the database transaction. This module owns
//! the smaller durable transition that follows it: the terminal submission
//! status and any score or policy effect. Correct results serialize
//! solved-state policy by team or standalone user before inspecting prior
//! solves. A valid in-game cheat additionally takes the same per-game advisory
//! lock used by the calculator before changing team state and the score
//! revision.

use anyhow::{Context as _, anyhow};
use cds_db::{
    DB, GameDetail, SubmissionView,
    sea_orm::{AccessMode, ConnectionTrait, IsolationLevel, TransactionTrait},
    submission::Status,
    team::State,
};

/// Semantic result produced by the checker before platform policy is applied.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Verdict {
    Correct,
    Incorrect,
    Cheat { peer_team_id: i64 },
}

/// Result of attempting to finalize the submission owned by `processing_at`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum FinalizeOutcome {
    /// The terminal transition committed. A game id means the calculator must
    /// be notified after the transaction has committed.
    Committed {
        status: Status,
        score_game_id: Option<i64>,
    },
    /// Another worker took over or finished the row before this worker wrote.
    LeaseLost,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct CheatPolicy {
    game_id: i64,
    team_id: i64,
    peer_team_id: i64,
}

/// Commits a checker verdict and its associated durable effects.
pub(crate) async fn finalize(
    db: &DB,
    submission: &SubmissionView,
    processing_at: i64,
    verdict: Verdict,
) -> Result<FinalizeOutcome, anyhow::Error> {
    let transaction = db
        .conn
        .begin_with_config(
            Some(IsolationLevel::ReadCommitted),
            Some(AccessMode::ReadWrite),
        )
        .await?;

    let (status, cheat) = resolve_status(&transaction, submission, verdict).await?;

    // The calculator takes this lock before reading or writing any score
    // inputs. Taking it before checking the lease gives cheat policy and score
    // calculation a single ordering across application instances. Ordinary
    // verdicts intentionally skip it, so incorrect submissions never contend
    // with the scoring path.
    if let Some(cheat) = cheat {
        cds_db::game::lock_score_recalculation(&transaction, cheat.game_id).await?;
    }

    let Some(finalized) = cds_db::submission::finish_processing(
        &transaction,
        submission.id,
        processing_at,
        status.clone(),
    )
    .await?
    else {
        transaction.rollback().await?;
        return Ok(FinalizeOutcome::LeaseLost);
    };

    let score_game_id = if let Some(cheat) = cheat {
        apply_cheat_policy(&transaction, cheat).await?
    } else if status == Status::Correct {
        if let Some(game_id) = finalized.game_id {
            cds_db::game::request_score_recalculation(&transaction, game_id).await?;
            Some(game_id)
        } else {
            None
        }
    } else {
        None
    };

    transaction.commit().await?;
    Ok(FinalizeOutcome::Committed {
        status,
        score_game_id,
    })
}

async fn resolve_status(
    transaction: &impl ConnectionTrait,
    submission: &SubmissionView,
    verdict: Verdict,
) -> Result<(Status, Option<CheatPolicy>), anyhow::Error> {
    match verdict {
        Verdict::Correct => Ok((resolve_correct_status(transaction, submission).await?, None)),
        Verdict::Incorrect => Ok((Status::Incorrect, None)),
        Verdict::Cheat { peer_team_id } => match (submission.game_id, submission.team_id) {
            (Some(game_id), Some(team_id)) => Ok((
                Status::Cheat,
                Some(CheatPolicy {
                    game_id,
                    team_id,
                    peer_team_id,
                }),
            )),
            // A cheat verdict has no enforceable platform meaning without both
            // sides of the authenticated game context.
            _ => Ok((Status::Incorrect, None)),
        },
    }
}

async fn resolve_correct_status(
    transaction: &impl ConnectionTrait,
    submission: &SubmissionView,
) -> Result<Status, anyhow::Error> {
    if let (Some(game_id), Some(_team_id)) = (submission.game_id, submission.team_id) {
        let game = cds_db::game::find_by_id::<GameDetail>(transaction, game_id)
            .await?
            .context("game_not_found")?;
        let game_challenge =
            cds_db::game_challenge::find_by_id(transaction, game_id, submission.challenge_id)
                .await?
                .context("game_challenge_not_found")?;
        let now = time::OffsetDateTime::now_utc().unix_timestamp();
        if now > game.frozen_at
            || now > game.ended_at
            || game_challenge
                .frozen_at
                .is_some_and(|frozen_at| now > frozen_at)
        {
            return Ok(Status::Expired);
        }
    }

    cds_db::submission::lock_solve_owner(transaction, submission).await?;
    if cds_db::submission::has_other_correct_in_scope(transaction, submission).await? {
        Ok(Status::Duplicate)
    } else {
        Ok(Status::Correct)
    }
}

async fn apply_cheat_policy(
    transaction: &cds_db::sea_orm::DatabaseTransaction,
    cheat: CheatPolicy,
) -> Result<Option<i64>, anyhow::Error> {
    let team =
        cds_db::team::find_by_id::<cds_db::team::Model>(transaction, cheat.team_id, cheat.game_id)
            .await?;
    let peer_team = cds_db::team::find_by_id::<cds_db::team::Model>(
        transaction,
        cheat.peer_team_id,
        cheat.game_id,
    )
    .await?;

    // Preserve the existing boundary for a peer outside the submitting game:
    // retain the Cheat terminal evidence, but do not change either team.
    if team.is_none() || peer_team.is_none() {
        return Ok(None);
    }

    if !cds_db::team::set_state_in_game(transaction, cheat.team_id, cheat.game_id, State::Banned)
        .await?
    {
        return Err(anyhow!("submitting_team_changed_during_finalization"));
    }
    if cheat.peer_team_id != cheat.team_id
        && !cds_db::team::set_state_in_game(
            transaction,
            cheat.peer_team_id,
            cheat.game_id,
            State::Banned,
        )
        .await?
    {
        return Err(anyhow!("peer_team_changed_during_finalization"));
    }

    cds_db::game::request_score_recalculation(transaction, cheat.game_id).await?;
    Ok(Some(cheat.game_id))
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
    const PROCESSING_AT: i64 = 1_700_000_000;

    async fn test_db() -> DB {
        let database_url = std::env::var("CDS_TEST_DATABASE_URL")
            .expect("CDS_TEST_DATABASE_URL must point to a migrated CdsCTF PostgreSQL database");
        let mut options = ConnectOptions::new(database_url);
        options.max_connections(1).min_connections(1);
        let conn = Database::connect(options).await.unwrap();

        // A one-connection pool keeps these shadow tables visible to every
        // finalizer transaction. Closing the connection removes all test data.
        conn.execute_unprepared(
            r#"
                CREATE TEMP TABLE users AS SELECT * FROM public.users WITH NO DATA;
                CREATE TEMP TABLE challenges AS SELECT * FROM public.challenges WITH NO DATA;
                CREATE TEMP TABLE games AS SELECT * FROM public.games WITH NO DATA;
                CREATE TEMP TABLE teams AS SELECT * FROM public.teams WITH NO DATA;
                CREATE TEMP TABLE game_challenges AS
                    SELECT * FROM public.game_challenges WITH NO DATA;
                CREATE TEMP TABLE submissions AS
                    SELECT * FROM public.submissions WITH NO DATA;
                ALTER TABLE teams ADD CONSTRAINT reject_atomicity_peer_ban
                    CHECK (id <> 105 OR state <> 0);

                INSERT INTO users (
                    id, name, username, description, "group", hashed_password,
                    avatar_hash, deleted_at, created_at, updated_at
                ) VALUES (1, 'User', 'user', NULL, 2, '', NULL, NULL, 0, 0);
                INSERT INTO challenges (
                    id, title, description, category, tags, has_instance,
                    has_attachment, has_writeup, public, instance, checker,
                    writeup, deleted_at, created_at, updated_at
                ) VALUES (
                    10, 'Challenge', '', 0, ARRAY[]::TEXT[], FALSE, FALSE,
                    FALSE, TRUE, NULL, NULL, NULL, NULL, 0, 0
                );
                INSERT INTO games (
                    id, title, sketch, description, enabled, public, paused,
                    blacked_out, member_limit_min, member_limit_max,
                    writeup_required, timeslots, started_at, frozen_at,
                    ended_at, icon_hash, poster_hash, score_revision, created_at
                ) VALUES
                    (7, 'Game', NULL, NULL, TRUE, TRUE, FALSE, FALSE, 1, 5,
                     FALSE, '[]'::JSONB, 0, 4102444800, 4102444800, NULL,
                     NULL, 0, 0),
                    (8, 'Other game', NULL, NULL, TRUE, TRUE, FALSE, FALSE, 1,
                     5, FALSE, '[]'::JSONB, 0, 4102444800, 4102444800, NULL,
                     NULL, 0, 0);
                INSERT INTO teams (
                    id, game_id, name, email, slogan, avatar_hash, has_writeup,
                    state, pts, rank
                ) VALUES
                    (100, 7, 'Submitting', NULL, NULL, NULL, FALSE, 3, 0, 0),
                    (101, 7, 'Peer', NULL, NULL, NULL, FALSE, 3, 0, 0),
                    (102, 8, 'Other game peer', NULL, NULL, NULL, FALSE, 3, 0, 0),
                    (104, 7, 'Rollback submitter', NULL, NULL, NULL, FALSE, 3, 0, 0),
                    (105, 7, 'Rollback peer', NULL, NULL, NULL, FALSE, 3, 0, 0);
                INSERT INTO game_challenges (
                    game_id, challenge_id, difficulty, max_pts, min_pts,
                    bonus_ratios, enabled, frozen_at, pts
                ) VALUES
                    (7, 10, 10, 1000, 100, ARRAY[10, 0]::BIGINT[], TRUE, NULL, 0);
                INSERT INTO submissions (
                    id, content, status, challenge_id, user_id, team_id,
                    game_id, created_at, processing_at, checked_at, pts, rank
                ) VALUES
                    (1, 'flag', 'processing', 10, 1, 100, 7, 0, 1700000000, NULL, 0, 0),
                    (2, 'flag', 'processing', 10, 1, 100, 7, 0, 1700000000, NULL, 0, 0),
                    (3, 'flag', 'processing', 10, 1, 100, 7, 0, 1700000000, NULL, 0, 0),
                    (4, 'flag', 'processing', 10, 1, 104, 7, 0, 1700000000, NULL, 0, 0),
                    (5, 'flag', 'processing', 10, 1, 100, 7, 0, 1700000000, NULL, 0, 0),
                    (6, 'flag', 'processing', 10, 1, 100, 7, 0, 1700000000, NULL, 0, 0),
                    (7, 'flag', 'processing', 10, 1, NULL, NULL, 0, 1700000000, NULL, 0, 0);
            "#,
        )
        .await
        .unwrap();

        DB { conn }
    }

    async fn schema_db(database_url: &str, schema: &str) -> DB {
        let mut options = ConnectOptions::new(database_url);
        options
            .max_connections(8)
            .min_connections(1)
            .set_schema_search_path(format!("{schema},public"));
        DB {
            conn: Database::connect(options).await.unwrap(),
        }
    }

    async fn concurrent_test_dbs() -> (cds_db::sea_orm::DatabaseConnection, DB, DB, String) {
        let database_url = std::env::var("CDS_TEST_DATABASE_URL")
            .expect("CDS_TEST_DATABASE_URL must point to a migrated CdsCTF PostgreSQL database");
        let control = Database::connect(&database_url).await.unwrap();
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let schema = format!("cds_finalizer_{}_{}", std::process::id(), nonce);

        control
            .execute_unprepared(&format!(
                r#"
                    CREATE SCHEMA {schema};
                    CREATE TABLE {schema}.users
                        (LIKE public.users INCLUDING ALL);
                    CREATE TABLE {schema}.challenges
                        (LIKE public.challenges INCLUDING ALL);
                    CREATE TABLE {schema}.games
                        (LIKE public.games INCLUDING ALL);
                    CREATE TABLE {schema}.teams
                        (LIKE public.teams INCLUDING ALL);
                    CREATE TABLE {schema}.game_challenges
                        (LIKE public.game_challenges INCLUDING ALL);
                    CREATE TABLE {schema}.submissions
                        (LIKE public.submissions INCLUDING ALL);

                    INSERT INTO {schema}.users (
                        id, name, username, description, "group", hashed_password,
                        avatar_hash, deleted_at, created_at, updated_at
                    ) VALUES
                        (1, 'Team user', 'team-user', NULL, 2, '', NULL, NULL, 0, 0),
                        (2, 'Standalone user', 'standalone-user', NULL, 2, '', NULL, NULL, 0, 0);
                    INSERT INTO {schema}.challenges (
                        id, title, description, category, tags, has_instance,
                        has_attachment, has_writeup, public, instance, checker,
                        writeup, deleted_at, created_at, updated_at
                    ) VALUES (
                        10, 'Challenge', '', 0, ARRAY[]::TEXT[], FALSE, FALSE,
                        FALSE, TRUE, NULL, NULL, NULL, NULL, 0, 0
                    );
                    INSERT INTO {schema}.games (
                        id, title, sketch, description, enabled, public, paused,
                        blacked_out, member_limit_min, member_limit_max,
                        writeup_required, timeslots, started_at, frozen_at,
                        ended_at, icon_hash, poster_hash, score_revision, created_at
                    ) VALUES (
                        7, 'Game', NULL, NULL, TRUE, TRUE, FALSE, FALSE, 1, 5,
                        FALSE, '[]'::JSONB, 0, 4102444800, 4102444800, NULL,
                        NULL, 0, 0
                    );
                    INSERT INTO {schema}.teams (
                        id, game_id, name, email, slogan, avatar_hash, has_writeup,
                        state, pts, rank
                    ) VALUES (100, 7, 'Team', NULL, NULL, NULL, FALSE, 3, 0, 0);
                    INSERT INTO {schema}.game_challenges (
                        game_id, challenge_id, difficulty, max_pts, min_pts,
                        bonus_ratios, enabled, frozen_at, pts
                    ) VALUES (7, 10, 10, 1000, 100, ARRAY[10, 0]::BIGINT[], TRUE, NULL, 0);

                    INSERT INTO {schema}.submissions (
                        id, content, status, challenge_id, user_id, team_id,
                        game_id, created_at, processing_at, checked_at, pts, rank
                    )
                    SELECT id, 'flag', 'processing', 10, 1, 100, 7, id,
                           {PROCESSING_AT}, NULL, 0, 0
                    FROM generate_series(1000, 1009) AS id;
                    INSERT INTO {schema}.submissions (
                        id, content, status, challenge_id, user_id, team_id,
                        game_id, created_at, processing_at, checked_at, pts, rank
                    )
                    SELECT id, 'flag', 'processing', 10, 2, NULL, NULL, id,
                           {PROCESSING_AT}, NULL, 0, 0
                    FROM generate_series(2000, 2009) AS id;
                "#
            ))
            .await
            .unwrap();

        let first = schema_db(&database_url, &schema).await;
        let second = schema_db(&database_url, &schema).await;
        (control, first, second, schema)
    }

    fn submission(id: i64, team_id: Option<i64>, game_id: Option<i64>) -> SubmissionView {
        SubmissionView {
            id,
            content: "flag".to_owned(),
            status: Status::Processing,
            user_id: 1,
            user_name: "User".to_owned(),
            user_avatar_hash: None,
            team_id,
            team_name: team_id.map(|_| "Team".to_owned()),
            team_avatar_hash: None,
            game_id,
            game_title: game_id.map(|_| "Game".to_owned()),
            challenge_id: 10,
            challenge_title: "Challenge".to_owned(),
            challenge_category: 0,
            created_at: 0,
            processing_at: Some(PROCESSING_AT),
            checked_at: None,
            pts: 0,
            rank: 0,
        }
    }

    fn game_submission(id: i64, team_id: i64) -> SubmissionView {
        submission(id, Some(team_id), Some(GAME_ID))
    }

    async fn scalar_i64(db: &DB, sql: &str) -> i64 {
        db.conn
            .query_one_raw(Statement::from_string(DbBackend::Postgres, sql))
            .await
            .unwrap()
            .unwrap()
            .try_get("", "value")
            .unwrap()
    }

    async fn status(db: &DB, submission_id: i64) -> String {
        db.conn
            .query_one_raw(Statement::from_string(
                DbBackend::Postgres,
                format!("SELECT status AS value FROM submissions WHERE id = {submission_id}"),
            ))
            .await
            .unwrap()
            .unwrap()
            .try_get("", "value")
            .unwrap()
    }

    async fn assert_terminal_counts(
        db: &DB,
        user_id: i64,
        team_id: Option<i64>,
        correct: i64,
        duplicate: i64,
    ) {
        let scope = match team_id {
            Some(team_id) => format!("team_id = {team_id} AND game_id = {GAME_ID}"),
            None => format!("user_id = {user_id} AND team_id IS NULL AND game_id IS NULL"),
        };
        let row = db
            .conn
            .query_one_raw(Statement::from_string(
                DbBackend::Postgres,
                format!(
                    r#"
                        SELECT
                            COUNT(*) FILTER (WHERE status = 'correct') AS correct,
                            COUNT(*) FILTER (WHERE status = 'duplicate') AS duplicate,
                            COUNT(*) FILTER (WHERE status = 'processing') AS processing
                        FROM submissions
                        WHERE challenge_id = 10 AND {scope}
                    "#
                ),
            ))
            .await
            .unwrap()
            .unwrap();
        assert_eq!(row.try_get::<i64>("", "correct").unwrap(), correct);
        assert_eq!(row.try_get::<i64>("", "duplicate").unwrap(), duplicate);
        assert_eq!(row.try_get::<i64>("", "processing").unwrap(), 0);
    }

    #[tokio::test]
    #[ignore = "requires CDS_TEST_DATABASE_URL pointing to PostgreSQL"]
    async fn finalization_is_atomic_and_scoped_on_postgres() {
        let db = test_db().await;

        let ordinary = finalize(
            &db,
            &game_submission(1, 100),
            PROCESSING_AT,
            Verdict::Incorrect,
        )
        .await
        .unwrap();
        assert_eq!(
            ordinary,
            FinalizeOutcome::Committed {
                status: Status::Incorrect,
                score_game_id: None,
            }
        );
        assert_eq!(status(&db, 1).await, "incorrect");
        assert_eq!(
            scalar_i64(&db, "SELECT COUNT(*) AS value FROM teams WHERE state = 0").await,
            0
        );
        assert_eq!(
            scalar_i64(
                &db,
                "SELECT score_revision AS value FROM games WHERE id = 7"
            )
            .await,
            0
        );

        let lease_lost = finalize(
            &db,
            &game_submission(2, 100),
            PROCESSING_AT + 1,
            Verdict::Cheat { peer_team_id: 101 },
        )
        .await
        .unwrap();
        assert_eq!(lease_lost, FinalizeOutcome::LeaseLost);
        assert_eq!(status(&db, 2).await, "processing");
        assert_eq!(
            scalar_i64(&db, "SELECT COUNT(*) AS value FROM teams WHERE state = 0").await,
            0
        );

        let cross_game = finalize(
            &db,
            &game_submission(2, 100),
            PROCESSING_AT,
            Verdict::Cheat { peer_team_id: 102 },
        )
        .await
        .unwrap();
        assert_eq!(
            cross_game,
            FinalizeOutcome::Committed {
                status: Status::Cheat,
                score_game_id: None,
            }
        );
        assert_eq!(status(&db, 2).await, "cheat");
        assert_eq!(
            scalar_i64(&db, "SELECT COUNT(*) AS value FROM teams WHERE state = 0").await,
            0
        );

        let contextless = finalize(
            &db,
            &submission(7, None, None),
            PROCESSING_AT,
            Verdict::Cheat { peer_team_id: 101 },
        )
        .await
        .unwrap();
        assert_eq!(
            contextless,
            FinalizeOutcome::Committed {
                status: Status::Incorrect,
                score_game_id: None,
            }
        );
        assert_eq!(status(&db, 7).await, "incorrect");

        let correct = finalize(
            &db,
            &game_submission(5, 100),
            PROCESSING_AT,
            Verdict::Correct,
        )
        .await
        .unwrap();
        assert_eq!(
            correct,
            FinalizeOutcome::Committed {
                status: Status::Correct,
                score_game_id: Some(GAME_ID),
            }
        );
        assert_eq!(status(&db, 5).await, "correct");
        assert_eq!(
            scalar_i64(
                &db,
                "SELECT score_revision AS value FROM games WHERE id = 7"
            )
            .await,
            1
        );

        let duplicate = finalize(
            &db,
            &game_submission(6, 100),
            PROCESSING_AT,
            Verdict::Correct,
        )
        .await
        .unwrap();
        assert_eq!(
            duplicate,
            FinalizeOutcome::Committed {
                status: Status::Duplicate,
                score_game_id: None,
            }
        );
        assert_eq!(status(&db, 6).await, "duplicate");
        assert_eq!(
            scalar_i64(
                &db,
                "SELECT score_revision AS value FROM games WHERE id = 7"
            )
            .await,
            1
        );

        let valid = finalize(
            &db,
            &game_submission(3, 100),
            PROCESSING_AT,
            Verdict::Cheat { peer_team_id: 101 },
        )
        .await
        .unwrap();
        assert_eq!(
            valid,
            FinalizeOutcome::Committed {
                status: Status::Cheat,
                score_game_id: Some(GAME_ID),
            }
        );
        assert_eq!(status(&db, 3).await, "cheat");
        assert_eq!(
            scalar_i64(
                &db,
                "SELECT COUNT(*) AS value FROM teams WHERE id IN (100, 101) AND state = 0"
            )
            .await,
            2
        );
        assert_eq!(
            scalar_i64(
                &db,
                "SELECT score_revision AS value FROM games WHERE id = 7"
            )
            .await,
            2
        );
        assert_eq!(
            scalar_i64(
                &db,
                "SELECT score_revision AS value FROM games WHERE id = 8"
            )
            .await,
            0
        );

        let error = finalize(
            &db,
            &game_submission(4, 104),
            PROCESSING_AT,
            Verdict::Cheat { peer_team_id: 105 },
        )
        .await;
        assert!(error.is_err());
        assert_eq!(status(&db, 4).await, "processing");
        assert_eq!(
            scalar_i64(
                &db,
                "SELECT COUNT(*) AS value FROM teams WHERE id IN (104, 105) AND state = 3"
            )
            .await,
            2
        );
        assert_eq!(
            scalar_i64(
                &db,
                "SELECT score_revision AS value FROM games WHERE id = 7"
            )
            .await,
            2
        );
    }

    #[tokio::test]
    #[ignore = "requires CDS_TEST_DATABASE_URL pointing to PostgreSQL"]
    async fn concurrent_correct_finalization_is_unique_across_connections() {
        let (control, first, second, schema) = concurrent_test_dbs().await;

        let barrier = Arc::new(Barrier::new(20));
        let mut tasks = Vec::new();
        for index in 0..10_i64 {
            let db = if index % 2 == 0 {
                first.clone()
            } else {
                second.clone()
            };
            let barrier = Arc::clone(&barrier);
            tasks.push(tokio::spawn(async move {
                barrier.wait().await;
                finalize(
                    &db,
                    &game_submission(1000 + index, 100),
                    PROCESSING_AT,
                    Verdict::Correct,
                )
                .await
            }));
        }
        for index in 0..10_i64 {
            let db = if index % 2 == 0 {
                second.clone()
            } else {
                first.clone()
            };
            let barrier = Arc::clone(&barrier);
            tasks.push(tokio::spawn(async move {
                let mut standalone = submission(2000 + index, None, None);
                standalone.user_id = 2;
                barrier.wait().await;
                finalize(&db, &standalone, PROCESSING_AT, Verdict::Correct).await
            }));
        }

        for task in tasks {
            assert!(matches!(
                task.await.unwrap().unwrap(),
                FinalizeOutcome::Committed { .. }
            ));
        }

        assert_terminal_counts(&first, 1, Some(100), 1, 9).await;
        assert_terminal_counts(&first, 2, None, 1, 9).await;
        assert_eq!(
            scalar_i64(
                &first,
                "SELECT score_revision AS value FROM games WHERE id = 7"
            )
            .await,
            1
        );

        first.conn.close().await.unwrap();
        second.conn.close().await.unwrap();
        control
            .execute_unprepared(&format!("DROP SCHEMA {schema} CASCADE"))
            .await
            .unwrap();
    }
}
