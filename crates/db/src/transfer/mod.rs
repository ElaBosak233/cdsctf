//! Transfer module is used to store structures with additional fields and
//! preload functions.
pub mod challenge;
pub mod game;
pub mod game_challenge;
pub mod game_notice;
pub mod submission;
pub mod team;
pub mod team_user;
pub mod user;

pub use challenge::Challenge;
pub use game::Game;
pub use game_challenge::GameChallenge;
pub use game_notice::GameNotice;
pub use submission::Submission;
pub use team::Team;
pub use team_user::GameTeamUser;
pub use user::User;
