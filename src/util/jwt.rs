use crate::config;
use jsonwebtoken::{encode, EncodingKey, Header};
use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};

static SECRET: Lazy<String> = Lazy::new(|| {
    let mut secret_key = config::get_config().auth.jwt.secret_key.clone();
    let re = Regex::new(r"\[([Uu][Uu][Ii][Dd])\]").unwrap();
    secret_key = re
        .replace_all(&secret_key, uuid::Uuid::new_v4().simple().to_string())
        .to_string();
    return secret_key;
});

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub id: i64,
    pub exp: usize,
}

#[derive(Debug, Clone)]
#[repr(u8)]
pub enum Group {
    Admin = 3,
    User = 2,
    Guest = 1,
    Banned = 0,
}

impl Group {
    pub fn from_str(s: String) -> Result<Group, &'static str> {
        match s.as_str() {
            "admin" => Ok(Group::Admin),
            "user" => Ok(Group::User),
            "guest" => Ok(Group::Guest),
            "banned" => Ok(Group::Banned),
            _ => Err("Invalid group"),
        }
    }
}

pub async fn get_secret() -> String {
    return SECRET.clone();
}

pub async fn generate_jwt_token(user_id: i64) -> String {
    let secret = get_secret().await;
    let claims = Claims {
        id: user_id,
        exp: (chrono::Utc::now()
            + chrono::Duration::minutes(crate::config::get_config().auth.jwt.expiration as i64))
        .timestamp() as usize,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .unwrap();

    return token;
}
