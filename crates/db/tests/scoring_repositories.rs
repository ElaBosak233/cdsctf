use cds_db::{game_challenge, sea_orm, submission, team};
use sea_orm::{ConnectionTrait, Database, DbBackend, Statement, TransactionTrait};

#[tokio::test]
#[ignore = "requires CDS_TEST_DATABASE_URL pointing to PostgreSQL"]
async fn scoring_and_queue_repositories_execute_on_postgres() {
    let database_url = std::env::var("CDS_TEST_DATABASE_URL")
        .expect("CDS_TEST_DATABASE_URL must be set for this ignored test");
    let database = Database::connect(database_url).await.unwrap();
    let transaction = database.begin().await.unwrap();

    transaction
        .execute_unprepared(
            r#"
                CREATE TEMP TABLE teams (
                    id BIGINT PRIMARY KEY,
                    game_id BIGINT NOT NULL,
                    state INTEGER NOT NULL
                ) ON COMMIT DROP;
                CREATE TEMP TABLE game_challenges (
                    game_id BIGINT NOT NULL,
                    challenge_id BIGINT NOT NULL,
                    difficulty BIGINT NOT NULL,
                    max_pts BIGINT NOT NULL,
                    min_pts BIGINT NOT NULL,
                    bonus_ratios BIGINT[] NOT NULL,
                    frozen_at TIMESTAMPTZ,
                    PRIMARY KEY (game_id, challenge_id)
                ) ON COMMIT DROP;
                CREATE TEMP TABLE submissions (
                    id BIGINT PRIMARY KEY,
                    content TEXT NOT NULL,
                    status TEXT NOT NULL,
                    challenge_id BIGINT NOT NULL,
                    user_id BIGINT NOT NULL,
                    team_id BIGINT,
                    game_id BIGINT,
                    created_at TIMESTAMPTZ NOT NULL,
                    processing_at TIMESTAMPTZ,
                    claims BIGINT NOT NULL DEFAULT 0,
                    checked_at TIMESTAMPTZ
                ) ON COMMIT DROP;
                CREATE UNIQUE INDEX uq_temp_submissions_correct_game
                    ON submissions (game_id, team_id, challenge_id)
                    WHERE status = 'correct' AND game_id IS NOT NULL AND team_id IS NOT NULL;

                INSERT INTO teams VALUES
                    (100, 7, 3), -- Passed
                    (101, 7, 0); -- Banned
                INSERT INTO game_challenges VALUES
                    (7, 10, 10, 1000, 100, ARRAY[10, 0]::BIGINT[], NULL);
                INSERT INTO submissions VALUES
                    (1, 'flag', 'correct', 10, 1, 100, 7, to_timestamp(1000), NULL, 1, to_timestamp(1001)),
                    (2, 'flag', 'correct', 10, 2, 101, 7, to_timestamp(900), NULL, 1, to_timestamp(901)),
                    (3, 'flag', 'queued',  10, 1, 100, 7, to_timestamp(1100), NULL, 0, NULL),
                    (4, 'flag', 'queued',  11, 1, 100, 7, to_timestamp(1101), NULL, 0, NULL);
            "#,
        )
        .await
        .unwrap();

    // Score inputs must include only Passed team 100 and exclude Banned team
    // 101.
    let inputs = submission::find_score_inputs(&transaction, 7)
        .await
        .unwrap();
    assert_eq!(inputs.len(), 1);
    assert_eq!(inputs[0].id, 1);
    assert_eq!(inputs[0].team_id, 100);

    assert_eq!(
        team::find_passed_ids(&transaction, 7).await.unwrap(),
        vec![100]
    );
    assert_eq!(
        game_challenge::find_score_inputs(&transaction, 7)
            .await
            .unwrap()
            .len(),
        1
    );

    // Claim skipping challenge 10 must claim only submission 4 (challenge 11).
    let claimed = submission::claim_batch(&transaction, 4, &[10])
        .await
        .unwrap();
    assert_eq!(claimed.len(), 1);
    assert_eq!(claimed[0].id, 4);
    assert_eq!(claimed[0].claims, 1);

    // Claiming without exclusions picks up submission 3 (challenge 10).
    let claimed_two = submission::claim_batch(&transaction, 4, &[]).await.unwrap();
    assert_eq!(claimed_two.len(), 1);
    assert_eq!(claimed_two[0].id, 3);
    assert_eq!(claimed_two[0].claims, 1);

    // Partial unique index maps duplicate `correct` to DuplicateCorrect (inside
    // a savepoint).
    let sp = transaction.begin().await.unwrap();
    let res = submission::finish_processing(&sp, 3, 1, submission::Status::Correct)
        .await
        .unwrap();
    assert_eq!(res, submission::FinishResult::DuplicateCorrect);
    sp.rollback().await.unwrap();

    // Stale claims generation is fenced out.
    assert_eq!(
        submission::finish_processing(&transaction, 3, 0, submission::Status::Duplicate)
            .await
            .unwrap(),
        submission::FinishResult::LeaseLost
    );
    assert_eq!(
        submission::finish_processing(&transaction, 3, 1, submission::Status::Duplicate)
            .await
            .unwrap(),
        submission::FinishResult::Committed
    );

    // Releasing submission 4 returns it to queued.
    assert!(
        !submission::release_processing(&transaction, 4, 0)
            .await
            .unwrap()
    );
    assert!(
        submission::release_processing(&transaction, 4, 1)
            .await
            .unwrap()
    );
    let status: String = transaction
        .query_one_raw(Statement::from_string(
            DbBackend::Postgres,
            "SELECT status AS value FROM submissions WHERE id = 4",
        ))
        .await
        .unwrap()
        .unwrap()
        .try_get("", "value")
        .unwrap();
    assert_eq!(status, "queued");

    transaction.rollback().await.unwrap();
}
