use std::fs;

use rust_embed::Embed;

#[derive(Embed)]
#[folder = "./embed/"]
pub struct Embeds;

pub fn get(path: &str) -> Option<Vec<u8>> {
    if let Ok(file) = fs::read(format!("assets/{}", path)) {
        return Some(file);
    }
    Embeds::get(path).map(|e| e.data.into_owned())
}
