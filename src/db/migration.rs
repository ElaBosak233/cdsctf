use sea_orm::{ConnectionTrait, DbConn, EntityTrait, Schema};
use tracing::error;

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

    if let Err(e) = db.execute(stmt).await { error!("Error: {}", e) }
}

pub async fn migrate(db: &DbConn) {
    create_tables!(
        db,
        super::entity::config::Entity,
        super::entity::user::Entity,
        super::entity::team::Entity,
        super::entity::user_team::Entity,
        super::entity::challenge::Entity,
        super::entity::game::Entity,
        super::entity::submission::Entity,
        super::entity::pod::Entity,
        super::entity::game_challenge::Entity,
        super::entity::game_team::Entity
    );
}
