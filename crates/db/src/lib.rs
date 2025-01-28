pub mod entity;
pub mod transfer;
pub mod util;

use std::time::Duration;

use once_cell::sync::OnceCell;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use tracing::info;

static DB: OnceCell<DatabaseConnection> = OnceCell::new();

pub async fn init() {
    let url = format!(
        "postgres://{}:{}@{}:{}/{}",
        cds_config::get_config().db.username,
        cds_config::get_config().db.password,
        cds_config::get_config().db.host,
        cds_config::get_config().db.port,
        cds_config::get_config().db.dbname,
    );
    let mut opt = ConnectOptions::new(url);
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(false)
        .set_schema_search_path("public");

    let db: DatabaseConnection = Database::connect(opt).await.unwrap();
    DB.set(db).unwrap();
    info!("Database connection established successfully.");
}

pub fn get_db() -> &'static DatabaseConnection {
    DB.get().unwrap()
}
