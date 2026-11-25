use cds_db::{sea_orm, submission};
use sea_orm::{ConnectionTrait, Database, Set, TransactionTrait};

#[tokio::test]
#[ignore = "requires CDS_TEST_DATABASE_URL pointing to PostgreSQL"]
async fn submission_team_lookup_requires_member_game_id_and_passed_state() {
    let database = Database::connect(std::env::var("CDS_TEST_DATABASE_URL").unwrap())
        .await
        .unwrap();
    let transaction = database.begin().await.unwrap();
    transaction
        .execute_unprepared(
            r#"
        CREATE TEMP TABLE teams (
            id BIGINT PRIMARY KEY, game_id BIGINT NOT NULL, name TEXT NOT NULL,
            email TEXT, slogan TEXT, avatar_hash TEXT, has_writeup BOOL NOT NULL,
            state INT NOT NULL
        ) ON COMMIT DROP;
        CREATE TEMP TABLE team_users (
            team_id BIGINT NOT NULL, user_id BIGINT NOT NULL,
            PRIMARY KEY (team_id, user_id)
        ) ON COMMIT DROP;
        INSERT INTO teams VALUES
            (1, 10, 'Allowed', NULL, NULL, 'team-avatar', false, 3),
            (2, 20, 'Other game', NULL, NULL, NULL, false, 3),
            (3, 10, 'Other team', NULL, NULL, NULL, false, 3);
        INSERT INTO team_users VALUES (1, 100), (1, 101), (2, 100), (3, 101);
    "#,
        )
        .await
        .unwrap();
    let team = cds_db::team::find_passed_member_in_game(&transaction, 1, 10, 100)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(team.name, "Allowed");
    assert_eq!(team.avatar_hash.as_deref(), Some("team-avatar"));
    for (team_id, game_id, user_id) in [(1, 10, 999), (1, 20, 100), (2, 10, 100), (3, 10, 100)] {
        assert!(
            cds_db::team::find_passed_member_in_game(&transaction, team_id, game_id, user_id)
                .await
                .unwrap()
                .is_none()
        );
    }
    for state in [0, 1, 2] {
        transaction
            .execute_unprepared(&format!("UPDATE teams SET state={state} WHERE id=1"))
            .await
            .unwrap();
        assert!(
            cds_db::team::find_passed_member_in_game(&transaction, 1, 10, 100)
                .await
                .unwrap()
                .is_none()
        );
    }
    transaction.rollback().await.unwrap();
}

#[tokio::test]
#[ignore = "requires CDS_TEST_DATABASE_URL pointing to PostgreSQL"]
async fn claim_batch_reclaims_expired_processing_leases_and_skips_active_ones() {
    let database = Database::connect(std::env::var("CDS_TEST_DATABASE_URL").unwrap())
        .await
        .unwrap();
    let transaction = database.begin().await.unwrap();
    let now = time::OffsetDateTime::now_utc().unix_timestamp();
    transaction
        .execute_unprepared(&format!(
            r#"
        CREATE TEMP TABLE submissions (
            id BIGINT PRIMARY KEY, content TEXT NOT NULL, status TEXT NOT NULL,
            challenge_id BIGINT NOT NULL, user_id BIGINT NOT NULL, team_id BIGINT,
            game_id BIGINT, created_at TIMESTAMPTZ NOT NULL, processing_at TIMESTAMPTZ,
            claims BIGINT NOT NULL DEFAULT 0, checked_at TIMESTAMPTZ
        ) ON COMMIT DROP;
        INSERT INTO submissions VALUES
            (1, 'f', 'queued',     10, 1, NULL, NULL, to_timestamp({}), NULL, 0, NULL),
            (2, 'f', 'processing', 10, 1, NULL, NULL, to_timestamp({}), to_timestamp({}), 1, NULL),
            (3, 'f', 'processing', 10, 1, NULL, NULL, to_timestamp({}), to_timestamp({}), 1, NULL),
            (4, 'f', 'correct',    10, 1, NULL, NULL, to_timestamp({}), to_timestamp({}), 1, to_timestamp({}));
    "#,
            now - 30,
            now - 25,
            now - 20, // expired (> 15s)
            now - 10,
            now - 2, // active lease
            now - 40,
            now - 40,
            now - 39
        ))
        .await
        .unwrap();

    let claimed = submission::claim_batch(&transaction, 10, &[])
        .await
        .unwrap();
    let ids: Vec<i64> = claimed.iter().map(|c| c.id).collect();
    assert_eq!(ids.len(), 2);
    assert!(ids.contains(&1));
    assert!(ids.contains(&2));
    // Reclaimed row 2 must have incremented claims from 1 -> 2.
    assert_eq!(claimed.iter().find(|c| c.id == 2).unwrap().claims, 2);

    transaction.rollback().await.unwrap();
}

