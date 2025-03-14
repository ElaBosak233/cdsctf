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
        entity::challenge::Entity,
        entity::game::Entity,
        entity::team::Entity,
        entity::team_user::Entity,
        entity::submission::Entity,
        entity::game_challenge::Entity,
        entity::game_notice::Entity
    );
}
