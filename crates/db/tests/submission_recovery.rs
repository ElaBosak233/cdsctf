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
            state INT NOT NULL, pts BIGINT NOT NULL, rank BIGINT NOT NULL
        ) ON COMMIT DROP;
        CREATE TEMP TABLE team_users (
            team_id BIGINT NOT NULL, user_id BIGINT NOT NULL,
            PRIMARY KEY (team_id, user_id)
        ) ON COMMIT DROP;
        INSERT INTO teams VALUES
            (1, 10, 'Allowed', NULL, NULL, 'team-avatar', false, 3, 0, 0),
            (2, 20, 'Other game', NULL, NULL, NULL, false, 3, 0, 0),
            (3, 10, 'Other team', NULL, NULL, NULL, false, 3, 0, 0);
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
async fn queued_recovery_pages_without_loading_relations_or_changing_claims() {
    let database = Database::connect(std::env::var("CDS_TEST_DATABASE_URL").unwrap())
        .await
        .unwrap();
    let transaction = database.begin().await.unwrap();
    transaction
        .execute_unprepared(
            r#"
        CREATE TEMP TABLE submissions (
            id BIGINT PRIMARY KEY, status TEXT NOT NULL,
            created_at TIMESTAMPTZ NOT NULL, claims BIGINT NOT NULL
        ) ON COMMIT DROP;
        INSERT INTO submissions VALUES
            (1, 'queued', to_timestamp(100), 0),
            (2, 'queued', to_timestamp(100), 9),
            (3, 'processing', to_timestamp(100), 1),
            (4, 'correct', to_timestamp(100), 1),
            (5, 'queued', to_timestamp(300), 0),
            (6, 'queued', to_timestamp(100), 0),
            (7, 'failed', to_timestamp(100), 1);
    "#,
        )
        .await
        .unwrap();
    let cutoff = time::OffsetDateTime::from_unix_timestamp(200).unwrap();
    assert_eq!(
        submission::find_queued_ids_before(&transaction, cutoff, None, 2)
            .await
            .unwrap(),
        vec![1, 2]
    );
    assert_eq!(
        submission::find_queued_ids_before(&transaction, cutoff, Some(2), 2)
            .await
            .unwrap(),
        vec![6]
    );
    assert!(
        submission::find_queued_ids_before(&transaction, cutoff, Some(6), 2)
            .await
            .unwrap()
            .is_empty()
    );
    // The same page remains available, including a legitimately requeued row
    // with historical claims. Scanning itself acquires no processing ownership.
    assert_eq!(
        submission::find_queued_ids_before(&transaction, cutoff, None, 10)
            .await
            .unwrap(),
        vec![1, 2, 6]
    );
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
            claims BIGINT NOT NULL DEFAULT 0, checked_at TIMESTAMPTZ,
            pts BIGINT NOT NULL DEFAULT 0, rank BIGINT NOT NULL DEFAULT 0
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
    assert_eq!((record.claims, record.pts, record.rank), (0, 0, 0));
    assert!(record.processing_at.is_none() && record.checked_at.is_none());
    transaction.rollback().await.unwrap();
}
