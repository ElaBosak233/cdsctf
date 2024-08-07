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
    E: EntityTrait,
{
    let builder = db.get_database_backend();
    let schema = Schema::new(builder);
    let stmt = builder.build(schema.create_table_from_entity(entity).if_not_exists());

    match db.execute(stmt).await {
        Err(e) => error!("Error: {}", e),
        _ => {}
    }
}

pub async fn migrate(db: &DbConn) {
    create_tables!(
        db,
        crate::model::user::Entity,
        crate::model::team::Entity,
        crate::model::user_team::Entity,
        crate::model::category::Entity,
        crate::model::challenge::Entity,
        crate::model::game::Entity,
        crate::model::submission::Entity,
        crate::model::pod::Entity,
        crate::model::game_challenge::Entity,
        crate::model::game_team::Entity
    );
}
