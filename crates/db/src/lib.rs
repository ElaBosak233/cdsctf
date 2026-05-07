//! Database access with SeaORM: PostgreSQL connection, entities, and query
//! helpers.
//!
//! [`DB`] is the shared handle passed through the HTTP server’s application
//! state. [`init`] configures pool size and timeouts from the application
//! environment. [`get_config`] returns the cached singleton row of platform
//! settings (title, email, captcha, …).

/// Defines the `challenge` submodule (see sibling `*.rs` files).
pub mod challenge;

/// Defines the `config` submodule (see sibling `*.rs` files).
pub mod config;

/// Defines the `email` submodule (see sibling `*.rs` files).
pub mod email;

/// SeaORM entity models (internal to `cds-db`, re-exported selectively via `pub
/// use`).
pub(crate) mod entity;

/// Defines the `game` submodule (see sibling `*.rs` files).
pub mod game;

/// Defines the `game_challenge` submodule (see sibling `*.rs` files).
pub mod game_challenge;

/// Defines the `game_notice` submodule (see sibling `*.rs` files).
pub mod game_notice;

/// Defines the `idp` submodule (see sibling `*.rs` files).
pub mod idp;

/// Defines the `note` submodule (see sibling `*.rs` files).
pub mod note;

/// Defines the `submission` submodule (see sibling `*.rs` files).
pub mod submission;

/// Defines the `team` submodule (see sibling `*.rs` files).
pub mod team;

/// Defines the `team_user` submodule (see sibling `*.rs` files).
pub mod team_user;

/// Defines the `user_idp` submodule (see sibling `*.rs` files).
pub mod user_idp;

/// Defines the `traits` submodule (see sibling `*.rs` files).
pub mod traits;

/// Defines the `user` submodule (see sibling `*.rs` files).
pub mod user;

/// Defines the `util` submodule (see sibling `*.rs` files).
pub mod util;

use std::time::Duration;

use cds_env::Env;
pub use challenge::{Challenge, ChallengeMini};
pub use config::Config;
pub use email::Email;
pub use game::{Game, GameMini};
pub use game_challenge::{GameChallenge, GameChallengeMini};
pub use game_notice::GameNotice;
pub use idp::Idp;
pub use sea_orm;
use sea_orm::{ConnectOptions, ConnectionTrait, Database, DatabaseConnection};
pub use submission::Submission;
pub use team::Team;
pub use team_user::TeamUser;
use tracing::{info, log};
pub use traits::DbError;
pub use user::{User, UserMini};
pub use user_idp::UserIdp;

/// Shared database connection (actually a connection pool managed by SeaORM /
/// SQLx).
#[derive(Clone, Debug)]
pub struct DB {
    pub conn: DatabaseConnection,
}

/// Opens PostgreSQL using credentials in `env.db` and applies conservative pool
/// limits.
pub async fn init(env: &Env) -> Result<DB, DbError> {
    let url = format!(
        "postgres://{}:{}@{}:{}/{}",
        env.db.username, env.db.password, env.db.host, env.db.port, env.db.dbname,
    );
    let mut opt = ConnectOptions::new(url);
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(2))
        .idle_timeout(Duration::from_mins(10))
        .max_lifetime(Duration::from_mins(30))
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Debug)
        .set_schema_search_path("public");

    let db: DatabaseConnection = Database::connect(opt).await?;
    info!("Database connection established successfully.");

    Ok(DB { conn: db })
}

/// Loads the platform configuration row; panics if missing (migrations should
/// always seed one).
pub async fn get_config(conn: &impl ConnectionTrait) -> Config {
    config::get(conn)
        .await
        .expect("No config in db, could there be a problem with the migration?")
}
