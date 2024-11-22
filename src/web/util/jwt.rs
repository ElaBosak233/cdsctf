use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};

use crate::config;

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub id: i64,
    pub exp: usize,
}

pub async fn get_secret() -> String {
    config::get_config().await.auth.jwt.secret_key
}

pub async fn generate_jwt_token(user_id: i64) -> String {
    let secret = get_secret().await;
    let claims = Claims {
        id: user_id,
        exp: (chrono::Utc::now()
            + chrono::Duration::minutes(config::get_config().await.auth.jwt.expiration))
        .timestamp() as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .unwrap()
}
