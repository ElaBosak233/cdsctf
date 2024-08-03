mod migration;

use bcrypt::{hash, DEFAULT_COST};
use once_cell::sync::OnceCell;
use sea_orm::{
    ActiveModelTrait, ConnectOptions, Database, DatabaseConnection, EntityTrait, PaginatorTrait,
    Set,
};
use std::time::Duration;
use tracing::info;

use crate::config;

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
        .set_schema_search_path(&config::get_config().db.schema);

    let db: DatabaseConnection = Database::connect(opt).await.unwrap();
    DB.set(db).unwrap();
    {
        // let mut db_lock = DB.write().await;
        // *db_lock = Some(db);
    }
    migration::migrate(&get_db()).await;
    info!("Database connection established successfully.");
    init_admin().await;
    init_category().await;
}

pub fn get_db() -> DatabaseConnection {
    // let db_lock = DB.read().await;
    // return db_lock.clone().expect("Database not initialized");
    return DB.get().unwrap().clone();
}

pub async fn init_admin() {
    let total = crate::model::user::Entity::find()
        .count(&get_db())
        .await
        .unwrap();
    if total == 0 {
        let hashed_password = hash("123456".to_string(), DEFAULT_COST).unwrap();
        let user = crate::model::user::ActiveModel {
            username: Set("admin".to_string()),
            nickname: Set("Administrator".to_string()),
            email: Set(Some("admin@admin.com".to_string())),
            group: Set("admin".to_string()),
            password: Set(Some(hashed_password)),
            ..Default::default()
        };
        user.insert(&get_db()).await.unwrap();
        info!("Admin user created successfully.");
    }
}

pub async fn init_category() {
    let total = crate::model::category::Entity::find()
        .count(&get_db())
        .await
        .unwrap();
    if total == 0 {
        let default_categories = vec![
            crate::model::category::ActiveModel {
                name: Set("web".to_string()),
                color: Set("#009688".to_string()),
                icon: Set("language".to_string()),
                ..Default::default()
            },
            crate::model::category::ActiveModel {
                name: Set("pwn".to_string()),
                color: Set("#673AB7".to_string()),
                icon: Set("function".to_string()),
                ..Default::default()
            },
            crate::model::category::ActiveModel {
                name: Set("crypto".to_string()),
                color: Set("#607D8B".to_string()),
                icon: Set("tag".to_string()),
                ..Default::default()
            },
            crate::model::category::ActiveModel {
                name: Set("misc".to_string()),
                color: Set("#3F51B5".to_string()),
                icon: Set("fingerprint".to_string()),
                ..Default::default()
            },
            crate::model::category::ActiveModel {
                name: Set("reverse".to_string()),
                color: Set("#6D4C41".to_string()),
                icon: Set("keyboard_double_arrow_left".to_string()),
                ..Default::default()
            },
        ];
        for categ0ry in default_categories {
            categ0ry.insert(&get_db()).await.unwrap();
        }
        info!("Default category created successfully.");
    }
}
