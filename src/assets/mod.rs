use std::fs;

use rust_embed::Embed;

#[derive(Embed)]
#[folder = "assets/"]
pub struct Assets;

pub fn get(path: &str) -> Option<Vec<u8>> {
    let path = format!("assets/{}", path);
    if let Ok(file) = fs::read(&path) {
        return Some(file);
    }
    return Assets::get(&path).map(|e| e.data.into_owned());
}
