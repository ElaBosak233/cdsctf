mod migration;

use std::time::Duration;

use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHasher,
};
use once_cell::sync::OnceCell;
use sea_orm::{
    ActiveModelTrait, ConnectOptions, Database, DatabaseConnection, EntityTrait, PaginatorTrait,
    Set,
};
use tracing::info;

static DB: OnceCell<DatabaseConnection> = OnceCell::new();

pub async fn init() {
    let url = format!(
        "postgres://{}:{}@{}:{}/{}",
        crate::env::get_env().db.username,
        crate::env::get_env().db.password,
        crate::env::get_env().db.host,
        crate::env::get_env().db.port,
        crate::env::get_env().db.dbname,
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
    init_config().await;
}

pub fn get_db() -> DatabaseConnection {
    DB.get().unwrap().clone()
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

pub async fn init_config() {
    let total = crate::model::config::Entity::find()
        .count(&get_db())
        .await
        .unwrap();
    if total == 0 {
        let config = crate::model::config::ActiveModel {
            auth: Set(crate::config::auth::Config {
                jwt: crate::config::auth::jwt::Config {
                    secret_key: String::from("123456"),
                    expiration: 1800,
                },
                registration: crate::config::auth::registration::Config {
                    enabled: true,
                    captcha: false,
                    email: crate::config::auth::registration::email::Config {
                        enabled: false,
                        domains: vec![],
                    },
                },
            }),
            cluster: Set(crate::config::cluster::Config {
                namespace: String::from("default"),
                entry: String::from("127.0.0.1"),
                proxy: crate::config::cluster::proxy::Config {
                    enabled: true,
                    traffic_capture: false,
                },
                strategy: crate::config::cluster::strategy::Config {
                    parallel_limit: 0,
                    request_limit: 0,
                },
            }),
            site: Set(crate::config::site::Config {
                title: String::from("CdsCTF"),
                description: String::from("CdsCTF is a CTF platform."),
                color: String::from("#0C4497"),
                favicon: String::from(""),
            }),
            ..Default::default()
        };
        config.insert(&get_db()).await.unwrap();
        info!("Default configuration created successfully.");
    }
}
