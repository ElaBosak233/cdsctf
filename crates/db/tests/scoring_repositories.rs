use cds_db::{game, game_challenge, sea_orm, submission, team};
use sea_orm::{ConnectionTrait, Database, DbBackend, Statement, TransactionTrait};

#[tokio::test]
#[ignore = "requires CDS_TEST_DATABASE_URL pointing to PostgreSQL"]
async fn scoring_repository_queries_execute_on_postgres() {
    let database_url = std::env::var("CDS_TEST_DATABASE_URL")
        .expect("CDS_TEST_DATABASE_URL must be set for this ignored test");
    let database = Database::connect(database_url).await.unwrap();
    let transaction = database.begin().await.unwrap();
    transaction
        .execute_unprepared(
            r#"
                CREATE TEMP TABLE games (
                    id BIGINT PRIMARY KEY,
                    score_revision BIGINT NOT NULL DEFAULT 0
                ) ON COMMIT DROP;
                CREATE TEMP TABLE submissions (
                    id BIGINT PRIMARY KEY,
                    challenge_id BIGINT NOT NULL,
                    team_id BIGINT,
                    game_id BIGINT,
                    created_at BIGINT NOT NULL,
                    status TEXT NOT NULL,
                    processing_at BIGINT,
                    checked_at BIGINT,
                    pts BIGINT NOT NULL,
                    rank BIGINT NOT NULL
                ) ON COMMIT DROP;
                CREATE TEMP TABLE game_challenges (
                    game_id BIGINT NOT NULL,
                    challenge_id BIGINT NOT NULL,
                    difficulty BIGINT NOT NULL,
                    max_pts BIGINT NOT NULL,
                    min_pts BIGINT NOT NULL,
                    bonus_ratios BIGINT[] NOT NULL,
                    pts BIGINT NOT NULL,
                    PRIMARY KEY (game_id, challenge_id)
                ) ON COMMIT DROP;
                CREATE TEMP TABLE teams (
                    id BIGINT PRIMARY KEY,
                    game_id BIGINT NOT NULL,
                    state INTEGER NOT NULL,
                    pts BIGINT NOT NULL,
                    rank BIGINT NOT NULL
                ) ON COMMIT DROP;

                INSERT INTO games (id) VALUES (7), (8);
                INSERT INTO submissions (
                    id, challenge_id, team_id, game_id, created_at, status, pts, rank
                ) VALUES (1, 10, 100, 7, 1000, 'correct', 0, 0);
                INSERT INTO game_challenges VALUES
                    (7, 10, 10, 1000, 100, ARRAY[10, 0]::BIGINT[], 0),
                    (8, 10, 10, 1000, 100, ARRAY[10, 0]::BIGINT[], 0);
                INSERT INTO teams VALUES (100, 7, 3, 0, 0);
            "#,
        )
        .await
        .unwrap();
    let game_id = 7;

    assert!(
        game::find_dirty_score_ids(&transaction)
            .await
            .unwrap()
            .is_empty()
    );
    game::request_score_recalculation(&transaction, game_id)
        .await
        .unwrap();
    assert_eq!(
        game::find_score_revision(&transaction, game_id)
            .await
            .unwrap(),
        Some(1)
    );
    assert_eq!(
        game::find_dirty_score_ids(&transaction).await.unwrap(),
        vec![game_id]
    );
    assert!(
        !game::mark_score_recalculation_applied(&transaction, game_id, 0)
            .await
            .unwrap()
    );
    assert!(
        game::mark_score_recalculation_applied(&transaction, game_id, 1)
            .await
            .unwrap()
    );
    assert!(
        game::find_dirty_score_ids(&transaction)
            .await
            .unwrap()
            .is_empty()
    );
    let savepoint = transaction.begin().await.unwrap();
    game::request_score_recalculation(&savepoint, game_id)
        .await
        .unwrap();
    savepoint.rollback().await.unwrap();
    assert_eq!(
        game::find_score_revision(&transaction, game_id)
            .await
            .unwrap(),
        Some(0)
    );

    let now = time::OffsetDateTime::now_utc().unix_timestamp();
    transaction
        .execute_unprepared(&format!(
            r#"
                INSERT INTO submissions (
                    id, challenge_id, created_at, status, processing_at, pts, rank
                ) VALUES
                    (2, 10, 1001, 'processing', {}, 0, 0),
                    (3, 10, 1002, 'processing', {}, 0, 0),
                    (4, 10, 1003, 'processing', NULL, 0, 0);
            "#,
            now - 20,
            now
        ))
        .await
        .unwrap();

    assert_eq!(
        submission::reset_stale_processing(&transaction)
            .await
            .unwrap(),
        2
    );
    let still_processing = transaction
        .query_one_raw(Statement::from_string(
            DbBackend::Postgres,
            "SELECT COUNT(*) AS count FROM submissions WHERE status = 'processing'",
        ))
        .await
        .unwrap()
        .unwrap()
        .try_get::<i64>("", "count")
        .unwrap();
    assert_eq!(still_processing, 1);
    assert!(
        !submission::release_processing(&transaction, 3, now - 1)
            .await
            .unwrap()
    );
    assert!(
        submission::release_processing(&transaction, 3, now)
            .await
            .unwrap()
    );

    game::lock_score_recalculation(&transaction, game_id)
        .await
        .unwrap();
    assert_eq!(game::find_ids(&transaction).await.unwrap(), vec![7, 8]);
    assert_eq!(
        submission::find_score_inputs(&transaction, game_id)
            .await
            .unwrap()
            .len(),
        1
    );
    assert_eq!(
        game_challenge::find_score_inputs(&transaction, game_id)
            .await
            .unwrap()
            .len(),
        1
    );
    assert_eq!(
        team::find_score_inputs(&transaction, game_id)
            .await
            .unwrap()
            .len(),
        1
    );

    assert_eq!(
        submission::update_scores(
            &transaction,
            game_id,
            &[submission::ScoreUpdate {
                id: 1,
                pts: 100,
                rank: 1,
            }],
        )
        .await
        .unwrap(),
        1
    );
    assert_eq!(
        game_challenge::update_scores(
            &transaction,
            game_id,
            &[game_challenge::ScoreUpdate {
                challenge_id: 10,
                pts: 100,
            }],
        )
        .await
        .unwrap(),
        1
    );
    assert_eq!(
        team::update_scores(
            &transaction,
            game_id,
            &[team::ScoreUpdate {
                id: 100,
                pts: 100,
                rank: 1,
            }],
        )
        .await
        .unwrap(),
        1
    );

    let submissions = submission::find_score_inputs(&transaction, game_id)
        .await
        .unwrap();
    let challenges = game_challenge::find_score_inputs(&transaction, game_id)
        .await
        .unwrap();
    let teams = team::find_score_inputs(&transaction, game_id)
        .await
        .unwrap();
    assert_eq!((submissions[0].pts, submissions[0].rank), (100, 1));
    assert_eq!(challenges[0].pts, 100);
    assert_eq!((teams[0].pts, teams[0].rank), (100, 1));
    assert_eq!(
        game_challenge::find_score_inputs(&transaction, 8)
            .await
            .unwrap()[0]
            .pts,
        0
    );
    assert_eq!(
        submission::update_scores(
            &transaction,
            game_id,
            &[submission::ScoreUpdate {
                id: 1,
                pts: 100,
                rank: 1,
            }],
        )
        .await
        .unwrap(),
        0
    );

    transaction.rollback().await.unwrap();
}
