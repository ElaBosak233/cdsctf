use serde::Deserialize;

#[derive(Deserialize)]
pub struct LinkRequest {
    pub port: u32,
}
