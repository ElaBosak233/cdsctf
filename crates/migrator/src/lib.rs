mod migrations;

use async_trait::async_trait;
use cds_db::DbError;
use sea_orm_migration::prelude::*;
use tracing::info;

pub struct Migrator;

#[async_trait]
impl MigratorTrait for Migrator {
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
        ]
    }
}

pub async fn run(db: &cds_db::DB) -> Result<(), DbError> {
    if !Migrator::get_pending_migrations(&db.conn).await?.is_empty() {
        info!("Migration activating");
        Migrator::up(&db.conn, None).await?;
    }

    if cds_db::config::count(&db.conn).await? < 1 {
        let _ = cds_db::config::save(
            &db.conn,
            cds_db::config::ActiveModel {
                id: sea_orm::ActiveValue::Set(1),
                meta: sea_orm::ActiveValue::Set(cds_db::config::meta::Config::default()),
                auth: sea_orm::ActiveValue::Set(cds_db::config::auth::Config::default()),
                email: sea_orm::ActiveValue::Set(cds_db::config::email::Config::default()),
                captcha: sea_orm::ActiveValue::Set(cds_db::config::captcha::Config::default()),
            },
        )
        .await?;
    }

    Ok(())
}
