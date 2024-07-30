use std::fs;

use rust_embed::Embed;

#[derive(Embed)]
#[folder = "assets/"]
pub struct Assets;

pub fn get(path: &str) -> Option<Vec<u8>> {
    if let Ok(file) = fs::read(format!("assets/{}", path)) {
        return Some(file);
    }
    return Assets::get(&path).map(|e| e.data.into_owned());
}
