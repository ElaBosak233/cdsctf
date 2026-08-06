//! Database queries and mutations grouped by aggregate.
//!
//! The crate root re-exports these modules to preserve paths such as
//! `cds_db::user::find_by_id`; new code may also use the explicit
//! `cds_db::repository::user` path.

pub mod challenge;
pub mod config;
pub mod email;
pub mod game;
pub mod game_challenge;
pub mod game_notice;
pub mod idp;
pub mod note;
pub mod submission;
pub mod team;
pub mod team_user;
pub mod user;
pub mod user_idp;
