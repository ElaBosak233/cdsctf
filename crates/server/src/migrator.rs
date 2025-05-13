use cds_db::get_db;
use sea_orm::{
    ConnectionTrait, DbConn, EntityName, EntityTrait, Iden, PaginatorTrait, Schema, Statement,
};
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

    if let Err(e) = db.execute(stmt).await {
        error!("Error: {}", e)
    }
}

pub async fn run() -> Result<(), anyhow::Error> {
    create_tables!(
        get_db(),
        cds_db::entity::config::Entity,
        cds_db::entity::user::Entity,
        cds_db::entity::challenge::Entity,
        cds_db::entity::game::Entity,
        cds_db::entity::team::Entity,
        cds_db::entity::team_user::Entity,
        cds_db::entity::submission::Entity,
        cds_db::entity::game_challenge::Entity,
        cds_db::entity::game_notice::Entity
    );

    get_db()
        .execute(Statement::from_string(
            sea_orm::DatabaseBackend::Postgres,
            "CREATE EXTENSION IF NOT EXISTS pg_trgm;",
        ))
        .await?;

    get_db()
        .execute(Statement::from_string(
            sea_orm::DatabaseBackend::Postgres,
            format!(
                "CREATE INDEX IF NOT EXISTS idx_challenges_title ON \"{}\" USING gin(\"{}\" gin_trgm_ops);",
                cds_db::entity::challenge::Entity.table_name(),
                cds_db::entity::challenge::Column::Title.to_string()
            ).as_str(),
        ))
        .await?;

    if cds_db::entity::config::Entity::find()
        .count(get_db())
        .await?
        < 1
    {
        cds_db::entity::config::Entity::insert(cds_db::entity::config::ActiveModel {
            id: sea_orm::ActiveValue::Set(1),
            meta: sea_orm::ActiveValue::Set(cds_db::entity::config::meta::Config::default()),
            auth: sea_orm::ActiveValue::Set(cds_db::entity::config::auth::Config::default()),
            email: sea_orm::ActiveValue::Set(cds_db::entity::config::email::Config::default()),
            captcha: sea_orm::ActiveValue::Set(cds_db::entity::config::captcha::Config::default()),
        })
        .exec(get_db())
        .await?;
    }

    Ok(())
}