#[tokio::test]
#[ignore = "requires CDS_TEST_DATABASE_URL pointing to PostgreSQL"]
async fn admission_insert_succeeds_without_any_associated_tables() {
    let database = Database::connect(std::env::var("CDS_TEST_DATABASE_URL").unwrap())
        .await
        .unwrap();
    let transaction = database.begin().await.unwrap();
    transaction
        .execute_unprepared(
            r#"
        CREATE TEMP TABLE submissions (
            id BIGINT PRIMARY KEY, content TEXT NOT NULL, status TEXT NOT NULL,
            challenge_id BIGINT NOT NULL, user_id BIGINT NOT NULL, team_id BIGINT,
            game_id BIGINT, created_at TIMESTAMPTZ NOT NULL, processing_at TIMESTAMPTZ,
            claims BIGINT NOT NULL DEFAULT 0, checked_at TIMESTAMPTZ
        ) ON COMMIT DROP;
    "#,
        )
        .await
        .unwrap();
    let record = submission::create_record(
        &transaction,
        submission::ActiveModel {
            id: Set(77),
            content: Set("flag{test}".to_owned()),
            status: Set(submission::Status::Queued),
            challenge_id: Set(10),
            user_id: Set(20),
            ..Default::default()
        },
    )
    .await
    .unwrap();
    assert_eq!(record.id, 77);
    assert_eq!(record.status, submission::Status::Queued);
    assert_eq!(record.claims, 0);
    assert!(record.processing_at.is_none() && record.checked_at.is_none());
    transaction.rollback().await.unwrap();
}

#[tokio::test]
#[ignore = "requires CDS_TEST_DATABASE_URL pointing to PostgreSQL"]
async fn cheat_precheck_uses_only_the_requested_submission_scope() {
    let database = Database::connect(std::env::var("CDS_TEST_DATABASE_URL").unwrap())
        .await
        .unwrap();
    let transaction = database.begin().await.unwrap();
    transaction
        .execute_unprepared(
            r#"
        CREATE TEMP TABLE submissions (
            id BIGINT PRIMARY KEY, status TEXT NOT NULL,
            challenge_id BIGINT NOT NULL, team_id BIGINT, game_id BIGINT
        ) ON COMMIT DROP;
        INSERT INTO submissions VALUES
            (1, 'cheat', 10, 20, 30),
            (2, 'correct', 11, 20, 30),
            (3, 'incorrect', 10, 21, 30),
            (4, 'cheat', 10, NULL, NULL);
    "#,
        )
        .await
        .unwrap();
    assert!(
        submission::has_cheat(&transaction, 10, 20, 30)
            .await
            .unwrap()
    );
    for (challenge, team, game) in [(11, 20, 30), (10, 21, 30), (10, 20, 31)] {
        assert!(
            !submission::has_cheat(&transaction, challenge, team, game)
                .await
                .unwrap()
        );
    }
    assert_eq!(
        submission::find_cheat_challenge_ids(&transaction, vec![10, 11], 20, 30)
            .await
            .unwrap(),
        vec![10]
    );
    assert!(
        submission::find_cheat_challenge_ids(&transaction, vec![10], 20, 31)
            .await
            .unwrap()
            .is_empty()
    );
    transaction
        .execute_unprepared("DELETE FROM submissions WHERE id=1")
        .await
        .unwrap();
    assert!(
        !submission::has_cheat(&transaction, 10, 20, 30)
            .await
            .unwrap()
    );
    transaction.rollback().await.unwrap();
}
