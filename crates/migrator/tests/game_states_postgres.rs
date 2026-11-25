use cds_migrator::Migrator;
use sea_orm::{ConnectionTrait, Database, DbBackend, Statement};
use sea_orm_migration::MigratorTrait;

async fn isolated_database(prefix: &str) -> (sea_orm::DatabaseConnection, String) {
    let mut options = sea_orm::ConnectOptions::new(
        std::env::var("CDS_TEST_DATABASE_URL").expect("requires disposable PostgreSQL"),
    );
    options.max_connections(1).min_connections(1);
    let database = Database::connect(options).await.unwrap();
    // Both migrations need this database-wide extension. Install it once in
    // public, outside the per-test schemas that are removed after each test.
    static EXTENSION: tokio::sync::OnceCell<()> = tokio::sync::OnceCell::const_new();
    EXTENSION
        .get_or_init(|| async {
            database
                .execute_unprepared("CREATE EXTENSION IF NOT EXISTS pg_trgm WITH SCHEMA public")
                .await
                .unwrap();
        })
        .await;
    let schema = format!("{prefix}_{}", std::process::id());
    database
        .execute_unprepared(&format!(
            "CREATE SCHEMA {schema}; SET search_path TO {schema}"
        ))
        .await
        .unwrap();
    Migrator::install(&database).await.unwrap();
    database
        .execute_unprepared(&format!("SET search_path TO {schema}, public"))
        .await
        .unwrap();
    (database, schema)
}

async fn game_state_columns(
    database: &sea_orm::DatabaseConnection,
) -> Vec<(String, String, String)> {
    database
        .query_all_raw(Statement::from_string(
            DbBackend::Postgres,
            r#"
                SELECT column_name, column_default, is_nullable
                FROM information_schema.columns
                WHERE table_schema = current_schema()
                  AND table_name = 'games'
                  AND column_name IN ('paused', 'blacked_out')
                ORDER BY column_name
            "#,
        ))
        .await
        .unwrap()
        .into_iter()
        .map(|row| {
            (
                row.try_get("", "column_name").unwrap(),
                row.try_get("", "column_default").unwrap(),
                row.try_get("", "is_nullable").unwrap(),
            )
        })
        .collect()
}

async fn idp_state_columns(
    database: &sea_orm::DatabaseConnection,
) -> Vec<(String, String, String, String, Option<String>)> {
    database
        .query_all_raw(Statement::from_string(
            DbBackend::Postgres,
            r#"
                SELECT table_name, column_name, data_type, is_nullable, column_default
                FROM information_schema.columns
                WHERE table_schema = current_schema()
                  AND (table_name, column_name) IN (
                    ('idps', 'registration_enabled'),
                    ('user_idps', 'source')
                  )
                ORDER BY table_name, column_name
            "#,
        ))
        .await
        .unwrap()
        .into_iter()
        .map(|row| {
            (
                row.try_get("", "table_name").unwrap(),
                row.try_get("", "column_name").unwrap(),
                row.try_get("", "data_type").unwrap(),
                row.try_get("", "is_nullable").unwrap(),
                row.try_get("", "column_default").unwrap(),
            )
        })
        .collect()
}

async fn timestamp_column_types(database: &sea_orm::DatabaseConnection) -> Vec<(String, String)> {
    database
        .query_all_raw(Statement::from_string(
            DbBackend::Postgres,
            r#"
                SELECT table_name, column_name
                FROM information_schema.columns
                WHERE table_schema = current_schema()
                  AND data_type = 'timestamp with time zone'
                  AND (table_name, column_name) IN (
                    ('users', 'deleted_at'), ('users', 'created_at'), ('users', 'updated_at'),
                    ('games', 'started_at'), ('games', 'frozen_at'), ('games', 'ended_at'), ('games', 'created_at'),
                    ('challenges', 'deleted_at'), ('challenges', 'created_at'), ('challenges', 'updated_at'),
                    ('game_notices', 'created_at'), ('game_notices', 'updated_at'), ('game_challenges', 'frozen_at'),
                    ('submissions', 'created_at'), ('submissions', 'processing_at'), ('submissions', 'checked_at'),
                    ('notes', 'created_at'), ('notes', 'updated_at'),
                    ('idps', 'created_at'), ('idps', 'updated_at'),
                    ('user_idps', 'created_at'), ('user_idps', 'updated_at')
                  )
                ORDER BY table_name, column_name
            "#,
        ))
        .await
        .unwrap()
        .into_iter()
        .map(|row| {
            (
                row.try_get("", "table_name").unwrap(),
                row.try_get("", "column_name").unwrap(),
            )
        })
        .collect()
}

