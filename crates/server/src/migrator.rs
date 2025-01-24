use argon2::{
    Argon2, PasswordHasher,
    password_hash::{SaltString, rand_core::OsRng},
};
use cds_db::{entity, get_db};
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ConnectionTrait, DbConn, EntityTrait, PaginatorTrait,
    Schema, sqlx::types::uuid,
};
use tracing::{error, info};

macro_rules! create_tables {
    ($db:expr, $($entity:expr),*) => {
        $(
            create_table($db, $entity).await;
        )*
    };
}

async fn create_table<E>(db: &DbConn, entity: E)
where
    E: EntityTrait, {
    let builder = db.get_database_backend();
    let schema = Schema::new(builder);
    let stmt = builder.build(schema.create_table_from_entity(entity).if_not_exists());

    if let Err(e) = db.execute(stmt).await {
        error!("Error: {}", e)
    }
}

pub async fn run() {
    create_tables!(
        get_db(),
        entity::config::Entity,
        entity::user::Entity,
        entity::team::Entity,
        entity::user_team::Entity,
        entity::challenge::Entity,
        entity::game::Entity,
        entity::submission::Entity,
        entity::pod::Entity,
        entity::game_challenge::Entity,
        entity::game_team::Entity
    );

    init_admin().await;
    init_config().await;
}

pub async fn init_admin() {
    let total = entity::user::Entity::find().count(get_db()).await.unwrap();
    if total == 0 {
        let hashed_password = Argon2::default()
            .hash_password("123456".as_bytes(), &SaltString::generate(&mut OsRng))
            .unwrap()
            .to_string();
        let user = entity::user::ActiveModel {
            username: Set(String::from("admin")),
            nickname: Set(String::from("Administrator")),
            email: Set(String::from("admin@admin.com")),
            group: Set(entity::user::Group::Admin),
            hashed_password: Set(hashed_password),
            ..Default::default()
        };
        user.insert(get_db()).await.unwrap();
        info!("Admin user created successfully.");
    }
}

pub async fn init_config() {
    let total = entity::config::Entity::find()
        .count(get_db())
        .await
        .unwrap();
    if total == 0 {
        let config = entity::config::ActiveModel {
            value: Set(serde_json::to_value(cds_config::Config {
                auth: cds_config::auth::Config {
                    registration: cds_config::auth::registration::Config {
                        enabled: true,
                        captcha: false,
                        email: cds_config::auth::registration::email::Config {
                            enabled: false,
                            domains: vec![],
                        },
                    },
                },
                site: cds_config::site::Config {
                    title: String::from("CdsCTF"),
                    description: String::from(
                        "Reality is an illusion, the universe is a hologram.",
                    ),
                    color: String::from("#0C4497"),
                    favicon: String::from(""),
                },
                cluster: cds_config::cluster::Config {
                    entry: String::from("127.0.0.1"),
                    strategy: cds_config::cluster::strategy::Config {
                        parallel_limit: 0,
                        request_limit: 0,
                    },
                },
            })
            .unwrap()),
            ..Default::default()
        };
        config.insert(get_db()).await.unwrap();
        info!("Default configuration created successfully.");
    }
}
