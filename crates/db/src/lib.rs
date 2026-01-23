pub mod challenge;
pub mod config;
pub mod email;
pub(crate) mod entity;
pub mod game;
pub mod game_challenge;
pub mod game_notice;
pub mod submission;
pub mod team;
pub mod team_user;
pub mod traits;
pub mod user;
pub mod util;

use std::time::Duration;

use cds_env::Env;
pub use challenge::{Challenge, ChallengeMini};
pub use email::Email;
pub use game::{Game, GameMini};
pub use game_challenge::{GameChallenge, GameChallengeMini};
pub use game_notice::GameNotice;
pub use sea_orm;
use sea_orm::{ConnectOptions, ConnectionTrait, Database, DatabaseConnection};
pub use submission::Submission;
pub use team::Team;
pub use team_user::TeamUser;
use tracing::{info, log};
pub use traits::DbError;
pub use user::{User, UserMini};

#[derive(Clone, Debug)]
pub struct DB {
    pub conn: DatabaseConnection,
}

pub async fn init(env: &Env) -> Result<DB, DbError> {
    let url = format!(
        "postgres://{}:{}@{}:{}/{}",
        env.db.username, env.db.password, env.db.host, env.db.port, env.db.dbname,
    );
    let mut opt = ConnectOptions::new(url);
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Debug)
        .set_schema_search_path("public");

    let db: DatabaseConnection = Database::connect(opt).await?;
    info!("Database connection established successfully.");

    Ok(DB { conn: db })
}

pub async fn get_config(conn: &impl ConnectionTrait) -> config::Model {
    config::get(conn)
        .await
        .expect("No config in db, could there be a problem with the migration?")
}