async fn user_idp_check_constraint_count(database: &sea_orm::DatabaseConnection) -> i64 {
    database
        .query_one_raw(Statement::from_string(
            DbBackend::Postgres,
            r#"
                SELECT COUNT(*) AS count
                FROM pg_constraint
                WHERE conrelid = 'user_idps'::regclass
                  AND contype = 'c'
            "#,
        ))
        .await
        .unwrap()
        .unwrap()
        .try_get("", "count")
        .unwrap()
}

#[tokio::test]
#[ignore = "requires CDS_TEST_DATABASE_URL pointing to disposable PostgreSQL"]
async fn initial_state_columns_are_created_on_postgres() {
    let (database, schema) = isolated_database("initial_states").await;
    Migrator::up(&database, None).await.unwrap();
    assert_eq!(timestamp_column_types(&database).await.len(), 22);
    let columns = game_state_columns(&database).await;
    assert_eq!(columns.len(), 2);
    assert!(columns.iter().all(|(_, default, nullable)| {
        default.eq_ignore_ascii_case("false") && nullable == "NO"
    }));

    let idp_columns = idp_state_columns(&database).await;
    assert_eq!(
        idp_columns,
        vec![
            (
                "idps".to_string(),
                "registration_enabled".to_string(),
                "boolean".to_string(),
                "NO".to_string(),
                Some("false".to_string()),
            ),
            (
                "user_idps".to_string(),
                "source".to_string(),
                "character varying".to_string(),
                "NO".to_string(),
                None,
            ),
        ]
    );
    assert_eq!(user_idp_check_constraint_count(&database).await, 0);
    database
        .execute_unprepared(&format!(
            "SET search_path TO public; DROP SCHEMA {schema} CASCADE"
        ))
        .await
        .unwrap();
}

#[tokio::test]
#[ignore = "requires CDS_TEST_DATABASE_URL pointing to disposable PostgreSQL"]
async fn submission_failure_constraint_supports_upgrade_and_rollback() {
    let (database, schema) = isolated_database("submission_failure").await;
    Migrator::up(&database, None).await.unwrap();
    let definition = |database: sea_orm::DatabaseConnection| async move {
        database.query_one_raw(Statement::from_string(DbBackend::Postgres,
            "SELECT pg_get_constraintdef(oid) AS definition FROM pg_constraint WHERE conrelid = 'submissions'::regclass AND conname = 'chk_submissions_status'"
        )).await.unwrap().unwrap().try_get::<String>("", "definition").unwrap()
    };
    assert!(definition(database.clone()).await.contains("failed"));
    Migrator::down(&database, Some(4)).await.unwrap();
    assert!(!definition(database.clone()).await.contains("failed"));
    Migrator::up(&database, None).await.unwrap();
    assert!(definition(database.clone()).await.contains("failed"));
    database
        .execute_unprepared(&format!(
            "SET search_path TO public; DROP SCHEMA {schema} CASCADE"
        ))
        .await
        .unwrap();
}

#[tokio::test]
#[ignore = "requires CDS_TEST_DATABASE_URL pointing to disposable PostgreSQL"]
async fn cqrs_scoring_and_queue_migration_upgrades_and_rolls_back() {
    let (database, schema) = isolated_database("cqrs_scoring_queue").await;
    Migrator::up(&database, None).await.unwrap();

    // Derived score columns and games.score_revision must be removed.
    let remaining: i64 = database
        .query_one_raw(Statement::from_string(
            DbBackend::Postgres,
            r#"
                SELECT COUNT(*) AS count
                FROM information_schema.columns
                WHERE table_schema = current_schema()
                  AND (
                    (table_name = 'games' AND column_name = 'score_revision')
                    OR (table_name = 'game_challenges' AND column_name = 'pts')
                    OR (table_name = 'teams' AND column_name IN ('pts', 'rank'))
                    OR (table_name = 'submissions' AND column_name IN ('pts', 'rank'))
                  )
            "#,
        ))
        .await
        .unwrap()
        .unwrap()
        .try_get("", "count")
        .unwrap();
    assert_eq!(remaining, 0);

    // New indexes must exist.
    let indexes: i64 = database
        .query_one_raw(Statement::from_string(
            DbBackend::Postgres,
            r#"
                SELECT COUNT(*) AS count
                FROM pg_indexes
                WHERE schemaname = current_schema()
                  AND indexname IN (
                    'idx_submissions_work_queue',
                    'uq_submissions_correct_game',
                    'uq_submissions_correct_standalone',
                    'idx_submissions_scoring',
                    'idx_teams_game_state'
                  )
            "#,
        ))
        .await
        .unwrap()
        .unwrap()
        .try_get("", "count")
        .unwrap();
    assert_eq!(indexes, 5);

    // Rollback restores legacy columns and drops new indexes cleanly.
    Migrator::down(&database, Some(2)).await.unwrap();
    let restored: i64 = database
        .query_one_raw(Statement::from_string(
            DbBackend::Postgres,
            r#"
                SELECT COUNT(*) AS count
                FROM information_schema.columns
                WHERE table_schema = current_schema()
                  AND (
                    (table_name = 'games' AND column_name = 'score_revision')
                    OR (table_name = 'game_challenges' AND column_name = 'pts')
                    OR (table_name = 'teams' AND column_name IN ('pts', 'rank'))
                    OR (table_name = 'submissions' AND column_name IN ('pts', 'rank'))
                  )
            "#,
        ))
        .await
        .unwrap()
        .unwrap()
        .try_get("", "count")
        .unwrap();
    assert_eq!(restored, 6);

    Migrator::up(&database, None).await.unwrap();
    database
        .execute_unprepared(&format!(
            "SET search_path TO public; DROP SCHEMA {schema} CASCADE"
        ))
        .await
        .unwrap();
}

