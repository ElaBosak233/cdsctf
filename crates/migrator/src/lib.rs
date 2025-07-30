mod migrations;

use async_trait::async_trait;
use cds_db::get_db;
use sea_orm_migration::prelude::*;
use tracing::info;

pub struct Migrator;

#[async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(migrations::m20250501_000001_create_config::Migration),
            Box::new(migrations::m20250501_000002_create_user::Migration),
            Box::new(migrations::m20250501_000003_create_game::Migration),
            Box::new(migrations::m20250501_000004_create_challenge::Migration),
            Box::new(migrations::m20250501_000005_create_team::Migration),
            Box::new(migrations::m20250501_000006_create_game_notice::Migration),
            Box::new(migrations::m20250501_000007_create_game_challenge::Migration),
            Box::new(migrations::m20250501_000008_create_team_user::Migration),
            Box::new(migrations::m20250501_000009_create_submission::Migration),
        ]
    }
}

pub async fn run() -> Result<(), DbErr> {
    if !Migrator::get_pending_migrations(get_db()).await?.is_empty() {
        info!("Migration activating");
        Migrator::up(get_db(), None).await?;
    }

    if cds_db::config::count().await? < 1 {
        cds_db::config::save(cds_db::config::ActiveModel {
            id: sea_orm::ActiveValue::Set(1),
            meta: sea_orm::ActiveValue::Set(cds_db::config::meta::Config::default()),
            auth: sea_orm::ActiveValue::Set(cds_db::config::auth::Config::default()),
            email: sea_orm::ActiveValue::Set(cds_db::config::email::Config::default()),
            captcha: sea_orm::ActiveValue::Set(cds_db::config::captcha::Config::default()),
        })
        .await?;
    }

    Ok(())
}
