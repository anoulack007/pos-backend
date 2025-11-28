use serde::{Serialize, Deserialize};
use jsonwebtoken::{encode, Header,EncodingKey};
use chrono::{Utc, Duration};

use crate::config::Config;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims{
    pub sub: String,
    pub role: String,
    pub typ: String,
    pub exp: usize,
}

#[derive(Serialize)]
pub struct TokenPair{
    pub access_token: String,
    pub refresh_token: String,
}

const ACCESS_TOKEN_TTL_SECS: i64 = 60 * 60 * 24;
const REFRESH_TOKEN_TTL_SECS: i64 = 60 * 60 * 24 * 7;

pub fn generate_token_pair(user_id: &str, role:&str,config:&Config) -> anyhow::Result<TokenPair>{
    let now = Utc::now();

    let access_claims = Claims{
        sub: user_id.to_string(),
        role: role.to_string(),
        typ:"access".to_string(),
        exp: (now + Duration::seconds(ACCESS_TOKEN_TTL_SECS)).timestamp() as usize,
    };

    let refresh_claims = Claims{
        sub: user_id.to_string(),
        role: role.to_string(),
        typ:"refresh".to_string(),
        exp: (now + Duration::seconds(REFRESH_TOKEN_TTL_SECS)).timestamp() as usize,
    };

    let access_token = encode(
        &Header::default(),
        &access_claims,
        &EncodingKey::from_secret(config.access_token_secret.as_bytes()),
    )?;
    
    let refresh_token = encode(
        &Header::default(),
        &refresh_claims,
        &EncodingKey::from_secret(config.refresh_token_secret.as_bytes()),
    )?;

    Ok(TokenPair{
        access_token,
        refresh_token,
    })
}