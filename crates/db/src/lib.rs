//! Database access with SeaORM: PostgreSQL connection, entities, and query
//! helpers.
//!
//! [`DB`] is the shared handle passed through the HTTP server’s application
//! state. [`init`] configures pool size and timeouts from the application
//! environment. [`get_config`] returns the cached singleton row of platform
//! settings (title, email, captcha, …).

/// Serializable query projections kept separate from SeaORM entities.
pub mod dto;

/// SeaORM entity models (internal to `cds-db`, re-exported selectively via `pub
/// use`).
pub(crate) mod entity;

/// Database queries, loaders, pagination, and mutations.
pub mod repository;

/// Defines the `traits` submodule (see sibling `*.rs` files).
pub mod traits;

use std::time::Duration;

use cds_env::Env;
pub use config::Config;
pub use dto::{
    ChallengeDetail, ChallengeSummary, ChallengeView, EmailView, GameChallengeSummary,
    GameChallengeView, GameDetail, GameNoticeView, GameSummary, GameView, IdpSummary, IdpView,
    NoteView, PlayerTeamView, PublicCaptchaConfig, PublicCaptchaSiteConfig, PublicConfig,
    PublicEmailConfig, ScoreboardEntry, ScoreboardSubmission, ScoreboardTeam, SubmissionSummary,
    SubmissionView, TeamUserView, TeamView, UserAccountView, UserIdpSummary, UserIdpView,
    UserProfile, UserSummary,
};
pub use entity::user_idp::Source as UserIdpSource;
pub use repository::{
    challenge, config, email, game, game_challenge, game_notice, idp, note, submission, team,
    team_user, user, user_idp,
};
pub use sea_orm;
use sea_orm::{ConnectOptions, ConnectionTrait, Database, DatabaseConnection};
use tracing::{info, log};
pub use traits::DbError;

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
