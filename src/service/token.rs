use std::time::{SystemTime, UNIX_EPOCH};

use jsonwebtoken::{EncodingKey, Header, encode};

use crate::schema::claims::Claims;

pub fn create_token(
    account: &str,
    role: &str,
    secret: &str,
    expires_secs: u64,
) -> Result<String, jsonwebtoken::errors::Error> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let claims = Claims {
        sub: account.to_string(),
        role: role.to_string(),
        iat: now,
        exp: now + expires_secs,
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}
