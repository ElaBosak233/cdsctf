mod migration;

use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHasher,
};
use once_cell::sync::OnceCell;
use sea_orm::{
    ActiveModelTrait, ConnectOptions, Database, DatabaseConnection, EntityTrait, PaginatorTrait,
    Set,
};
use std::time::Duration;
use tracing::info;

static DB: OnceCell<DatabaseConnection> = OnceCell::new();

pub async fn init() {
    let url = format!(
        "postgres://{}:{}@{}:{}/{}",
        crate::config::get_config().db.username,
        crate::config::get_config().db.password,
        crate::config::get_config().db.host,
        crate::config::get_config().db.port,
        crate::config::get_config().db.dbname,
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
    migration::migrate(&get_db()).await;
    info!("Database connection established successfully.");
    init_admin().await;
}

pub fn get_db() -> DatabaseConnection {
    return DB.get().unwrap().clone();
}

pub async fn init_admin() {
    let total = crate::model::user::Entity::find()
        .count(&get_db())
        .await
        .unwrap();
    if total == 0 {
        let hashed_password = Argon2::default()
            .hash_password("123456".as_bytes(), &SaltString::generate(&mut OsRng))
            .unwrap()
            .to_string();
        let user = crate::model::user::ActiveModel {
            username: Set(String::from("admin")),
            nickname: Set(String::from("Administrator")),
            email: Set(String::from("admin@admin.com")),
            group: Set(crate::model::user::group::Group::Admin),
            password: Set(hashed_password),
            ..Default::default()
        };
        user.insert(&get_db()).await.unwrap();
        info!("Admin user created successfully.");
    }
}
