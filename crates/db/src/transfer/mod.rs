//! Transfer module is used to store structures with additional fields and
//! preload functions.
pub mod challenge;
pub mod game;
pub mod game_challenge;
pub mod game_notice;
pub mod game_team;
pub mod submission;
pub mod game_team_user;
pub mod user;

pub use challenge::Challenge;
pub use game::Game;
pub use game_challenge::GameChallenge;
pub use game_notice::GameNotice;
pub use game_team::GameTeam;
pub use submission::Submission;
pub use game_team_user::GameTeamUser;
pub use user::User;
