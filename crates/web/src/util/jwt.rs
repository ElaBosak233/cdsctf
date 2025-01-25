use jsonwebtoken::{EncodingKey, Header, encode};
use regex::Regex;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub id: i64,
    pub exp: usize,
}

pub async fn get_jwt_config() -> cds_config::auth::Config {
    if let Some(jwt) = cds_cache::get::<cds_config::auth::Config>("jwt")
        .await
        .unwrap()
    {
        return jwt;
    }

    let mut jwt = cds_config::get_config().auth.clone();
    let re = Regex::new(r"\[([Uu][Uu][Ii][Dd])]").unwrap();
    jwt.secret = re
        .replace_all(&jwt.secret, Uuid::new_v4().simple().to_string())
        .to_string();
    let _ = cds_cache::set("jwt", jwt.clone()).await;

    jwt
}

pub async fn generate_jwt_token(user_id: i64) -> String {
    let jwt_config = get_jwt_config().await;
    let claims = Claims {
        id: user_id,
        exp: (chrono::Utc::now() + chrono::Duration::minutes(jwt_config.expiration)).timestamp()
            as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_config.secret.as_bytes()),
    )
    .unwrap()
}
