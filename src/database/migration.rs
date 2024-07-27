use sea_orm::{ConnectionTrait, DbConn, EntityTrait, Schema};
use tracing::error;

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
    create_table(db, crate::model::user::Entity).await;
    create_table(db, crate::model::team::Entity).await;
    create_table(db, crate::model::user_team::Entity).await;
    create_table(db, crate::model::category::Entity).await;
    create_table(db, crate::model::challenge::Entity).await;
    create_table(db, crate::model::submission::Entity).await;
    create_table(db, crate::model::game::Entity).await;
    create_table(db, crate::model::pod::Entity).await;
    create_table(db, crate::model::game_challenge::Entity).await;
    create_table(db, crate::model::game_team::Entity).await;
}
