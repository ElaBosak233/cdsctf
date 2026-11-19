use cds_db::submission;
use cds_migrator::Migrator;
use sea_orm::{ConnectionTrait, Database, DbBackend, Statement, TransactionTrait};
use sea_orm_migration::{MigratorTrait, SchemaManager};

#[tokio::test]
#[ignore = "requires CDS_TEST_DATABASE_URL pointing to disposable PostgreSQL"]
async fn cheat_index_upgrade_supports_generic_plans_and_rolls_back() {
    let database = Database::connect(std::env::var("CDS_TEST_DATABASE_URL").unwrap())
        .await
        .unwrap();
    let transaction = database.begin().await.unwrap();
    transaction
        .execute_unprepared(
            "CREATE TEMP TABLE submissions (
                id BIGINT PRIMARY KEY, status TEXT NOT NULL,
                challenge_id BIGINT NOT NULL, team_id BIGINT, game_id BIGINT,
                created_at TIMESTAMPTZ NOT NULL DEFAULT now()
             ) ON COMMIT DROP;
             INSERT INTO submissions (id,status,challenge_id,team_id,game_id)
                SELECT n,'incorrect',11,21,31 FROM generate_series(1,20000) n;
             INSERT INTO submissions (id,status,challenge_id,team_id,game_id)
                VALUES (20001,'cheat',10,20,30), (20002,'correct',10,20,30);
             CREATE INDEX idx_submissions_scoring
                ON submissions (game_id,challenge_id,created_at,id)
                WHERE status='correct';
             ANALYZE submissions;
             SET LOCAL plan_cache_mode=force_generic_plan;
             PREPARE cheat_index_scope(bigint,bigint,bigint,bigint) AS
                SELECT id FROM submissions
                WHERE challenge_id=$1 AND team_id=$2 AND game_id=$3
                AND status='cheat' LIMIT $4;
             PREPARE correct_index_scope(bigint,bigint,bigint,bigint) AS
                SELECT id FROM submissions
                WHERE challenge_id=$1 AND team_id=$2 AND game_id=$3
                AND status='correct' AND id<>$4 LIMIT 1;",
        )
        .await
        .unwrap();
    let migrations = Migrator::migrations();
    let migration = migrations
        .iter()
        .find(|migration| migration.name() == "m20261119_000001_submission_cheat_index")
        .unwrap();
    let manager = SchemaManager::new(&transaction);
    migration.up(&manager).await.unwrap();

    for (query, index) in [
        (
            "EXPLAIN EXECUTE cheat_index_scope(10,20,30,1)",
            "idx_submissions_cheat",
        ),
        (
            "EXPLAIN EXECUTE correct_index_scope(10,20,30,-1)",
            "idx_submissions_scoring",
        ),
    ] {
        let rows = transaction
            .query_all_raw(Statement::from_string(DbBackend::Postgres, query))
            .await
            .unwrap();
        let plan = rows
            .into_iter()
            .map(|row| row.try_get::<String>("", "QUERY PLAN").unwrap())
            .collect::<Vec<_>>()
            .join("\n");
        assert!(plan.contains(index), "{plan}");
        assert!(!plan.contains("Seq Scan"), "{plan}");
    }
    assert!(
        submission::has_cheat(&transaction, 10, 20, 30)
            .await
            .unwrap()
    );
    assert!(
        !submission::has_cheat(&transaction, 10, 20, 31)
            .await
            .unwrap()
    );
    transaction
        .execute_unprepared("UPDATE submissions SET status='incorrect' WHERE id=20001")
        .await
        .unwrap();
    assert!(
        !submission::has_cheat(&transaction, 10, 20, 30)
            .await
            .unwrap()
    );

    migration.down(&manager).await.unwrap();
    let row = transaction
        .query_one_raw(Statement::from_string(
            DbBackend::Postgres,
            "SELECT to_regclass('pg_temp.idx_submissions_cheat') IS NULL AS removed",
        ))
        .await
        .unwrap()
        .unwrap();
    assert!(row.try_get::<bool>("", "removed").unwrap());
    transaction.rollback().await.unwrap();
}
