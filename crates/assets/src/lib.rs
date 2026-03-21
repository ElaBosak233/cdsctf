//! Compile-time embedded assets with an optional on-disk override directory.
//!
//! [`get`] first tries `./data/assets/{path}` so operators can replace files
//! without rebuilding, then falls back to bytes embedded from `./embed/` via
//! [`rust_embed::Embed`].

use std::fs;

use rust_embed::Embed;

/// Embedded file tree included in the binary (see `#[folder = "./embed/"]` in
/// this crate’s `embed/`).
#[derive(Embed)]
#[folder = "./embed/"]
pub struct Embeds;

/// Returns file bytes if found on disk under `./data/assets/` or in [`Embeds`].
pub fn get(path: &str) -> Option<Vec<u8>> {
    if let Ok(file) = fs::read(format!("./data/assets/{}", path)) {
        return Some(file);
    }
    Embeds::get(path).map(|e| e.data.into_owned())
}