#[tokio::test]
#[ignore = "requires CDS_TEST_DATABASE_URL pointing to disposable PostgreSQL"]
async fn game_notice_refactor_migration_upgrades_cascades_and_rolls_back() {
    let (database, schema) = isolated_database("game_notice_refactor").await;
    Migrator::up(&database, None).await.unwrap();

    // Seed one user, one game, one challenge, one game_challenge, and two
    // notices (one general, one bound to the game_challenge).
    database
        .execute_unprepared(
            r#"
                INSERT INTO users (id, name, username, banned, permissions, hashed_password, created_at, updated_at)
                VALUES (1, 'Admin', 'admin', FALSE, ARRAY[]::TEXT[], 'hash', NOW(), NOW());

                INSERT INTO games (id, title, enabled, public, paused, blacked_out, writeup_required, member_limit_min, member_limit_max, timeslots, started_at, frozen_at, ended_at, created_at)
                VALUES (1, 'Test Game', TRUE, TRUE, FALSE, FALSE, FALSE, 1, 3, '[]'::JSONB, NOW(), NOW(), NOW(), NOW());

                INSERT INTO challenges (id, title, description, category, tags, has_instance, has_attachment, has_writeup, public, created_at, updated_at)
                VALUES (10, 'Web 1', 'desc', 1, ARRAY[]::TEXT[], FALSE, FALSE, FALSE, FALSE, NOW(), NOW());

                INSERT INTO game_challenges (game_id, challenge_id, difficulty, max_pts, min_pts, bonus_ratios, enabled)
                VALUES (1, 10, 5, 1000, 100, ARRAY[10, 5]::BIGINT[], TRUE);

                INSERT INTO game_notices (game_id, challenge_id, title, content, created_by, updated_by, created_at, updated_at)
                VALUES
                    (1, NULL, 'General', 'general body', 1, 1, NOW(), NOW()),
                    (1, 10, 'Challenge Hint', 'hint body', 1, 1, NOW(), NOW());
            "#,
        )
        .await
        .unwrap();

    // Removing the challenge from the game cascades to its bound notice only.
    database
        .execute_unprepared("DELETE FROM game_challenges WHERE game_id = 1 AND challenge_id = 10")
        .await
        .unwrap();

    let remaining_notices: i64 = database
        .query_one_raw(Statement::from_string(
            DbBackend::Postgres,
            "SELECT COUNT(*) AS count FROM game_notices WHERE game_id = 1",
        ))
        .await
        .unwrap()
        .unwrap()
        .try_get("", "count")
        .unwrap();
    assert_eq!(remaining_notices, 1);

    // Deleting the admin user sets created_by / updated_by to NULL cleanly.
    database
        .execute_unprepared("DELETE FROM users WHERE id = 1")
        .await
        .unwrap();
    let null_authors: i64 = database
        .query_one_raw(Statement::from_string(
            DbBackend::Postgres,
            "SELECT COUNT(*) AS count FROM game_notices WHERE game_id = 1 AND created_by IS NULL AND updated_by IS NULL",
        ))
        .await
        .unwrap()
        .unwrap()
        .try_get("", "count")
        .unwrap();
    assert_eq!(null_authors, 1);

    // Rollback drops the new columns and re-upgrade restores them cleanly.
    Migrator::down(&database, Some(1)).await.unwrap();
    let remaining_cols: i64 = database
        .query_one_raw(Statement::from_string(
            DbBackend::Postgres,
            r#"
                SELECT COUNT(*) AS count
                FROM information_schema.columns
                WHERE table_schema = current_schema()
                  AND table_name = 'game_notices'
                  AND column_name IN ('challenge_id', 'created_by', 'updated_by', 'updated_at')
            "#,
        ))
        .await
        .unwrap()
        .unwrap()
        .try_get("", "count")
        .unwrap();
    assert_eq!(remaining_cols, 0);

    Migrator::up(&database, None).await.unwrap();
    database
        .execute_unprepared(&format!(
            "SET search_path TO public; DROP SCHEMA {schema} CASCADE"
        ))
        .await
        .unwrap();
}
