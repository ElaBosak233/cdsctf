//! Database schema migrations (SeaORM) plus a safety net for empty
//! configuration.
//!
//! [`run`] applies pending migrations when needed, then inserts default config
//! if the table is empty.

/// Defines the `migrations` submodule (see sibling `*.rs` files).
mod migrations;

use async_trait::async_trait;
use cds_db::DbError;
use sea_orm_migration::prelude::*;
use tracing::info;

/// SeaORM migrator listing all versioned migration modules in order.
pub struct Migrator;

#[async_trait]
impl MigratorTrait for Migrator {
    /// Returns every boxed migration in chronological order.
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(migrations::m20260201_000001_create_config::Migration),
            Box::new(migrations::m20260201_000002_create_user::Migration),
            Box::new(migrations::m20260201_000003_create_email::Migration),
            Box::new(migrations::m20260201_000004_create_game::Migration),
            Box::new(migrations::m20260201_000005_create_challenge::Migration),
            Box::new(migrations::m20260201_000006_create_team::Migration),
            Box::new(migrations::m20260201_000007_create_game_notice::Migration),
            Box::new(migrations::m20260201_000008_create_game_challenge::Migration),
            Box::new(migrations::m20260201_000009_create_team_user::Migration),
            Box::new(migrations::m20260201_000010_create_submission::Migration),
            Box::new(migrations::m20260201_000011_create_note::Migration),
        ]
    }
}

/// Applies migrations and ensures a default [`cds_db::config::Config`] row
/// exists.
pub async fn run(db: &cds_db::DB) -> Result<(), DbError> {
    if !Migrator::get_pending_migrations(&db.conn).await?.is_empty() {
        info!("Migration activating");
        Migrator::up(&db.conn, None).await?;
    }

    if cds_db::config::count(&db.conn).await? < 1 {
        let _ = cds_db::config::save(&db.conn, cds_db::config::Config::default()).await?;
    }

    Ok(())
}
