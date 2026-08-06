//! Serializable database projections shared by the Web and worker layers.
//!
//! These types are deliberately separate from SeaORM entities. Entity models
//! describe storage and relation graphs; DTOs describe the data a caller is
//! allowed to consume.

pub mod challenge;
pub mod config;
pub mod email;
pub mod game;
pub mod game_challenge;
pub mod game_notice;
pub mod idp;
pub mod note;
pub mod scoreboard;
pub mod submission;
pub mod team;
pub mod team_user;
pub mod user;
pub mod user_idp;

pub use challenge::{ChallengeDetail, ChallengeSummary, ChallengeView};
pub use config::{PublicCaptchaConfig, PublicCaptchaSiteConfig, PublicConfig, PublicEmailConfig};
pub use email::EmailView;
pub use game::{GameDetail, GameSummary, GameView};
pub use game_challenge::{GameChallengeSummary, GameChallengeView};
pub use game_notice::GameNoticeView;
pub use idp::{IdpSummary, IdpView};
pub use note::NoteView;
pub use scoreboard::{ScoreboardEntry, ScoreboardSubmission, ScoreboardTeam};
pub use submission::{SubmissionSummary, SubmissionView};
pub use team::TeamView;
pub use team_user::TeamUserView;
pub use user::{UserAccountView, UserProfile, UserSummary};
pub use user_idp::{UserIdpSummary, UserIdpView};
