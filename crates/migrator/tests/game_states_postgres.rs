use cds_migrator::Migrator;
use sea_orm::{ConnectionTrait, Database, DbBackend, Statement};
use sea_orm_migration::MigratorTrait;

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
                    ('game_notices', 'created_at'), ('game_challenges', 'frozen_at'),
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
    let database_url = std::env::var("CDS_TEST_DATABASE_URL")
        .expect("CDS_TEST_DATABASE_URL must point to disposable PostgreSQL");
    let database = Database::connect(database_url).await.unwrap();

    database
        .execute_unprepared("DROP SCHEMA public CASCADE; CREATE SCHEMA public;")
        .await
        .unwrap();
    Migrator::up(&database, None).await.unwrap();
    assert_eq!(timestamp_column_types(&database).await.len(), 21);
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
}
