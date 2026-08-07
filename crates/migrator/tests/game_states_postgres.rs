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

#[tokio::test]
#[ignore = "requires CDS_TEST_DATABASE_URL pointing to disposable PostgreSQL"]
async fn game_state_migration_round_trips_on_postgres() {
    let database_url = std::env::var("CDS_TEST_DATABASE_URL")
        .expect("CDS_TEST_DATABASE_URL must point to disposable PostgreSQL");
    let database = Database::connect(database_url).await.unwrap();

    database
        .execute_unprepared("DROP SCHEMA public CASCADE; CREATE SCHEMA public;")
        .await
        .unwrap();
    Migrator::up(&database, None).await.unwrap();
    let columns = game_state_columns(&database).await;
    assert_eq!(columns.len(), 2);
    assert!(columns.iter().all(|(_, default, nullable)| {
        default.eq_ignore_ascii_case("false") && nullable == "NO"
    }));

    Migrator::down(&database, Some(1)).await.unwrap();
    assert!(game_state_columns(&database).await.is_empty());

    Migrator::up(&database, Some(1)).await.unwrap();
    assert_eq!(game_state_columns(&database).await.len(), 2);
}
