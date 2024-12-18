use jsonwebtoken::{EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub id: i64,
    pub exp: usize,
}

pub async fn get_secret() -> String {
    cds_config::get_config().await.auth.jwt.secret_key
}

pub async fn generate_jwt_token(user_id: i64) -> String {
    let secret = get_secret().await;
    let claims = Claims {
        id: user_id,
        exp: (chrono::Utc::now()
            + chrono::Duration::minutes(cds_config::get_config().await.auth.jwt.expiration))
        .timestamp() as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .unwrap()
}
