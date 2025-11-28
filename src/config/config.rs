use anyhow::Context;

#[derive(Clone)]
pub struct Config {
    pub database_url: String,
    pub access_token_secret: String,
    pub refresh_token_secret: String,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        let database_url =
            std::env::var("DATABASE_URL").context("DATABASE_URL is not set")?;
        let access_token_secret =
            std::env::var("ACCESS_TOKEN_SECRET").context("ACCESS_TOKEN_SECRET is not set")?;
        let refresh_token_secret =
            std::env::var("REFRESH_TOKEN_SECRET").context("REFRESH_TOKEN_SECRET is not set")?;

        Ok(Self {
            database_url,
            access_token_secret,
            refresh_token_secret,
        })
    }
}
