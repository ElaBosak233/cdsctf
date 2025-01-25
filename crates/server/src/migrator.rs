use argon2::{
    Argon2, PasswordHasher,
    password_hash::{SaltString, rand_core::OsRng},
};
use cds_db::{entity, get_db};
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ConnectionTrait, DbConn, EntityTrait, PaginatorTrait,
    Schema,
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
