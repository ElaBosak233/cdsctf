use jsonwebtoken::{EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub id: i64,
    pub exp: usize,
}

pub async fn generate_jwt_token(user_id: i64) -> String {
    let claims = Claims {
        id: user_id,
        exp: (chrono::Utc::now() + chrono::Duration::seconds(cds_env::get_config().jwt.expiration))
            .timestamp() as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(cds_env::get_config().jwt.secret.as_bytes()),
    )
    .unwrap()
}
