use cds_db::{challenge, game, sea_orm};
use sea_orm::{ConnectionTrait, Database, FromQueryResult, TransactionTrait};

#[derive(Debug, FromQueryResult)]
struct RecordId {
    id: i64,
}

#[tokio::test]
#[ignore = "requires CDS_TEST_DATABASE_URL pointing to PostgreSQL"]
async fn title_search_matches_literal_text_before_filters_and_pagination() {
    let database = Database::connect(std::env::var("CDS_TEST_DATABASE_URL").unwrap())
        .await
        .unwrap();
    let transaction = database.begin().await.unwrap();
    transaction
        .execute_unprepared(
            r#"
            CREATE TEMP TABLE games ON COMMIT DROP AS
            SELECT id::bigint, title, enabled, NULL::text AS sketch,
                   NULL::text AS description, true AS public, false AS paused,
                   false AS blacked_out, 1::bigint AS member_limit_min,
                   4::bigint AS member_limit_max, false AS writeup_required,
                   '[]'::jsonb AS timeslots, now() AS started_at, now() AS frozen_at,
                   now() AS ended_at, NULL::text AS icon_hash, NULL::text AS poster_hash,
                   0::bigint AS score_revision, now() AS created_at
            FROM (VALUES (1, 'Summer %_ Cup', true), (2, 'SUMMER %_ Final', true),
                         (3, 'Summer ABC Cup', true), (4, 'Summer %_ Disabled', false))
                AS fixtures(id, title, enabled);

            CREATE TEMP TABLE challenges ON COMMIT DROP AS
            SELECT id::bigint, title, category, public, deleted_at, ''::text AS description,
                   ARRAY[]::text[] AS tags, false AS has_instance, false AS has_attachment,
                   false AS has_writeup, NULL::jsonb AS instance, NULL::text AS checker,
                   NULL::text AS writeup, now() AS created_at, now() AS updated_at
            FROM (VALUES (1, 'WEB %_ Warmup', 2, false, NULL::timestamptz),
                         (2, 'Web %_ Public', 2, true, NULL::timestamptz),
                         (3, 'Web %_ Crypto', 3, false, NULL::timestamptz),
                         (4, 'Web %_ Deleted', 2, false, now()),
                         (5, 'Web ABC Warmup', 2, false, NULL::timestamptz))
                AS fixtures(id, title, category, public, deleted_at);
            "#,
        )
        .await
        .unwrap();

    let (games, total) = game::find::<RecordId>(
        &transaction,
        game::FindGameOptions {
            keyword: Some("SUMMER %_".to_owned()),
            enabled: Some(true),
            sorts: Some("started_at".to_owned()),
            page: Some(2),
            size: Some(1),
            ..Default::default()
        },
    )
    .await
    .unwrap();
    assert_eq!(total, 2);
    assert_eq!(games.iter().map(|game| game.id).collect::<Vec<_>>(), [2]);

    let (challenges, total) = challenge::find::<RecordId>(
        &transaction,
        challenge::FindChallengeOptions {
            keyword: Some("web %_".to_owned()),
            category: Some(2),
            public: Some(false),
            ..Default::default()
        },
    )
    .await
    .unwrap();
    assert_eq!(total, 1);
    assert_eq!(
        challenges
            .iter()
            .map(|challenge| challenge.id)
            .collect::<Vec<_>>(),
        [1]
    );

    let (challenges, total) = challenge::find::<RecordId>(
        &transaction,
        challenge::FindChallengeOptions {
            id: Some(4),
            ..Default::default()
        },
    )
    .await
    .unwrap();
    assert!(challenges.is_empty());
    assert_eq!(total, 0);
    transaction.rollback().await.unwrap();
}
