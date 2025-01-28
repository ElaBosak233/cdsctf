//! Transfer module is used to store structures with additional fields and
//! preload functions.
pub mod challenge;
pub mod game;
pub mod game_challenge;
pub mod game_team;
pub mod submission;
pub mod team;
pub mod user;
pub mod user_team;

pub use challenge::Challenge;
pub use game::Game;
pub use game_challenge::GameChallenge;
pub use game_team::GameTeam;
pub use submission::Submission;
pub use team::Team;
pub use user::User;
pub use user_team::UserTeam;
